use std::error::Error;

use axum::{Router};
use axum::serve::Serve;
use redis::RedisResult;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
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
use std::time::Duration;  
use crate::utils::constants::{DATABASE_URL, REDIS_URL, POSTMARK_AUTH_TOKEN, POSTMARK_TIMEOUT_SECONDS};
use crate::utils::constants::prod;
use crate::models::email::Email;
use secrecy::SecretString;  

use crate::services::postmark_email_client::PostmarkEmailClient;

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
    tracing::info!("lib::run_until_stopped -> escutando na porta {}", self.address);
    self.server.await?;
    Ok(())
  }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
  pub error: String,
}

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct SignupResponse {
  message: String,
}

impl IntoResponse for AuthAPIError {
  fn into_response(self) -> Response {
    log_error_chain(&self);
    let (status, error_message) = match self {
        AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "Usuário já existe".to_string()),
        AuthAPIError::UserNotFound => (StatusCode::NOT_FOUND, "Usuário não encontrado".to_string()),
        AuthAPIError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Credenciais inválidas".to_string()),
        AuthAPIError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Token ausente".to_string()),
        AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Token inválido".to_string()),
        AuthAPIError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Token expirado".to_string()),
        AuthAPIError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        AuthAPIError::UnexpectedError(report) => (StatusCode::INTERNAL_SERVER_ERROR, report.to_string()),
    };  
    let error_response = ErrorResponse {
      error: error_message,
    };
    let body = Json(error_response);
    (status, body).into_response()
  }
}

pub async fn get_postgres_pool(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
  PgPoolOptions::new()
    .max_connections(5);
  let pool = sqlx::PgPool::connect(database_url).await?;
  Ok(pool)
}

pub async fn configure_postgres() -> PgPool {
  //let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let pg_pool = get_postgres_pool(&DATABASE_URL).await.expect("Failed to connect to PostgreSQL");
  sqlx::migrate!("./migrations").run(&pg_pool).await.expect("Failed to run migrations");
  pg_pool
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<redis::Client> {
  redis::Client::open(redis_hostname)
}

pub fn configure_redis() -> redis::Connection {
  get_redis_client(REDIS_URL.to_owned())
    .expect("Failed to create Redis client")
    .get_connection()
    .expect("Failed to connect to Redis")
}

pub fn log_error_chain(error: &(dyn Error + 'static)) {
  let separator = "\n------------------------------\n";
  let mut report = format!("{}{:?}\n", separator, error);
  let mut current = error.source();
  while let Some(cause) = current {
    report.push_str(&format!("Caused by: {:?}\n", cause));
    current = cause.source();
  }
  report.push_str(separator);
  tracing::error!("{}", report);
}

pub fn configure_postmark_email_client() -> PostmarkEmailClient {
    let http_client = Client::builder()
        .timeout(Duration::from_secs(POSTMARK_TIMEOUT_SECONDS))
        .build()
        .expect("Falha ao construir o cliente HTTP para o Postmark");

    PostmarkEmailClient::new(
        SecretString::from(POSTMARK_AUTH_TOKEN.to_owned()),
        Email::new(prod::email_client::SENDER.to_owned().into()).unwrap(),
        Some(prod::email_client::BASE_URL.to_owned()),
        http_client,
    )
}