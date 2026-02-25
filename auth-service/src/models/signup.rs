use secrecy::SecretString;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SignupRequest {
  pub email: SecretString,
  pub password: SecretString,
  #[serde(rename = "requires2FA")]
  pub requires_2_fa: bool,
}