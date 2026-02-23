use std::sync::Arc;

use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::models::data_store::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use crate::models::email::Email;
use crate::utils::constants::{TEN_MINUTES_IN_SECONDS, TWO_FA_CODE_KEY_PREFIX};
use crate::configure_redis;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwoFATuple (pub String, pub String);

pub fn get_key_based_on_email(email: &Email) -> String {
  format!("{}{}", TWO_FA_CODE_KEY_PREFIX, email.as_ref())
}

pub struct RedisTwoFACodeStore {
  connection: Arc<RwLock<Connection>>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
  async fn new() -> Self {
    let client = configure_redis();
    let connection = Arc::new(RwLock::new(client));
    Self { connection }
  }

  async fn add_code(
    &mut self, 
    email: Email, 
    login_attempt_id: LoginAttemptId,
    code: TwoFACode, 
  ) -> Result<(), TwoFACodeStoreError> {
    let new_code = TwoFATuple(email.as_ref().to_string(), code.as_ref().to_string());
    let is_valid = self.validate_code(&login_attempt_id, &code).await.is_ok();
    if is_valid {
      return Err(TwoFACodeStoreError::CodeAlreadyExists);
    }
    let mut conn = self.connection.write().await;
    let serialized_code = serde_json::to_string(&new_code).map_err(|e| {
      eprintln!("add_code: Error serializing 2FA code: {}", e);
      TwoFACodeStoreError::UnexpectedError
    })?;
    println!("redis_2fa_code_store::add_code -> Adding 2FA code to Redis with key: {} and value: {}", login_attempt_id.as_ref(), serialized_code);
    conn.set_ex(login_attempt_id.as_ref(), serialized_code, TEN_MINUTES_IN_SECONDS).map_err(|e| {
      eprintln!("redis_2fa_code_store::add_code -> Error adding 2FA code to Redis: {}", e);
      TwoFACodeStoreError::UnexpectedError
    })
  }

  async fn validate_code(
    &self, 
    login_attempt_id: &LoginAttemptId, 
    code: &TwoFACode
  ) -> Result<Email, TwoFACodeStoreError> {
    let mut connection = self.connection.write().await;
    let stored_code: String = connection.get(login_attempt_id.as_ref()).map_err(|_| {
      TwoFACodeStoreError::NotFoundCode
    })?;

    let deserialized: TwoFATuple = serde_json::from_str(&stored_code).map_err(|e| {
      eprintln!("redis_2fa_code_store::validate_code -> Error deserializing 2FA code from Redis: {}", e);
      TwoFACodeStoreError::UnexpectedError
    })?;

    if deserialized.1 == code.as_ref() {
      Ok(Email{address: deserialized.0})
    } else {
      Err(TwoFACodeStoreError::InvalidCode)
    }
  }

  async fn remove_code(&mut self, login_attempt_id: &LoginAttemptId) -> Result<(), TwoFACodeStoreError> {
    let mut conn = self.connection.write().await;
    conn.del(login_attempt_id.as_ref()).map_err(|e| {
      eprintln!("redis_2fa_code_store::remove_code -> Error removing 2FA code from Redis: {}", e);
      TwoFACodeStoreError::UnexpectedError
    })
  }
}