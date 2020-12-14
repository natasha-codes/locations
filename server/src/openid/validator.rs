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
