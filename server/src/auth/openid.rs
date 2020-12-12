use std::time;

use reqwest;
use serde::Deserialize;

use crate::auth::keys::KeySet;

pub struct OpenIDValidator {
  key_set: KeySet,
  last_refreshed_keyset: time::Instant,
}

impl OpenIDValidator {
  pub fn new() -> Self {
    Self {
      key_set: KeySet::new(),
      last_refreshed_keyset: time::Instant::now(),
    }
  }

  async fn get_msa_key_set() -> Result<KeySet, reqwest::Error> {
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
}

#[derive(Deserialize)]
struct OpenIDMetadata {
  #[serde(rename(deserialize = "jwks_uri"))]
  key_roster_uri: String,
}
