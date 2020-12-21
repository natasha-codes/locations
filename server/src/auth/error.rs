#[derive(Debug)]
pub enum AuthError {
    FailedToGetJwtValidator,
    MissingAuthHeader,
    InvalidToken,
}
