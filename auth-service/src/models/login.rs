use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
  pub message: String,
  #[serde(rename = "loginAttemptId")]
  pub login_attempt_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginResponse {
  RegularAuth,
  TwoFactorAuthRequired(TwoFactorAuthResponse)
}