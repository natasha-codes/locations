use serde::Serialize;

use crate::models::{common::Ping, storage::User};

#[derive(Serialize)]
pub struct Contact {
    id: String,
    display_name: String,
    last_ping: Option<Ping>,
}

impl From<User> for Contact {
    fn from(stored_user: User) -> Self {
        Contact {
            id: String::from(stored_user.id()),
            display_name: String::from(stored_user.display_name()),
            last_ping: stored_user.last_ping().clone(),
        }
    }
}
