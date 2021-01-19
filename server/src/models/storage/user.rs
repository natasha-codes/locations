use std::collections::HashSet;

use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

use crate::models::common::{Location, Ping};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// A unique ID for this user.
    id: String,
    /// The user's display name.
    display_name: String,
    /// The user's last known ping.
    last_ping: Option<Ping>,
    /// The set of user IDs that are allowed to access this user's location.
    shared_to: HashSet<String>,
    /// The set of user IDs that this user is allowed to access. Note that this
    /// list is solely intended as a hint for lookups - this user's ID must be
    // in each other user's `shared_to` list for the access to succeed.
    shared_with_me_hint: HashSet<String>,
}

impl User {
    pub fn new(id: String) -> Self {
        Self {
            id,
            display_name: String::from("Agent 007"),
            last_ping: None,
            shared_to: HashSet::new(),
            shared_with_me_hint: HashSet::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn last_ping(&self) -> Option<Ping> {
        self.last_ping
    }
}

impl User {
    pub fn find_by_id(id: &str) -> Document {
        doc! {
            "id": id
        }
    }

    pub fn update_location(mut self, location: Location) -> Self {
        self.last_ping = Some(Ping::new_at_now(location));

        self
    }
}
