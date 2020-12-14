use std::time::{Duration, Instant};

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use tokio::sync::{Mutex, MutexGuard};

use crate::openid::authority::Authority;
use crate::openid::key_set::{Key, KeySet, KeySetFetcher, NetworkKeySetFetcher};

pub struct Validator<C: DeserializeOwned, F: KeySetFetcher> {
    authority: Authority<C>,
    cached_key_set: Mutex<CachedKeySet>,
    key_set_fetcher: F,
}

impl<C: DeserializeOwned> Validator<C, NetworkKeySetFetcher> {
    pub fn new(authority: Authority<C>) -> Self {
        Validator::new_with_config(authority, NetworkKeySetFetcher::new())
    }
}

impl<C: DeserializeOwned, F: KeySetFetcher> Validator<C, F> {
    pub fn new_with_config(authority: Authority<C>, fetcher: F) -> Self {
        Self {
            authority,
            cached_key_set: Mutex::new(CachedKeySet {
                key_set: KeySet::empty(),
                last_updated: Instant::now(),
            }),
            key_set_fetcher: fetcher,
        }
    }

    pub async fn validate(&self, jwt: &str) -> bool {
        if let Ok(header) = decode_header(jwt) {
            if let Some(thumbprint) = header.kid {
                if let Some(key) = self.get_key(&thumbprint).await {
                    let decoding_key =
                        DecodingKey::from_rsa_components(&key.modulus, &key.exponent);

                    let mut validation = Validation::new(Algorithm::from(header.alg));
                    validation.set_audience(&[self.authority.aud()]);

                    return decode::<C>(jwt, &decoding_key, &validation).is_ok();
                }
            }
        }

        false
    }

    async fn get_key(&self, thumbprint: &str) -> Option<Key> {
        let mut guard = self.cached_key_set.lock().await;

        match guard.key_set.key_with_thumbprint(&thumbprint) {
            Some(key) => Some(key),
            None => {
                if self.try_refresh_key_set(&mut guard).await {
                    guard.key_set.key_with_thumbprint(&thumbprint)
                } else {
                    None
                }
            }
        }
    }

    /// Try and refresh the cached key set. Returns a boolean representing if the
    /// cache was refreshed or not. The cache could fail to refresh if a refresh
    /// was attempted recently, or if there was an error performing a refresh.
    async fn try_refresh_key_set<'a>(&self, guard: &mut MutexGuard<'a, CachedKeySet>) -> bool {
        if Instant::now().duration_since(guard.last_updated) > Duration::from_secs(5 * 60) {
            let maybe_key_set = self.key_set_fetcher.fetch(&self.authority).await;

            // Regardless of if we succeeded in getting a fresh key set above,
            // set the updated time so we don't try again for another 5m.
            guard.last_updated = Instant::now();

            match maybe_key_set {
                Ok(fresh_key_set) => {
                    guard.key_set = fresh_key_set;
                    true
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }
}

struct CachedKeySet {
    key_set: KeySet,
    last_updated: Instant,
}

#[cfg(test)]
mod test {

    use super::*;

    mod utils {
        use super::*;

        use async_trait::async_trait;
        use jsonwebtoken::{encode, EncodingKey, Header};
        use serde::{Deserialize, Serialize};

        pub fn generate_jwt() -> String {
            encode(
                &Header::new(Algorithm::RS256),
                &TestClaims {
                    foo: String::from("foo_val"),
                    bar: String::from("bar_val"),
                },
                &EncodingKey::from_rsa_pem(TEST_RSA_PRIV_KEY.as_bytes())
                    .expect("Failed to load encoding key"),
            )
            .expect("Failed to generate token")
        }

        struct TestKeySetFetcher {}

        #[async_trait(?Send)]
        impl KeySetFetcher for TestKeySetFetcher {
            type Error = ();

            async fn fetch<Claims: DeserializeOwned>(
                &self,
                _authority: &Authority<Claims>,
            ) -> Result<KeySet, Self::Error> {
                Ok(KeySet::empty())
            }
        }

        #[derive(Serialize, Deserialize)]
        struct TestClaims {
            foo: String,
            bar: String,
        }

        const TEST_RSA_PRIV_KEY: &'static str = "-----BEGIN RSA PRIVATE KEY-----
MIIEpQIBAAKCAQEA4HtN0PMWbn6Zr5ikixpd0iKEVutzvlm15YC/OHAfvA/iijw0
hD21hV7cYlGCbtEoBXU1l1T5/ZJ3SuqmoKpBgzWAuNl7vGTLJc+Ar4erqV7Yois+
4lbBCPMmJh5SsqO//FYl1099S/7gry+OQee/gsWeW9Mpw/MpGJ5oQ9Z+Ynv3hYFi
cJBnoufppZdnqfm8xSpoyvQe4WvZjQkd5PDAU+OKRr90QVsRgG8bocnBCAHKLBVV
xFuD9SC3LmUcLoL6Qc9uAO0/e66WXMgX481osmHvKQBdpg2wYP0TD/177GW25lcE
otM/n4wrdtltDUkZ2oqn58akYv82uv6hRZ6z9wIDAQABAoIBAQDNLUaBzj3ZfpOA
IPd8QPwx/eSSAaEIAb006Mlej3UiEi7QhJjHqhOItJygrLmYCkoXOvtht4TLVRz9
952XSiaZA8UEr5veJQ5dH90SEuI+63b8OqS+gebsBDoBK0QRDYSD4kWyF3CBjpPU
65WN/YFYyMGmUkphVJZibx8DqkBYSBo620wvG2gYjLceVTOY1j8GKxHvpjDLKHhU
ukUc3apuPXNnUV0cerqCHLk1C3x2+A8Svqfen+Tz0oB/IJ9Lg5uyQi/dIbLiM2pU
tMdfaJddeO5msvgRYlTorH+N/kYwGMs0lRPv+KWexXwcCZ3ChBgh57Gal/tuqMOn
L8VRFoKRAoGBAPPWd9W5hhWi+8OAy+/l8Kj6uUkLEkXG8tWxtCBYsGHiJEVFWZW3
bD6V9qwh7iwbo5wymCMp/IoqaLpzvb0JTrQPJ53BtX1wfWMstupdYXEgGTh949V1
GCI5r3UMtH5Eh0KuVRYtVjAYlWvq+GJnfoG8A9dCdZIRXKup3mTXxqFPAoGBAOut
ribZZOsB7pHAYM3/FPIuNQj8EYYVvq7hrzfEpOPI5J59VAU5pKuwKKpMWimuUvcV
cdGC8HAV2C4GFZxEeJwtVuptJ1I6AVYYUDTPm/zPi+jmJIq8o8N8OGFR1SATHjVA
oo188ZOn7TW1TRYZwmT63PmsF6Dey4XH0BAvEojZAoGBALYJ1Fcj9V1r2yd+nUIR
WVTeMbu9Xzvmpl4xF7faXnwFF2z7tEDYuiATVx/1CNm3HLM89mWyL856kMs6I1ng
e/hjJAFbn4HxnDqRJFHduyR4gTuyiIhQrd7HUB1DifCGerCmc/FlkWXAxLTXq+3T
NBfo5Lks7ZdKDPQ/kj+Y87pzAoGAGToLBR+J/NnFFpbYBdTDAjVN+fs5SPf05DVG
ExsaZ0NurURPBQwpgzMk9y2bDREa0lXaTAnPAMBl1m9SStrNajI0Nn2ekt+gmv2Z
QD3kvYfduv0/dhZBFUCrrEcdIATL2/liLPDtztdPvcr9SFtTgomTs6nnEZIniNdd
fw361ukCgYEA4LMrapbXdsf4MIlIYfFPK0agK4NjMuL9b7pJm/a8cARIV10wza5p
xWm+bERSPS2bVCE1rSIpxo+rVLgoeoB0gY8s/GtRPEe0jpF4SIEBShrA4CkfR/8Q
PJjZ80v2QyRgj06DGInc7cIG1cMMc7WIYCOgYuh4geDyxLVE3K1iksw=
-----END RSA PRIVATE KEY-----
";

        const TEST_RSA_PUB_MODULUS: &'static str = "AOB7TdDzFm5+ma+YpIsaXdIihFbrc75ZteWAvzhwH7wP4oo8NIQ9tYVe3GJRgm7RKAV1NZdU+f2Sd0rqpqCqQYM1gLjZe7xkyyXPgK+Hq6le2KIrPuJWwQjzJiYeUrKjv/xWJddPfUv+4K8vjkHnv4LFnlvTKcPzKRieaEPWfmJ794WBYnCQZ6Ln6aWXZ6n5vMUqaMr0HuFr2Y0JHeTwwFPjika/dEFbEYBvG6HJwQgByiwVVcRbg/Ugty5lHC6C+kHPbgDtP3uullzIF+PNaLJh7ykAXaYNsGD9Ew/9e+xltuZXBKLTP5+MK3bZbQ1JGdqKp+fGpGL/Nrr+oUWes/c=";
        const TEST_RSA_PUB_EXPONENT: &'static str = "AQAB";
    }
}
