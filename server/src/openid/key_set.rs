use serde::{de::DeserializeOwned, Deserialize};

use crate::openid::authority::Authority;

pub async fn get_key_set<Claims: DeserializeOwned>(
    authority: &Authority<Claims>,
) -> Result<KeySet, reqwest::Error> {
    let keys_uri = reqwest::get(&authority.metadata_path())
        .await?
        .json::<Metadata>()
        .await?
        .key_roster_uri;

    reqwest::get(&keys_uri).await?.json::<KeySet>().await
}

#[derive(Deserialize)]
struct Metadata {
    #[serde(rename(deserialize = "jwks_uri"))]
    key_roster_uri: String,
}

#[derive(Deserialize)]
pub struct KeySet {
    keys: Vec<Key>,
}

impl KeySet {
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
