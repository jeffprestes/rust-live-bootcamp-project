pub enum AuthAPIError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    InternalError(String),
}