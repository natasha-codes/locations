use std::time::{Duration, Instant};

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;

use crate::openid::authority::Authority;
use crate::openid::key_set::{Key, KeySet, KeySetFetcher, NetworkKeySetFetcher};

pub struct Validator<C: DeserializeOwned, F: KeySetFetcher> {
    /// The OpenID authority to use to validate.
    authority: Authority<C>,
    /// Used for fetching fresh key sets from the authority.
    fetcher: F,
    /// The minimum interval between attempted key set refreshes.
    refresh_interval: Duration,
    /// The currently-held set of keys from the authority.
    key_set: KeySet,
    /// When the key set was last updated.
    key_set_last_updated: Instant,
}

impl<C: DeserializeOwned> Validator<C, NetworkKeySetFetcher> {
    pub fn new(authority: Authority<C>) -> Self {
        Validator::new_with_config(
            authority,
            NetworkKeySetFetcher::new(),
            Duration::from_secs(5 * 60),
        )
    }
}

impl<C: DeserializeOwned, F: KeySetFetcher> Validator<C, F> {
    pub fn new_with_config(
        authority: Authority<C>,
        fetcher: F,
        refresh_interval: Duration,
    ) -> Self {
        Self {
            authority,
            fetcher,
            refresh_interval,
            key_set: KeySet::empty(),
            key_set_last_updated: Instant::now() - refresh_interval,
        }
    }

    /// Returns a boolean indicating if the given JWT validated, using the authority
    /// this validator was initialized with. May perform a keyset cache refresh if
    /// the JWT was signed with a key we don't have locally.
    pub async fn validate(&mut self, jwt: &str) -> bool {
        if let Ok(header) = decode_header(jwt) {
            if let Some(thumbprint) = header.kid {
                if let Some(key) = self.get_key(&thumbprint).await {
                    let decoding_key =
                        DecodingKey::from_rsa_components(&key.modulus, &key.exponent);

                    let mut validation = Validation::new(Algorithm::from(header.alg));
                    validation.set_audience(&[self.authority.aud()]);

                    let decode_result = decode::<C>(jwt, &decoding_key, &validation);
                    return decode_result.is_ok();
                }
            }
        }

        false
    }

    async fn get_key(&mut self, thumbprint: &str) -> Option<Key> {
        match self.key_set.key_with_thumbprint(&thumbprint) {
            Some(key) => Some(key),
            None => {
                if self.try_refresh_key_set().await {
                    self.key_set.key_with_thumbprint(&thumbprint)
                } else {
                    None
                }
            }
        }
    }

    /// Try and refresh the cached key set. Returns a boolean representing if the
    /// cache was refreshed or not. The cache could fail to refresh if a refresh
    /// was attempted recently, or if there was an error performing a refresh.
    async fn try_refresh_key_set(&mut self) -> bool {
        if Instant::now().duration_since(self.key_set_last_updated) >= self.refresh_interval {
            let maybe_key_set = self.fetcher.fetch(&self.authority).await;

            // Regardless of if we succeeded in getting a fresh key set above,
            // set the updated time so we don't try again for another 5m.
            self.key_set_last_updated = Instant::now();

            match maybe_key_set {
                Ok(fresh_key_set) => {
                    self.key_set = fresh_key_set;
                    true
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    /// Tests that a JWT encoded with a local key successfully decodes with a
    /// fresh validator. Since the validator starts with an empty keyset, also
    /// tests that the validator will refresh its keyset for an unrecognized
    /// signing key.
    async fn test_fresh_validator_refreshes_cache_and_validates() {
        let mut validator = Validator::new_with_config(
            utils::generate_authority(),
            utils::TestKeySetFetcher::new(),
            Duration::from_secs(0),
        );

        let token = utils::generate_jwt();

        assert!(validator.validate(&token).await);
    }

    mod utils {
        use std::time::{SystemTime, UNIX_EPOCH};

        use super::*;

        use async_trait::async_trait;
        use jsonwebtoken::{encode, EncodingKey, Header};
        use serde::{Deserialize, Serialize};

        use crate::openid::key_set::Key;

        pub fn generate_authority() -> Authority<TestClaims> {
            Authority::new(TEST_AUTHORITY_DOMAIN, TEST_AUTHORITY_AUD)
        }

        pub fn generate_jwt() -> String {
            let mut header = Header::new(Algorithm::RS256);
            header.kid = Some(String::from("mytestkey"));

            let claims = TestClaims {
                aud: String::from(TEST_AUTHORITY_AUD),
                exp: (SystemTime::now() + Duration::from_secs(10))
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards!")
                    .as_secs(),
                foo: String::from("foo_val"),
                bar: String::from("bar_val"),
            };

            let encoding_key = EncodingKey::from_rsa_pem(TEST_RSA_PRIV_KEY.as_bytes())
                .expect("Failed to load encoding key");

            encode(&header, &claims, &encoding_key).expect("Failed to generate token")
        }

        pub const TEST_AUTHORITY_DOMAIN: &'static str = "https://example.com";
        pub const TEST_AUTHORITY_AUD: &'static str = "my::test::aud";

        #[derive(Serialize, Deserialize)]
        pub struct TestClaims {
            aud: String,
            exp: u64,
            foo: String,
            bar: String,
        }

        pub struct TestKeySetFetcher {}

        impl TestKeySetFetcher {
            pub fn new() -> Self {
                Self {}
            }
        }

        #[async_trait(?Send)]
        impl KeySetFetcher for TestKeySetFetcher {
            type Error = ();

            async fn fetch<Claims: DeserializeOwned>(
                &self,
                _authority: &Authority<Claims>,
            ) -> Result<KeySet, Self::Error> {
                let key = Key {
                    key_type: String::from("RSA"),
                    thumbprint: String::from("mytestkey"),
                    modulus: String::from(TEST_RSA_PUB_MODULUS),
                    exponent: String::from(TEST_RSA_PUB_EXPONENT),
                };

                Ok(KeySet::with_keys(vec![key]))
            }
        }

        /// These values represent a public/private RSA key pair we can use to encode/decode
        /// test JWTS. The private key is in PEM format, and the public key is represented by
        /// its modulus and exponent values. Together, these are sufficient to encode/decode
        /// a test JWT with our JWT library.
        ///
        /// To generate new values (if we ever needed to):
        /// - Generate an RSA PEM private key: `openssl genrsa -out test_key.key`. Copy the whole contents into this file.
        /// - Generate the corresponding public key: `openssl rsa -in test_key.key -outform PEM -pubout -out public.pem`
        /// - Take that public key and plug it into [this great website](https://superdry.apphb.com/tools/online-rsa-key-converter)
        ///   to get the modulus/exponent.
        /// - Related to [this issue](https://github.com/Keats/jsonwebtoken/issues/153), need to take the base64 modulus from
        ///   above and re-encode it with the URL-safe base64 charset. Paste the b64 modulus into
        ///   [this website](http://www.base64url.com/), and copy the URL-safe b64 modulus into this file.
        const TEST_RSA_PUB_MODULUS: &'static str =
            "qsxfYbJkogSb7JOBZtCgwEztVk1DVu6eniGzSAu3oedBVkAsjxIvMoXQVZp-g72Z9Fzvi43hMjk3o9RPUAju-xSo1gYOBEHj7B6QV799YecOZyAVYXEG5ugJSNxDeevRlcOny2vXqcLjDZaEIT7GZMYzrKxY2JdTsYqYfy2ZV5vm-7K79hePKvs3rhvFi-X51mgM3EzE2uJ8z8g4z3PvNyCIyZLztJuEqI_R_tkXDrtQqyv8Tpwxb22iDjNVw59iH_H7sf0rgQwyh8DtGreKlFXBuqgqWNphm8qpQ1F1StZxlckxNDJI_kRriBVb45J0iKS3FDIJFGBuZqd10XAs7Q";
        const TEST_RSA_PUB_EXPONENT: &'static str = "AQAB";
        const TEST_RSA_PRIV_KEY: &'static str = "-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEAqsxfYbJkogSb7JOBZtCgwEztVk1DVu6eniGzSAu3oedBVkAs
jxIvMoXQVZp+g72Z9Fzvi43hMjk3o9RPUAju+xSo1gYOBEHj7B6QV799YecOZyAV
YXEG5ugJSNxDeevRlcOny2vXqcLjDZaEIT7GZMYzrKxY2JdTsYqYfy2ZV5vm+7K7
9hePKvs3rhvFi+X51mgM3EzE2uJ8z8g4z3PvNyCIyZLztJuEqI/R/tkXDrtQqyv8
Tpwxb22iDjNVw59iH/H7sf0rgQwyh8DtGreKlFXBuqgqWNphm8qpQ1F1StZxlckx
NDJI/kRriBVb45J0iKS3FDIJFGBuZqd10XAs7QIDAQABAoIBAESb8Dy4goAqxc/U
uQhqCgj1XelrA//pvsHa27+3JT7ePHq/MKcQMPFkm3mno+abxTpKEtfGWI3qOhWP
dYS68fTeKaw/pmXDaiExbd4xx9YKENkQJEaONq2OzBv+jwRs3DYC8GZgnbNN3BNb
QRxxsRROIffdC5uFvlL1T6jW0mBEUhgGqeyKJIpKop6ULH7bbf6k2ocjqmHaiv/T
2KO/2eQpSSe7plRf5sw7ZzNmIPN52PQMQZ5U/hIKu8vHzsMiEPYbHb3vMVECbCzK
TrM93UqmwbS38mj9e9tNGsT7i2A0HwHmTFGTBvuzwc5uwRDEHbKNqOXwSsPwopY2
XcA3V2kCgYEA0dhQ5dWreQfTXxfdEGfCowawOv73FXYwKGtNLGMlaNumAF72tvff
8o9pIKnYq3nCL5xdYYyxOOwiIrWeY35L2Plzr1XWWY+pu6M7KuI3tY6L5RCa9SlY
bUg121wMdiCXUf5AQCIGweurzssugTC+LO1Z5OsJtpfJGvhY1IB2JO8CgYEA0F13
o32ZXxWRtu3kNBc0ntkGjiAtWote3jhYCWmZP9FTKAoZaCpXiTCGtxm1tx+iA+zR
n01+U+kZiTOOSist9d2UkXg06HZWOEa7hOdzv9uQwgFq3EwKH847us3d5aG0MPqI
yLJ9cGh5Kzwgy85CbmZXIxvwwEpXlpQCKL/rY+MCgYAlWo47+2cEqmHz4XmWfAHn
pz8joVM2XM2BxGf+aL+2BLNuCXl9ZG5W7TRXfiR1kb0BYKI2xSae7Vm+N/oz172E
qBLxuSPo2WvrXMkhfxdPmEpKwkPzNCLrgRklLSOCu5Z0IAAr6mKtjlgM/ZoPoS+Z
K77+wt/9wtMnm+GkIiHlvQKBgQCbKw/eY7lksuZZyRzpoeJg7RPRoarExAd5C5Kc
J9SlTthTd/nltaqMkuOv1Wian+cCb0jIDmimSm/m2cr79t36O/HYxyD3gLDCpgId
flOXrAZIQ8/1kHb1qpqWiZrW2C4dT2WvuCRsIpEhnlx6Cfc0yefYUIVgXbyOeViG
Si4cqwKBgCxca4EXjxXXi0XUvydwW2fgUQWPdyeCG6XA1lzdxnjQ9vvNCnrveOnT
06SGQX/J3voTd+/PLpk2EmEBlSBoM8+bfJC1XvbhhwalLleSv/5nY+uGuxii+qYj
XhDogh0OS9EWWrpofA1JleaCegmeXpJpknjJP+XHM7d4fNbhAlvZ
-----END RSA PRIVATE KEY-----";
    }
}
