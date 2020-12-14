use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize};

use crate::openid::authority::Authority;

#[async_trait(?Send)]
pub trait KeySetFetcher {
    type Error;

    async fn fetch<Claims: DeserializeOwned>(
        &mut self,
        authority: &Authority<Claims>,
    ) -> Result<KeySet, Self::Error>;
}

pub struct NetworkKeySetFetcher {}

impl NetworkKeySetFetcher {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl KeySetFetcher for NetworkKeySetFetcher {
    type Error = reqwest::Error;

    async fn fetch<Claims: DeserializeOwned>(
        &mut self,
        authority: &Authority<Claims>,
    ) -> Result<KeySet, Self::Error> {
        let keys_uri = reqwest::get(&authority.metadata_path())
            .await?
            .json::<Metadata>()
            .await?
            .key_roster_uri;

        reqwest::get(&keys_uri).await?.json::<KeySet>().await
    }
}

#[derive(Deserialize)]
struct Metadata {
    #[serde(rename(deserialize = "jwks_uri"))]
    key_roster_uri: String,
}

#[derive(Clone, Deserialize)]
pub struct KeySet {
    keys: Vec<Key>,
}

impl KeySet {
    pub fn empty() -> Self {
        Self { keys: vec![] }
    }

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
