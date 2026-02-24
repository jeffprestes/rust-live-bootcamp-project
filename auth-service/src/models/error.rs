use thiserror::Error;
use color_eyre::eyre::Report;

#[derive(Debug, Error)]
pub enum AuthAPIError {
    #[error("Usuário já existe")]
    UserAlreadyExists,
    #[error("Usuário não encontrado")]
    UserNotFound,
    #[error("Credenciais inválidas")]
    InvalidCredentials,
    #[error("Erro interno: {0}")]
    InternalError(String),
    #[error("Token ausente")]   
    MissingToken,
    #[error("Token inválido")]
    InvalidToken,
    #[error("Token expirado")]
    ExpiredToken,
    #[error("Erro de banco de dados: {0}")]
    DatabaseError(String),
    #[error("Erro inesperado")]
    UnexpectedError(#[source] Report),
}

