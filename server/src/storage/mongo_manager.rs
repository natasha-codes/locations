use std::time::Duration;

pub use mongodb::error::{Error as MongoError, Result as MongoResult};
use mongodb::{options::ClientOptions, Client, Collection, Database};

use crate::models::{
    common::Location,
    storage::{Storable, User},
};

pub struct MongoManager {
    client: Client,
}

impl MongoManager {
    const DATABASE_NAME: &'static str = "sonar";
    const USERS_COLLECTION_NAME: &'static str = "users";

    /// Create a `MongoManager` by connecting to the Mongo cluster at the
    /// given `uri`. Will fail if connection fails.
    pub async fn new(uri: &str) -> MongoResult<Self> {
        let mut options = ClientOptions::parse(uri).await?;
        options.app_name = Some(String::from("Sonar"));
        options.connect_timeout = Some(Duration::from_secs(3));
        options.server_selection_timeout = Some(Duration::from_secs(3));

        Ok(Self {
            client: Client::with_options(options)?,
        })
    }

    /// Get the `User` with the given `id`. If none exists, one is created.
    pub async fn get_user_by_id(&self, id: &str) -> MongoResult<User> {
        if let Some(existing_user) = self
            .users_collection()
            .find_one(User::find_by_id(id), None)
            .await?
        {
            Ok(User::from_document(existing_user)?)
        } else {
            let new_user = User::new(String::from(id));

            self.users_collection()
                .insert_one(new_user.to_document()?, None)
                .await?;

            Ok(new_user)
        }
    }

    /// Updates the location of the user with the given `id`. If no user exists,
    /// one is created.
    #[allow(dead_code)]
    pub async fn update_user_location(&self, id: &str, location: Location) -> MongoResult<User> {
        let user = self.get_user_by_id(id).await?;
        let updated_user = user.update_location(location);

        self.users_collection()
            .replace_one(User::find_by_id(id), updated_user.to_document()?, None)
            .await?;

        Ok(updated_user)
    }

    fn users_collection(&self) -> Collection {
        self.database()
            .collection(MongoManager::USERS_COLLECTION_NAME)
    }

    fn database(&self) -> Database {
        self.client.database(MongoManager::DATABASE_NAME)
    }
}
