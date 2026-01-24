use std::error::Error;

use axum::{Router};
use axum::serve::Serve;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use models::error::AuthAPIError;

pub mod routes;
pub mod models;
pub mod app_state;
pub mod services;
use routes::generate_routes;
use app_state::AppState;

#[derive(Debug)]
pub struct Application {
  server: Serve<TcpListener, Router, Router>,
  pub address: String,
}

impl Application {
  pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
    let listener = tokio::net::TcpListener::bind(address).await?;
    let address = listener.local_addr()?.to_string();
    let router = generate_routes(app_state);
    let server = axum::serve(listener, router);
    Ok(Self { server, address })
  }

  pub async fn run_until_stopped(self) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("listening on {}", self.address);
    self.server.await?;
    Ok(())
  }
}


#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
  pub error: String,
}

#[derive(serde::Serialize, Deserialize)]
pub struct SignupResponse {
  message: String,
}

impl IntoResponse for AuthAPIError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "Usuário já existe".to_string()),
      AuthAPIError::UserNotFound => (StatusCode::NOT_FOUND, "Usuário não encontrado".to_string()),
      AuthAPIError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Credenciais inválidas".to_string()),
      AuthAPIError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
    };
    let error_response = ErrorResponse {
      error: error_message,
    };
    let body = Json(error_response);
    (status, body).into_response()
  }
}



//TODO: Implement handlers for login, logout, verify-2fa, and verify-token