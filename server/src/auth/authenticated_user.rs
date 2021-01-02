use rocket::{
    async_trait,
    http::Status,
    request::{FromRequest, Outcome, Request},
    State,
};

use super::{
    openid::{Claims, MSAJwtValidator},
    AuthError,
};

pub struct AuthenticatedUser(String);

impl AuthenticatedUser {
    pub fn id(self) -> String {
        self.0
    }
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = AuthError;

    /// Allows `AuthenticatedUser` to be used as a Rocket request guard.
    ///
    /// Note that our routes don't guard on an `AuthenticatedUser`, but instead
    /// feed errors from here into `ApiError`. So, the status codes here are not
    /// used directly. Rather, the returned `AuthError` will be used by`ApiError`
    /// to determine the status code to respond with.
    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(auth_header) => {
                let validator_state = try_outcome!(request
                    .guard::<State<MSAJwtValidator>>()
                    .await
                    .map_failure(|_| { (Status::ImATeapot, AuthError::FailedToGetJwtValidator,) }));

                if let Some(token_claims) = validator_state.validate(auth_header).await {
                    Outcome::Success(Self(token_claims.user_id()))
                } else {
                    Outcome::Failure((Status::ImATeapot, AuthError::InvalidToken))
                }
            }
            None => Outcome::Failure((Status::ImATeapot, AuthError::MissingAuthHeader)),
        }
    }
}
