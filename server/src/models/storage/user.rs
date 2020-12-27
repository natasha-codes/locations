use std::collections::HashSet;

use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

use crate::models::common::{timestamp_now, Location, Timestamp};

#[derive(Serialize, Deserialize)]
pub struct User {
    /// A unique ID for this user.
    id: String,
    /// The user's display name.
    display_name: String,
    /// The user's last reported location.
    last_location: Location,
    /// The last time the user's location was reported, in epoch seconds.
    last_update: Timestamp,
    /// The set of user IDs that are allowed to access this user's location.
    shared_to: HashSet<String>,
    /// The set of user IDs that this user is allowed to access. Note that this
    /// list is solely intended as a hint for lookups - this user's ID must be
    // in each other user's `shared_to` list for the access to succeed.
    shared_with_me_hint: HashSet<String>,
}

impl User {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn display_name(&self) -> &String {
        &self.display_name
    }

    pub fn last_location(&self) -> Location {
        self.last_location
    }
}

impl User {
    pub fn query_by_id(id: &String) -> Document {
        doc! {
            "id": id
        }
    }

    /// Update this user's location.
    pub fn update_location(&mut self, location: Location) {
        self.last_location = location;
        self.last_update = timestamp_now();
    }

    /// Checks whether `other_user` is allowed to access this user.
    pub fn allows_access(&self, other_user: &mut User) -> bool {
        self.shared_to.contains(&other_user.id)
    }
}
