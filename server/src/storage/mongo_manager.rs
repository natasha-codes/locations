use std::time::Duration;

use mongodb::{error::Error as MongoError, options::ClientOptions, Client};

/*

Notes on Mongo and Rust:

- `bson` is re-exported from the mongodb crate
- Can do `bson::to_document` on and `Serialize` to convert custom types to Mongo document

*/

pub struct MongoManager {
    client: Client,
}

impl MongoManager {
    pub async fn new(uri: &str) -> Result<Self, MongoError> {
        let mut options = ClientOptions::parse(uri).await?;
        options.app_name = Some(String::from("Sonar"));
        options.connect_timeout = Some(Duration::from_secs(3));
        options.server_selection_timeout = Some(Duration::from_secs(3));

        Ok(Self {
            client: Client::with_options(options)?,
        })
    }
}
