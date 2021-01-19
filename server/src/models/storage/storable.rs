use mongodb::bson::{from_document, to_document, Document};
use serde::{de::DeserializeOwned, Serialize};

use crate::storage::{MongoError, MongoResult};

pub trait Storable: Sized {
    fn to_document(&self) -> MongoResult<Document>;
    fn from_document(document: Document) -> MongoResult<Self>;
}

impl<T: Serialize + DeserializeOwned> Storable for T {
    fn to_document(&self) -> MongoResult<Document> {
        to_document(self).map_err(|bson_err| MongoError::from(bson_err))
    }

    fn from_document(document: Document) -> MongoResult<Self> {
        from_document::<Self>(document).map_err(|bson_err| MongoError::from(bson_err))
    }
}
