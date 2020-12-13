use std::time::{Duration, Instant};

use reqwest;
use serde::Deserialize;
use tokio::sync::{Mutex, MutexGuard};

use crate::auth::keys::{JwtValidationResult, KeySet};

pub struct OpenIDValidator {
    cached_key_set: Mutex<CachedKeySet>,
}

struct CachedKeySet {
    key_set: KeySet,
    last_updated: Instant,
}

pub struct JWT {
    pub id: String,
    pub signature: String,
}

impl OpenIDValidator {
    pub async fn new() -> Result<Self, reqwest::Error> {
        let fresh_key_set = OpenIDValidator::get_fresh_msa_key_set().await?;

        Ok(Self {
            cached_key_set: Mutex::new(CachedKeySet {
                key_set: fresh_key_set,
                last_updated: Instant::now(),
            }),
        })
    }

    pub async fn validate_jwt(&self, jwt: JWT) -> bool {
        let mut guard = self.cached_key_set.lock().await;

        match guard.key_set.validate_jwt(&jwt) {
            // If we know about the key used to sign this JWT, validate it.
            JwtValidationResult::Success => true,
            JwtValidationResult::Failure => false,

            // Otherwise, try and refresh the cache and re-validate.
            JwtValidationResult::UnknownKey => {
                self.try_refresh_msa_key_set(&mut guard).await
                    && guard.key_set.validate_jwt(&jwt) == JwtValidationResult::Success
            }
        }
    }

    /// Try and refresh the MSA key set. Returns a boolean representing if the
    /// cache was refreshed or not. The cache could fail to refresh if a refresh
    /// was attempted recently, or if there was an error performing a refresh.
    async fn try_refresh_msa_key_set<'a>(&self, guard: &mut MutexGuard<'a, CachedKeySet>) -> bool {
        if Instant::now().duration_since(guard.last_updated) > Duration::from_secs(5 * 60) {
            let maybe_key_set = OpenIDValidator::get_fresh_msa_key_set().await;

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

    async fn get_fresh_msa_key_set() -> Result<KeySet, reqwest::Error> {
        // See: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc#fetch-the-openid-connect-metadata-document

        // Fill in "consumers" below because our MSA app is only accessible by personal MSAs.
        let msa_authority = "https://login.microsoftonline.com/consumers/v2.0";
        let discovery_suffix = ".well-known/openid-configuration";
        let openid_metadata_url = format!("{}/{}", msa_authority, discovery_suffix);

        let keys_uri = reqwest::get(&openid_metadata_url)
            .await?
            .json::<OpenIDMetadata>()
            .await?
            .key_roster_uri;

        reqwest::get(&keys_uri).await?.json::<KeySet>().await
    }
}

#[derive(Deserialize)]
struct OpenIDMetadata {
    #[serde(rename(deserialize = "jwks_uri"))]
    key_roster_uri: String,
}
