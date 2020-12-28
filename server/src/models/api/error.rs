use rocket::{
    http::Status,
    response::{Responder, Result as ResponderResult},
    Request,
};

use crate::{auth::AuthError, storage::MongoError};

/// An enum wrapping sub-error types and mapping them to an HTTP status code,
/// to simplify exposing errors through the API layer.
pub enum ApiError {
    Auth(AuthError),
    Mongo(MongoError),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> ResponderResult<'o> {
        match self {
            ApiError::Auth(_auth_error) => Status::Unauthorized,
            ApiError::Mongo(_mongo_error) => Status::InternalServerError,
        }
        .respond_to(req)
    }
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self {
        ApiError::Auth(err)
    }
}

impl From<MongoError> for ApiError {
    fn from(err: MongoError) -> Self {
        ApiError::Mongo(err)
    }
}
