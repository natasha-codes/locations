use std::time::{Duration, Instant};

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest;
use tokio::sync::{Mutex, MutexGuard};

use crate::openid::authority::{Authority, Claims};
use crate::openid::key_set::{get_key_set, Key, KeySet};

pub struct Validator<C: Claims> {
    authority: Authority<C>,
    cached_key_set: Mutex<CachedKeySet>,
}

impl<C: Claims> Validator<C> {
    pub async fn new(authority: Authority<C>) -> Result<Self, reqwest::Error> {
        let fresh_key_set = get_key_set(authority).await?;

        Ok(Self {
            authority,
            cached_key_set: Mutex::new(CachedKeySet {
                key_set: fresh_key_set,
                last_updated: Instant::now(),
            }),
        })
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
            let maybe_key_set = get_key_set(self.authority).await;

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
