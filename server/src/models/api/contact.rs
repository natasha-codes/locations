use serde::{Deserialize, Serialize};

use crate::models::storage;

#[derive(Serialize, Deserialize)]
pub struct Contact {
    id: String,
    display_name: String,
    last_location: Location,
    last_update: Timestamp,
}

impl From<storage::User> for Contact {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            display_name: user.display_name,
            last_location: user.last_location,
            last_update: user.last_update,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Timestamp(u64);

#[derive(Serialize, Deserialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}

impl From<storage::Location> for Location {}
