use serde::Serialize;

use super::ExternallyExposed;
use crate::models::{
    common::{Location, Timestamp},
    storage::User,
};

#[derive(Serialize)]
pub struct Contact {
    id: String,
    display_name: String,
    last_location: Location,
    last_update: Timestamp,
}

impl From<User> for Contact {
    fn from(stored_user: User) -> Self {
        Contact {
            id: stored_user.id().clone(),
            display_name: stored_user.display_name().clone(),
            last_location: stored_user.last_location(),
            last_update: stored_user.last_update(),
        }
    }
}

impl ExternallyExposed for Contact {}
