use std::sync::Arc;

use color_eyre::eyre::{self, Context};
use redis::{Commands, Connection};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::models::data_store::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use crate::models::email::Email;
use crate::utils::constants::{TEN_MINUTES_IN_SECONDS, TWO_FA_CODE_KEY_PREFIX};
use crate::configure_redis;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TwoFATuple (pub String, pub String);

pub fn get_key_based_on_email(email: &Email) -> String {
  format!("{}{}", TWO_FA_CODE_KEY_PREFIX, email.address.expose_secret())
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
    let new_code = TwoFATuple(email.address.expose_secret().to_string(), code.as_ref().to_string());
    let is_valid = self.validate_code(&login_attempt_id, &code).await.is_ok();
    if is_valid {
      return Err(TwoFACodeStoreError::CodeAlreadyExists);
    }
    let mut conn = self.connection.write().await;
    // let serialized_code = serde_json::to_string(&new_code).map_err(|e| {
    //   eprintln!("add_code: Error serializing 2FA code: {}", e);
    //   TwoFACodeStoreError::UnexpectedError(eyre::eyre!(e))
    // })?;
    let serialized_code = serde_json::to_string(&new_code)
    .wrap_err("Falha em serializar o código 2FA")
    .map_err(|e| {
      TwoFACodeStoreError::UnexpectedError(e)
    })?;
    tracing::info!("redis_2fa_code_store::add_code -> Adding 2FA code to Redis with key: {}", 
      login_attempt_id.as_ref(), 
    );
    conn.set_ex(login_attempt_id.as_ref(), serialized_code, TEN_MINUTES_IN_SECONDS)
    .wrap_err("Falha em adicionar o código 2FA ao Redis")
    .map_err(|e| {
      let err_test = format!("{}", e);
      tracing::error!("redis_2fa_code_store::add_code -> Error adding 2FA code to Redis: {}", eyre::eyre!(err_test));
      TwoFACodeStoreError::UnexpectedError(eyre::eyre!(e))
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

    let deserialized: TwoFATuple = serde_json::from_str(&stored_code)
    .wrap_err("Falha em desserializar o código 2FA")
    .map_err(|e| {
      eprintln!("redis_2fa_code_store::validate_code -> Error deserializing 2FA code from Redis: {}", e);
      TwoFACodeStoreError::UnexpectedError(eyre::eyre!(e))
    })?;

    if deserialized.1 == code.as_ref() {
      Ok(Email{address: deserialized.0.into()})
    } else {
      Err(TwoFACodeStoreError::InvalidCode)
    }
  }

  async fn remove_code(&mut self, login_attempt_id: &LoginAttemptId) -> Result<(), TwoFACodeStoreError> {
    let mut conn = self.connection.write().await;
    conn.del(login_attempt_id.as_ref())
    .wrap_err("Falha em remover codigo 2FA do cache Redis")
    .map_err(|e| {
      eprintln!("redis_2fa_code_store::remove_code -> Error removing 2FA code from Redis: {}", e);
      TwoFACodeStoreError::UnexpectedError(eyre::eyre!(e))
    })
  }
}