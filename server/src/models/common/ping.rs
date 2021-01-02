use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::models::api::ExternallyExposedIncoming;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ping {
    /// Location of the ping.
    location: Location,
    /// Time of the ping, in epoch-seconds.
    timestamp: u64,
}

impl Ping {
    pub fn new_at_now(location: Location) -> Self {
        Self {
            location,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime::now() is prior to the UNIX_EPOCH")
                .as_secs(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}

impl ExternallyExposedIncoming<'_> for Location {}
