use std::time::Duration;

pub use mongodb::error::Result as MongoResult;
use mongodb::{bson, options::ClientOptions, Client, Collection, Database};

use crate::models::storage::User;

pub struct MongoManager {
    client: Client,
}

impl MongoManager {
    const DATABASE_NAME: &'static str = "sonar";
    const USERS_COLLECTION_NAME: &'static str = "users";

    pub async fn new(uri: &str) -> MongoResult<Self> {
        let mut options = ClientOptions::parse(uri).await?;
        options.app_name = Some(String::from("Sonar"));
        options.connect_timeout = Some(Duration::from_secs(3));
        options.server_selection_timeout = Some(Duration::from_secs(3));

        Ok(Self {
            client: Client::with_options(options)?,
        })
    }

    pub async fn get_user_by_id(&self, id: String) -> MongoResult<Option<User>> {
        if let Some(user_doc) = self
            .users_collection()
            .find_one(User::query_by_id(id), None)
            .await?
        {
            let user = bson::from_document::<User>(user_doc)?;

            return Ok(Some(user));
        }

        Ok(None)
    }

    fn users_collection(&self) -> Collection {
        self.database()
            .collection(MongoManager::USERS_COLLECTION_NAME)
    }

    fn database(&self) -> Database {
        self.client.database(MongoManager::DATABASE_NAME)
    }
}
