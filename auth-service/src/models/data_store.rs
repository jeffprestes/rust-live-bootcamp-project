#[derive(Debug, PartialEq)]  
pub enum UserStoreError {
  UserAlreadyExists,
  UserNotFound,
  InvalidCredentials,
  UnexpectedError,
}

use super::user::User;

#[async_trait::async_trait]
pub trait UserStore {
  async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
  async fn get_user(&self, email: &str) -> Result<&User, UserStoreError>;
  async fn validate_user(&self, email: &str, password: &str) -> Result<&User, UserStoreError>;
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
  fn ban_token(&mut self, token: &str) -> Option<()>;
  fn is_token_banned(&self, token: &str) -> bool;
}

