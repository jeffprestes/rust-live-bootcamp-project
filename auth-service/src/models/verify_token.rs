#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct VerifyTokenRequest {
  pub token: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct VerifyTokenResponse {
  pub message: String,
}