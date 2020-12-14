use std::time::{Duration, Instant};

use reqwest;
use tokio::sync::{Mutex, MutexGuard};

use crate::openid::authority::Authority;
use crate::openid::key_set::{get_key_set, KeySet};

pub struct Validator {
    authority: Authority,
    cached_key_set: Mutex<CachedKeySet>,
}

impl Validator {
    pub async fn new(authority: Authority) -> Result<Self, reqwest::Error> {
        let fresh_key_set = get_key_set(authority).await?;

        Ok(Self {
            authority,
            cached_key_set: Mutex::new(CachedKeySet {
                key_set: fresh_key_set,
                last_updated: Instant::now(),
            }),
        })
    }

    pub async fn validate(&self, jwt: JWT) -> bool {
        let mut guard = self.cached_key_set.lock().await;

        if let Some(signing_key) = guard.key_set.key_with_thumbprint(&jwt.id) {
            true
        } else {
            self.try_refresh_msa_key_set(&mut guard).await
        }
    }

    /// Try and refresh the MSA key set. Returns a boolean representing if the
    /// cache was refreshed or not. The cache could fail to refresh if a refresh
    /// was attempted recently, or if there was an error performing a refresh.
    async fn try_refresh_msa_key_set<'a>(&self, guard: &mut MutexGuard<'a, CachedKeySet>) -> bool {
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

pub struct JWT {
    pub id: String,
    pub signature: String,
}
