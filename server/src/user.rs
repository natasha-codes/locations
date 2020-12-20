use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome, Request},
    State,
};

use crate::openid::{Claims, MSAJwtValidator};

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
                let validator_state = try_outcome!(request
                    .guard::<State<MSAJwtValidator>>()
                    .await
                    .map_failure(|_| {
                        (
                            Status::InternalServerError,
                            AuthError::FailedToGetJwtValidator,
                        )
                    }));

                if let Some(token_claims) = validator_state.validate(auth_header).await {
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
    FailedToGetJwtValidator,
    MissingAuthHeader,
    InvalidToken,
}
