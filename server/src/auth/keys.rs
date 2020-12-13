use serde::Deserialize;

use crate::auth::openid::JWT;

#[derive(Debug, Deserialize)]
pub struct KeySet {
    keys: Vec<Key>,
}

#[derive(Eq, PartialEq)]
pub enum JwtValidationResult {
    Success,
    Failure,
    UnknownKey,
}

impl KeySet {
    pub fn new() -> Self {
        Self { keys: vec![] }
    }

    pub fn validate_jwt(&self, jwt: &JWT) -> JwtValidationResult {
        match self.key_with_id(&jwt.id) {
            Some(signing_key) => {
                if signing_key.validate(&jwt.signature) {
                    JwtValidationResult::Success
                } else {
                    JwtValidationResult::Failure
                }
            }
            None => JwtValidationResult::UnknownKey,
        }
    }

    fn key_with_id(&self, id: &str) -> Option<&Key> {
        self.keys.iter().find(|key| key.thumbprint == id)
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
    pub fn validate(&self, _signature: &str) -> bool {
        // do jwt validation stuff

        true
    }

    pub fn key_data(&self) -> &String {
        // The first value in this array is the key to be used for token verification
        &self.x509_certs[0]
    }
}
