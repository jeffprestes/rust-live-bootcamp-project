use reqwest::StatusCode;
use axum::response::IntoResponse;


pub async fn signup(payload: axum::Json<SignupRequest>) -> impl IntoResponse {
  println!("routes::signup -> Payload Recebido para signup: {:?}", payload);
  StatusCode::OK.into_response()
}

#[derive(serde::Deserialize, Debug)]
pub struct SignupRequest {
  pub email: String,
  pub password: String,
  #[serde(rename = "requires2FA")]
  pub requires_2_fa: bool,
}