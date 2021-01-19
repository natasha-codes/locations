use std::time::{Duration, Instant};

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use tokio::sync::{Mutex, MutexGuard};

use super::{
    authority::{Authority, Claims, MSAClaims},
    key_set::{Key, KeySet, KeySetFetcher, NetworkKeySetFetcher},
};

pub type MSAJwtValidator = JwtValidator<MSAClaims, NetworkKeySetFetcher>;

impl MSAJwtValidator {
    pub fn new_msa() -> MSAJwtValidator {
        JwtValidator::new(Authority::MSA)
    }
}

pub struct JwtValidator<C: Claims, F: KeySetFetcher> {
    /// The OpenID authority to use to validate.
    authority: Authority<C>,
    /// Used for fetching fresh key sets from the authority.
    fetcher: F,
    /// The minimum interval between attempted key set refreshes.
    refresh_interval: Duration,
    /// A cached key set from the authority.
    key_set_cache: Mutex<KeySetCache>,
}

struct KeySetCache {
    /// The currently-cached keys.
    keys: KeySet,
    /// When the cache was last updated.
    last_updated: Instant,
}

impl<C: Claims> JwtValidator<C, NetworkKeySetFetcher> {
    pub fn new(authority: Authority<C>) -> Self {
        JwtValidator::new_with_config(
            authority,
            NetworkKeySetFetcher::new(),
            Duration::from_secs(5 * 60),
        )
    }
}

impl<C: Claims, F: KeySetFetcher> JwtValidator<C, F> {
    pub fn new_with_config(
        authority: Authority<C>,
        fetcher: F,
        refresh_interval: Duration,
    ) -> Self {
        Self {
            authority,
            fetcher,
            refresh_interval,
            key_set_cache: Mutex::new(KeySetCache {
                keys: KeySet::empty(),
                last_updated: Instant::now() - refresh_interval,
            }),
        }
    }

    /// Returns a boolean indicating if the given JWT validated, using the authority
    /// this validator was initialized with. May perform a keyset cache refresh if
    /// the JWT was signed with a key we don't have locally.
    pub async fn validate(&self, jwt: &str) -> Option<C> {
        if let Ok(header) = decode_header(jwt) {
            if let Some(thumbprint) = header.kid {
                if let Some(key) = self.get_key(&thumbprint).await {
                    let decoding_key =
                        DecodingKey::from_rsa_components(&key.modulus, &key.exponent);

                    let mut validation = Validation::new(Algorithm::from(header.alg));
                    validation.set_audience(&[self.authority.aud()]);

                    if let Ok(token_data) = decode::<C>(jwt, &decoding_key, &validation) {
                        return Some(token_data.claims);
                    }
                }
            }
        }

        None
    }

    async fn get_key(&self, thumbprint: &str) -> Option<Key> {
        let mut cache = self.key_set_cache.lock().await;

        match cache.keys.key_with_thumbprint(&thumbprint) {
            Some(key) => Some(key),
            None => {
                if self.try_refresh_key_set(&mut cache).await {
                    cache.keys.key_with_thumbprint(&thumbprint)
                } else {
                    None
                }
            }
        }
    }

    /// Try and refresh the cached key set. Returns a boolean representing if the
    /// cache was refreshed or not. The cache could fail to refresh if a refresh
    /// was attempted recently, or if there was an error performing a refresh.
    async fn try_refresh_key_set<'a>(&self, cache_guard: &mut MutexGuard<'a, KeySetCache>) -> bool {
        if Instant::now().duration_since(cache_guard.last_updated) >= self.refresh_interval {
            let maybe_key_set = self.fetcher.fetch(&self.authority).await;

            // Regardless of if we succeeded in getting a fresh key set above,
            // set the updated time so we don't try again for another 5m.
            cache_guard.last_updated = Instant::now();

            match maybe_key_set {
                Ok(fresh_key_set) => {
                    cache_guard.keys = fresh_key_set;
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
        let validator = JwtValidator::new_with_config(
            utils::generate_authority("my::aud"),
            utils::TestKeySetFetcher::new(utils::generate_keyset("keyid")),
            Duration::from_secs(0),
        );

        let token = utils::generate_jwt("keyid", "my::aud", 3);

        assert_eq!(
            validator
                .validate(&token)
                .await
                .expect("Token failed to validate")
                .user_id(),
            String::from("user_id")
        );
    }

    #[tokio::test]
    /// Tests that if a cache refresh returns a keyset missing the key
    /// then the validation fails. Also checks that an immediate subsequent
    /// validation fails, and that a validation after a sufficient delay
    /// succeeds.
    async fn test_validation_fails_if_refresh_doesnt_return_key() {
        let validator = JwtValidator::new_with_config(
            utils::generate_authority("my::aud"),
            utils::TestKeySetFetcher::new_with_multiple(
                KeySet::empty(),
                utils::generate_keyset("keyid"),
            ),
            Duration::from_millis(1),
        );

        let token = utils::generate_jwt("keyid", "my::aud", 3);

        // First validation should fail because fetcher will return an empty keyset.
        assert!(validator.validate(&token).await.is_none());

        // Second immediate validation should fail because fetcher will decline to
        // refresh the keyset.
        assert!(validator.validate(&token).await.is_none());

        tokio::time::sleep(Duration::from_millis(1)).await;

        // Third validation should succeed because fetcher will provide the keyset
        // with the right key.
        assert_eq!(
            validator
                .validate(&token)
                .await
                .expect("Token failed to validate")
                .user_id(),
            String::from("user_id")
        );
    }

    #[tokio::test]
    /// Tests that a JWT with an aud not matching ours is rejected.
    async fn test_validation_rejects_mismatched_aud() {
        let validator = JwtValidator::new_with_config(
            utils::generate_authority("my::aud"),
            utils::TestKeySetFetcher::new(utils::generate_keyset("keyid")),
            Duration::from_secs(0),
        );

        let token = utils::generate_jwt("keyid", "not::my::aud", 3);

        assert!(validator.validate(&token).await.is_none());
    }

    #[tokio::test]
    /// Tests that an expired JWT (via `exp`) is rejected.
    async fn test_validation_rejects_expired() {
        let validator = JwtValidator::new_with_config(
            utils::generate_authority("my::aud"),
            utils::TestKeySetFetcher::new(utils::generate_keyset("keyid")),
            Duration::from_secs(0),
        );

        let token = utils::generate_jwt("keyid", "not::my::aud", 1);

        tokio::time::sleep(Duration::from_millis(2)).await;

        assert!(validator.validate(&token).await.is_none());
    }

    #[tokio::test]
    /// Tests that a JWT with an invalid (manually broken) signature is rejected.
    async fn test_validation_rejects_invalid_signature() {
        let validator = JwtValidator::new_with_config(
            utils::generate_authority("my::aud"),
            utils::TestKeySetFetcher::new(utils::generate_keyset("keyid")),
            Duration::from_secs(0),
        );

        let mut token = utils::generate_jwt("keyid", "my::aud", 3);

        assert_eq!(
            validator
                .validate(&token)
                .await
                .expect("Token failed to validate")
                .user_id(),
            String::from("user_id")
        );

        // Manually break the signature by swapping the last char
        let last_char = token.pop().expect("Token was empty");
        token.push(match last_char {
            'a' => 'b',
            _ => 'a',
        });

        assert!(validator.validate(&token).await.is_none());
    }

    #[tokio::test]
    /// Tests that a malformed JWT is rejected.
    async fn test_validation_rejects_malformed_jwt() {
        let validator = JwtValidator::new_with_config(
            utils::generate_authority("my::aud"),
            utils::TestKeySetFetcher::new(utils::generate_keyset("keyid")),
            Duration::from_secs(0),
        );

        assert!(validator
            .validate("not_a_jwt_not_even_close")
            .await
            .is_none());
    }

    mod utils {
        use std::time::{SystemTime, UNIX_EPOCH};

        use super::*;

        use jsonwebtoken::{encode, EncodingKey, Header};
        use rocket::async_trait;
        use serde::{Deserialize, Serialize};

        pub fn generate_authority(aud: &'static str) -> Authority<TestClaims> {
            Authority::new("https://example.com", aud)
        }

        pub fn generate_keyset(thumbprint: &str) -> KeySet {
            KeySet::with_keys(vec![Key {
                key_type: String::from("RSA"),
                thumbprint: String::from(thumbprint),
                modulus: String::from(TEST_RSA_PUB_MODULUS),
                exponent: String::from(TEST_RSA_PUB_EXPONENT),
            }])
        }

        pub fn generate_jwt(thumbprint: &str, aud: &'static str, exp_ms: u64) -> String {
            let mut header = Header::new(Algorithm::RS256);
            header.kid = Some(String::from(thumbprint));

            let claims = TestClaims {
                aud: String::from(aud),
                exp: (SystemTime::now() + Duration::from_millis(exp_ms))
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards!")
                    .as_secs(),
                oid: String::from("user_id"),
            };

            let encoding_key = EncodingKey::from_rsa_pem(TEST_RSA_PRIV_KEY.as_bytes())
                .expect("Failed to load encoding key");

            encode(&header, &claims, &encoding_key).expect("Failed to generate token")
        }

        #[derive(Serialize, Deserialize)]
        pub struct TestClaims {
            aud: String,
            exp: u64,
            oid: String,
        }

        impl Claims for TestClaims {
            fn user_id(self) -> String {
                self.oid
            }
        }

        /// This test keyset fetcher will return `first` the first time its
        /// `fetch` is called, and `rest` for all future calls. Allows validation
        /// of keyset refresh logic through combinations of keysets prefilled
        /// by a given test.
        pub struct TestKeySetFetcher {
            first: KeySet,
            rest: KeySet,
            fetches: Mutex<usize>,
        }

        impl TestKeySetFetcher {
            pub fn new(first: KeySet) -> Self {
                Self::new_with_multiple(first.clone(), first)
            }

            pub fn new_with_multiple(first: KeySet, rest: KeySet) -> Self {
                Self {
                    first,
                    rest,
                    fetches: Mutex::new(0),
                }
            }
        }

        #[async_trait]
        impl KeySetFetcher for TestKeySetFetcher {
            type Error = ();

            async fn fetch<C: Claims>(
                &self,
                _authority: &Authority<C>,
            ) -> Result<KeySet, Self::Error> {
                let mut fetches = self.fetches.lock().await;
                *fetches += 1;

                if *fetches == 1 {
                    Ok(self.first.clone())
                } else {
                    Ok(self.rest.clone())
                }
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
