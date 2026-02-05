use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;

use crate::{
    AppState, models::verify_token::{VerifyTokenRequest, VerifyTokenResponse}, utils::auth::validate_token,
};

pub async fn verify_token(
    State(_state): State<Arc<AppState>>,
    _jar: CookieJar,
    Json(request): Json<VerifyTokenRequest>,
) -> impl IntoResponse {
  println!("routes::verify_token -> Payload Recebido para verify_token: {:?}", request);
  let token = request.token.trim();
  if token.is_empty() {
    return (
      StatusCode::BAD_REQUEST,
      Json(VerifyTokenResponse { message: "Token ausente".to_string() }),
    )
      .into_response();
  }

  // Validate token signature and expiration.
  let validation = validate_token(token).await;

  if let Err(err) = validation {
    eprintln!("routes::verify_token -> Token inválido: {:?}", err);
    return (
      StatusCode::UNAUTHORIZED,
      Json(VerifyTokenResponse { message: format!("Token inválido: {err}") }),
    )
      .into_response();
  }

  (StatusCode::OK, Json(VerifyTokenResponse  { message: "Token válido".to_string() })).into_response()
}

