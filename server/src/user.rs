use async_trait::async_trait;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use crate::openid::authority::{Authority, Claims};
use crate::openid::validator::Validator;

pub struct User {
    id: String,
}

impl User {
    pub fn id(&self) -> &String {
        &self.id
    }
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = AuthError;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(auth_header) => {
                let mut validator = Validator::new(Authority::MSA);

                if let Some(token_claims) = validator.validate(auth_header).await {
                    Outcome::Success(Self {
                        id: token_claims.user_id(),
                    })
                } else {
                    Outcome::Failure((Status::Unauthorized, AuthError::InvalidToken))
                }
            }
            None => Outcome::Failure((Status::Unauthorized, AuthError::MissingAuthHeader)),
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingAuthHeader,
    InvalidToken,
}
