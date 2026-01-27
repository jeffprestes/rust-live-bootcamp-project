#[derive(serde::Deserialize, Debug, Clone)]
pub struct SignupRequest {
  pub email: String,
  pub password: String,
  #[serde(rename = "requires2FA")]
  pub requires_2_fa: bool,
}