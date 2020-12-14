/// An OpenID authority.
#[derive(Copy, Clone)]
pub struct Authority {
    domain: &'static str,
}

impl Authority {
    /// The well-known URI path for the OpenID discovery metadata document.
    const OPENID_DISCOVERY_PATH: &'static str = ".well-known/openid-configuration";

    /// See: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc#fetch-the-openid-connect-metadata-document
    /// Uses "consumers" tenant below because our MSA app is only accessible by personal MSAs.
    pub const MSA: Self = Self {
        domain: "https://login.microsoftonline.com/consumers/v2.0",
    };

    pub fn metadata_path(&self) -> String {
        format!("{}/{}", self.domain, Authority::OPENID_DISCOVERY_PATH)
    }
}
