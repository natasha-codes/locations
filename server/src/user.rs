use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct User {}

#[derive(Debug)]
pub enum AuthError {
  Unknown,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
  type Error = AuthError;

  fn from_request(_request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
    Outcome::Failure((Status::Unauthorized, AuthError::Unknown))
  }
}
