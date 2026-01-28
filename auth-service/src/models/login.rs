#[derive(serde::Deserialize, Debug, Clone)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}