use std::{
    collections::HashSet,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    /// A unique ID for this user.
    id: String,
    /// The user's last reported location.
    last_location: Location,
    /// The last time the user's location was reported, in epoch seconds.
    last_update: u64,
    /// The set of user IDs that are allowed to access this user's location.
    shared_to: HashSet<String>,
    /// The set of user IDs that this user is allowed to access. Note that this
    /// list is solely intended as a hint for lookups - this user's ID must be
    // in each other user's `shared_to` list for the access to succeed.
    shared_with_me_hint: HashSet<String>,
}

impl User {
    /// Update this user's location.
    pub fn update_location(&mut self, location: Location) {
        self.last_location = location;
        self.last_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("now() is prior to the UNIX_EPOCH")
            .as_secs();
    }

    /// Checks whether `other_user` is allowed to access this user.
    pub fn allows_access(&self, other_user: &mut User) -> bool {
        self.shared_to.contains(&other_user.id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}
