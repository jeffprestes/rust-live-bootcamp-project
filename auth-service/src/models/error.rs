pub enum AuthAPIError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    InternalError(String),
    MissingToken,
    InvalidToken,
    ExpiredToken,
}

impl std::fmt::Display for AuthAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthAPIError::UserAlreadyExists => write!(f, "Usuário já existe"),
            AuthAPIError::UserNotFound => write!(f, "Usuário não encontrado"),
            AuthAPIError::InvalidCredentials => write!(f, "Credenciais inválidas"),
            AuthAPIError::InternalError(msg) => write!(f, "Erro interno: {}", msg),
            AuthAPIError::MissingToken => write!(f, "Token ausente"),
            AuthAPIError::InvalidToken => write!(f, "Token inválido"),
            AuthAPIError::ExpiredToken => write!(f, "Token expirado"),
        }
    }
}