use std::{
    convert::TryFrom,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::models::api::ExternallyExposedIncoming;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ping {
    /// Location of the ping.
    location: Location,
    /// Time of the ping, in epoch-seconds. Needs to be `i64` rather
    /// than `u64` because for some reason Mongo's BSON does not
    /// support `u64`.
    timestamp: i64,
}

impl Ping {
    pub fn new_at_now(location: Location) -> Self {
        let now_unsigned: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime::now() is prior to the UNIX_EPOCH")
            .as_secs();

        let now_signed: i64 =
            i64::try_from(now_unsigned).expect("Could not convert unsigned timestamp to signed");

        Self {
            location,
            timestamp: now_signed,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}

impl ExternallyExposedIncoming<'_> for Location {}
