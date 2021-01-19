use rocket::async_trait;
use serde::Deserialize;
use tokio_compat_02::FutureExt;

use super::authority::{Authority, Claims};

#[async_trait]
pub trait KeySetFetcher {
    type Error;

    async fn fetch<C: Claims>(&self, authority: &Authority<C>) -> Result<KeySet, Self::Error>;
}

pub struct NetworkKeySetFetcher {}

impl NetworkKeySetFetcher {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl KeySetFetcher for NetworkKeySetFetcher {
    type Error = reqwest::Error;

    async fn fetch<C: Claims>(&self, authority: &Authority<C>) -> Result<KeySet, Self::Error> {
        // Need the `.compat()` wrappers around futures from `reqwest`, since
        // it uses Tokio 0.2 and we will be running on Tokio 0.3.

        #[derive(Deserialize)]
        struct Metadata {
            #[serde(rename(deserialize = "jwks_uri"))]
            key_roster_uri: String,
        }

        let keys_uri = reqwest::get(&authority.metadata_path())
            .compat() // shim
            .await?
            .json::<Metadata>()
            .compat() // shim
            .await?
            .key_roster_uri;

        reqwest::get(&keys_uri)
            .compat() // shim
            .await?
            .json::<KeySet>()
            .compat() // shim
            .await
    }
}

#[derive(Clone, Deserialize)]
pub struct KeySet {
    keys: Vec<Key>,
}

impl KeySet {
    pub fn empty() -> Self {
        Self { keys: vec![] }
    }

    #[cfg(test)]
    pub fn with_keys(keys: Vec<Key>) -> Self {
        Self { keys }
    }

    pub fn key_with_thumbprint(&self, thumbprint: &str) -> Option<Key> {
        self.keys
            .iter()
            .find(|key| key.thumbprint == thumbprint)
            .map(|key| key.clone())
    }
}

#[derive(Clone, Deserialize)]
pub struct Key {
    #[serde(rename(deserialize = "kty"))]
    /// Expected to always be "RSA".
    pub key_type: String,

    #[serde(rename(deserialize = "kid"))]
    pub thumbprint: String,

    #[serde(rename(deserialize = "n"))]
    pub modulus: String,

    #[serde(rename(deserialize = "e"))]
    pub exponent: String,
}
