use std::time::Duration;

use mongodb::{error::Error as MongoError, options::ClientOptions, Client};

pub struct DbClient {}

impl DbClient {
    pub async fn connect() -> Result<(), MongoError> {
        let mut options = ClientOptions::parse("mongodb://localhost:27017/").await?;
        options.app_name = Some(String::from("Sonar"));
        options.connect_timeout = Some(Duration::from_secs(3));
        options.server_selection_timeout = Some(Duration::from_secs(3));

        let client = Client::with_options(options)?;

        // List the names of the databases in that deployment.
        for db_name in client.list_database_names(None, None).await? {
            println!("{}", db_name);
        }

        Ok(())
    }
}
