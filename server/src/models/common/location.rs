use serde::{Deserialize, Serialize};

use crate::models::api::ExternallyExposedIncoming;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}

impl ExternallyExposedIncoming<'_> for Location {}
