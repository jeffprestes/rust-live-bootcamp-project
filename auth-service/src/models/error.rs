pub enum AuthAPIError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    InternalError(String),
    MissingToken,
    InvalidToken,
    ExpiredToken,
}