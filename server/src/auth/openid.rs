use std::time::{Duration, Instant};

use jsonwebtoken::Validation;
use reqwest;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::auth::keys::KeySet;

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

pub enum ValidationError {
    UnknownKey,
    FailedToValidate,
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

    pub async fn validate(&self, jwt: JWT) -> Result<(), ValidationError> {
        let mut guard = self.cached_key_set.lock().await;

        // If we know about the key used to sign this JWT, validate it.
        if let Some(is_valid) = guard.key_set.validate_jwt(&jwt) {
            return if is_valid {
                Ok(())
            } else {
                Err(ValidationError::FailedToValidate)
            };
        }

        // Otherwise, if the cache is >5m old try and update it.
        if Instant::now().duration_since(guard.last_updated) > Duration::from_secs(5 * 60) {
            if let Ok(fresh_key_set) = OpenIDValidator::get_fresh_msa_key_set().await {
                guard.key_set = fresh_key_set
            }

            // Even if we failed to get a fresh key set above, set the updated time
            // so we don't try again for another 5m.
            guard.last_updated = Instant::now();

            Ok(())
        } else {
            Err(ValidationError::UnknownKey)
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
