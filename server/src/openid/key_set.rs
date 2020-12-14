use serde::Deserialize;

use crate::openid::authority::Authority;

pub async fn get_key_set(authority: Authority) -> Result<KeySet, reqwest::Error> {
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

#[derive(Debug, Deserialize)]
pub struct KeySet {
    keys: Vec<Key>,
}

impl KeySet {
    pub fn key_with_thumbprint(&self, thumbprint: &str) -> Option<&Key> {
        self.keys.iter().find(|key| key.thumbprint == thumbprint)
    }
}

#[derive(Debug, Deserialize)]
pub struct Key {
    #[serde(rename(deserialize = "kty"))]
    /// Expected to always be "RSA".
    pub key_type: String,

    #[serde(rename(deserialize = "kid"))]
    pub thumbprint: String,

    #[serde(rename(deserialize = "x5c"))]
    x509_certs: Vec<String>,
}

impl Key {
    pub fn key_data(&self) -> &String {
        // The first value in this array is the key to be used for token verification
        &self.x509_certs[0]
    }
}
