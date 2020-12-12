use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeySet {
  keys: Vec<Key>,
}

impl KeySet {
  pub fn new() -> Self {
    Self { keys: vec![] }
  }
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
