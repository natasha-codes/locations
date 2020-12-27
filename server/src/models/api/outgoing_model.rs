use rocket::{
    request::Request,
    response::{self, Responder},
};
use rocket_contrib::json::Json;

use super::ExternallyExposed;

/// A struct wrapping `ExternallyExposed` data to allow it to easily be used as
/// the result of a route handler.
pub struct OutgoingModel<T: ExternallyExposed>(T);

impl<'r, 'o: 'r, T: ExternallyExposed> Responder<'r, 'o> for OutgoingModel<T> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        Json(self.0).respond_to(request)
    }
}

impl<T: ExternallyExposed> From<T> for OutgoingModel<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}
