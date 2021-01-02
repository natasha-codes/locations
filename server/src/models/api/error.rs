use rocket::{
    http::Status,
    response::{Responder, Result as ResponderResult},
    Request,
};

use crate::{auth::AuthError, storage::MongoError};

/// An enum wrapping sub-error types and mapping them to an HTTP status code,
/// to simplify returning errors from a route handler.
///
/// This type implements `From` for its wrapped sub-types. Consequently, when
/// it is used as the top-level error type for a route handler, operations
/// within the handler that produce `Result<T, SubErrorType>` can use `?` to
/// early-return. E.g.,:
///
/// ```rust
/// fn get_auth_tokne() -> Result<String, AuthError> { ... }
///
/// #[get("/route")]
/// async fn handler() -> Result<(), ApiError> {
///    let auth_token = get_auth_result()?;
///    ...
/// }
/// ```
pub enum ApiError {
    Auth(AuthError),
    Mongo(MongoError),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> ResponderResult<'o> {
        match self {
            ApiError::Auth(AuthError::MissingAuthHeader) => Status::BadRequest,
            ApiError::Auth(_) => Status::Unauthorized,
            ApiError::Mongo(_) => Status::InternalServerError,
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
