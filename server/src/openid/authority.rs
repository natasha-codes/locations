use serde::de::DeserializeOwned;
use serde::Deserialize;

/// The well-known URI path for the OpenID discovery metadata document.
const OPENID_DISCOVERY_PATH: &'static str = ".well-known/openid-configuration";

/// An OpenID authority.
#[derive(Copy, Clone)]
pub struct Authority<C: Claims> {
    domain: &'static str,
    claims: C,
}

impl<C: Claims> Authority<C> {
    pub fn aud(&self) -> &'static str {
        self.claims.aud()
    }

    pub fn metadata_path(&self) -> String {
        format!("{}/{}", self.domain, OPENID_DISCOVERY_PATH)
    }
}

impl Authority<MSAClaims> {
    /// See: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc#fetch-the-openid-connect-metadata-document
    /// Uses "consumers" tenant below because our MSA app is only accessible by personal MSAs.
    pub const MSA: Self = Self {
        domain: "https://login.microsoftonline.com/consumers/v2.0",
        claims: MSAClaims {},
    };
}

pub trait Claims: Copy + DeserializeOwned {
    fn aud(&self) -> &'static str;
}

#[derive(Copy, Clone, Deserialize)]
pub struct MSAClaims {}

impl Claims for MSAClaims {
    fn aud(&self) -> &'static str {
        "our::azure::aud"
    }
}
