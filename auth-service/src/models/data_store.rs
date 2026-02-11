#[derive(Debug, PartialEq)]  
pub enum UserStoreError {
  UserAlreadyExists,
  UserNotFound,
  InvalidCredentials,
  UnexpectedError,
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
  CodeAlreadyExists,
  InvalidCode,
  NotFoundCode,
  UnexpectedError,
  ExpiredCode
}

use crate::models::email::Email;

use super::user::User;

#[async_trait::async_trait]
pub trait UserStore {
  async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
  async fn get_user(&self, email: &str) -> Result<&User, UserStoreError>;
  async fn validate_user(&self, email: &str, raw_password: &str) -> Result<&User, UserStoreError>;
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
  fn ban_token(&mut self, token: &str) -> Option<()>;
  fn is_token_banned(&self, token: &str) -> bool;
}

#[async_trait::async_trait]
pub trait TwoFACodeStore {
  async fn new() -> Self where Self: Sized;
  async fn add_code(
    &mut self, 
    email: Email, 
    login_attempt_id: LoginAttemptId,
    code: TwoFACode, 
  ) -> Result<(), TwoFACodeStoreError>;
  async fn validate_code(
    &self, 
    login_attempt_id: &LoginAttemptId, 
    code: &TwoFACode
  ) -> Result<Email, TwoFACodeStoreError>;
  async fn remove_code(&mut self, login_attempt_id: &LoginAttemptId) -> Result<(), TwoFACodeStoreError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
  pub fn parse(id: String) -> Result<Self, String> {
    if id.is_empty() {
      return Err("LoginAttemptId não pode ser vazio".to_string());
    }
    Ok(LoginAttemptId(id))
  }
}

impl Default for LoginAttemptId {
  fn default() -> Self {
    LoginAttemptId(uuid::Uuid::new_v4().to_string())
  }
}

impl AsRef<str> for LoginAttemptId {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
  pub fn parse(code: String) -> Result<Self, String> {
    if code.len() != 6 || !code.chars().all(|c| c.is_digit(10)) {
      return Err("Código 2FA deve ser um número de 6 dígitos".to_string());
    }
    Ok(TwoFACode(code))
  }
}

impl Default for TwoFACode {
  fn default() -> Self {
    TwoFACode(uuid::Uuid::new_v4().to_string())
  }
}

impl AsRef<str> for TwoFACode {
  fn as_ref(&self) -> &str {
    &self.0
  }
}