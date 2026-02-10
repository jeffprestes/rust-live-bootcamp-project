use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;

use crate::{
    AppState, models::verify_2fa::{Verify2FARequest, Verify2FAResponse}
};

pub async fn verify_2fa(
    State(_state): State<Arc<AppState>>,
    _jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> impl IntoResponse {
  println!("routes::verify_2fa -> Payload Recebido para verify_2fa: {:?}", request);
  let token = request.token.trim();
  if token.is_empty() {
    return (
      StatusCode::BAD_REQUEST,
      Json(Verify2FAResponse { message: "Token ausente".to_string() }),
    )
      .into_response();
  }

  (StatusCode::OK, Json(Verify2FAResponse  { message: "Token válido".to_string() })).into_response()
}


