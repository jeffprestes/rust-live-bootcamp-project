use std::error::Error;

use axum::{Router};
use axum::serve::Serve;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use axum::{
    http::{header::{AUTHORIZATION, CONTENT_TYPE}, StatusCode},
    response::{IntoResponse, Response},
    Json
};
use models::error::AuthAPIError;
use routes::generate_routes;
use app_state::AppState;
use tower_http::cors::CorsLayer;

pub mod routes;
pub mod models;
pub mod app_state;
pub mod services;
pub mod utils;

#[derive(Debug)]
pub struct Application {
  server: Serve<TcpListener, Router, Router>,
  pub address: String,
}

impl Application {
  pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {

    let allowed_origins = [
      "http://localhost:3000".parse()?,
      "http://localhost:8000".parse()?,
      "http://138.197.95.239:3000".parse()?,
    ];

    let cors = CorsLayer::new()
      .allow_origin(allowed_origins)
      .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
      .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION])
      .allow_credentials(true);

    let listener = tokio::net::TcpListener::bind(address).await?;
    let address = listener.local_addr()?.to_string();
    let router = generate_routes(app_state, cors);
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
      AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Token ausente".to_string()),
      AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Token inválido".to_string()),
      AuthAPIError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Token expirado".to_string())
    };
    let error_response = ErrorResponse {
      error: error_message,
    };
    let body = Json(error_response);
    (status, body).into_response()
  }
}



//TODO: Implement handlers for login, logout, verify-2fa, and verify-token