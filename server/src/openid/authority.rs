use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Deserialize};

/// The well-known URI path for the OpenID discovery metadata document.
const OPENID_DISCOVERY_PATH: &'static str = ".well-known/openid-configuration";

/// An OpenID authority.
pub struct Authority<Claims: DeserializeOwned> {
    domain: &'static str,
    aud: &'static str,
    claims: PhantomData<Claims>,
}

impl<Claims: DeserializeOwned> Authority<Claims> {
    pub fn new(domain: &'static str, aud: &'static str) -> Self {
        Self {
            domain,
            aud,
            claims: PhantomData,
        }
    }

    pub fn aud(&self) -> &'static str {
        self.aud
    }

    pub fn metadata_path(&self) -> String {
        format!("{}/{}", self.domain, OPENID_DISCOVERY_PATH)
    }
}

impl Authority<MSAClaims> {
    /// Represents our app registration with the MSA OpenID Connect authority. Uses
    /// "consumers" tenant because our MSA app is only accessible by personal MSAs.
    ///
    /// See: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc#fetch-the-openid-connect-metadata-document
    pub const MSA: Self = Self {
        domain: "https://login.microsoftonline.com/consumers/v2.0",
        aud: "97b5900d-bdbe-41bf-8afb-39fdcb0993ee",
        claims: PhantomData,
    };
}

#[derive(Deserialize)]
pub struct MSAClaims {}
