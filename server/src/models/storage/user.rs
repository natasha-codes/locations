use std::{
    collections::HashSet,
    time::{SystemTime, UNIX_EPOCH},
};

use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    /// A unique ID for this user.
    pub id: String,
    /// The user's display name.
    pub display_name: String,
    /// The user's last reported location.
    pub last_location: Location,
    /// The last time the user's location was reported, in epoch seconds.
    pub last_update: Timestamp,
    /// The set of user IDs that are allowed to access this user's location.
    shared_to: HashSet<String>,
    /// The set of user IDs that this user is allowed to access. Note that this
    /// list is solely intended as a hint for lookups - this user's ID must be
    // in each other user's `shared_to` list for the access to succeed.
    shared_with_me_hint: HashSet<String>,
}

impl User {
    pub fn query_by_id(id: String) -> Document {
        doc! {
            "id": id
        }
    }

    /// Update this user's location.
    pub fn update_location(&mut self, location: Location) {
        self.last_location = location;
        self.last_update = Timestamp::now();
    }

    /// Checks whether `other_user` is allowed to access this user.
    pub fn allows_access(&self, other_user: &mut User) -> bool {
        self.shared_to.contains(&other_user.id)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Timestamp(u64);

impl Timestamp {
    pub fn now() -> Self {
        Timestamp(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime::now() is prior to the UNIX_EPOCH")
                .as_secs(),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}
