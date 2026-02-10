use crate::models::{data_store::LoginAttemptId, email::Email};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Verify2FARequest {
  pub email: Email,
  pub login_attempt_id: LoginAttemptId,
  #[serde(rename = "2FACode")]
  pub token: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Verify2FAResponse {
  pub message: String,
}