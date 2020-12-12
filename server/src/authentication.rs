use reqwest;
use serde::Deserialize;

pub async fn get_msa_key_set() -> Result<KeySet, reqwest::Error> {
  // See: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc#fetch-the-openid-connect-metadata-document

  // Fill in "consumers" below because our MSA app is only accessible by personal MSAs.
  let msa_authority = format!("https://login.microsoftonline.com/{}/v2.0/", "consumers");
  let discovery_suffix = ".well-known/openid-configuration";

  let openid_metadata_url = msa_authority + discovery_suffix;

  let keys_uri = reqwest::get(&openid_metadata_url)
    .await?
    .json::<OpenIDMetadata>()
    .await?
    .key_roster_uri;

  reqwest::get(&keys_uri).await?.json::<KeySet>().await
}

#[derive(Deserialize)]
struct OpenIDMetadata {
  #[serde(rename(deserialize = "jwks_uri"))]
  key_roster_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct KeySet {
  keys: Vec<Key>,
}

#[derive(Debug, Deserialize)]
pub struct Key {
  #[serde(rename(deserialize = "kty"))]
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
