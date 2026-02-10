#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Verify2FARequest {
  #[serde(rename = "2FACode")]
  pub token: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Verify2FAResponse {
  pub message: String,
}