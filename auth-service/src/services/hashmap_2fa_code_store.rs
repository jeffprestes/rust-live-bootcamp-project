use std::collections::HashMap;
use crate::models::data_store::{TwoFACodeStore, TwoFACodeStoreError, LoginAttemptId, TwoFACode};
use crate::models::email::Email;

#[derive(Debug, Default)]
pub struct HashMapTwoFACodeStore {
  pub codes: HashMap<LoginAttemptId, (Email, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashMapTwoFACodeStore {

  async fn new() -> Self {
    HashMapTwoFACodeStore {
      codes: HashMap::new(),
    }
  }

  async fn add_code(&mut self, email: Email, login_attempt_id: LoginAttemptId, code: TwoFACode) -> Result<(), TwoFACodeStoreError> {
    if self.codes.contains_key(&login_attempt_id) {
      return Err(TwoFACodeStoreError::CodeAlreadyExists);
    }
    self.codes.insert(login_attempt_id, (email, code));
    Ok(())
  }

  async fn validate_code(
    &self, 
    login_attempt_id: &LoginAttemptId, 
    code: &TwoFACode
  ) -> Result<Email, TwoFACodeStoreError> {
    match self.codes.get(login_attempt_id) {
      Some((email, stored_code)) if stored_code == code => Ok(email.clone()),
      Some((_email, _stored_code)) => Err(TwoFACodeStoreError::NotFoundCode),
      None => Err(TwoFACodeStoreError::InvalidCode),
    }
  }

  async fn remove_code(&mut self, login_attempt_id: &LoginAttemptId) -> Result<(), TwoFACodeStoreError> {
    if self.codes.remove(login_attempt_id).is_some() {
      Ok(())
    } else {
      Err(TwoFACodeStoreError::NotFoundCode)
    }
  }
}

#[cfg(test)]
mod tests {
  use secrecy::SecretString;
  use secrecy::ExposeSecret;
  use crate::models::email::Email;

use super::*;

  #[tokio::test]
  async fn test_add_and_validate_code() {
    let mut store = HashMapTwoFACodeStore::default();
    let email = Email::new(SecretString::new("test@example.com".to_string().into())).unwrap();  
    let login_attempt_id = LoginAttemptId::parse("attempt1".to_string()).unwrap();
    let code = TwoFACode::parse("123456".to_string()).unwrap(); 

    store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await.unwrap();
    let result = store.validate_code(&login_attempt_id, &code).await;
    assert_eq!(result.unwrap().address.expose_secret(), email.address.expose_secret());

    let invalid_code = TwoFACode::parse("654321".to_string()).unwrap();
    let result_invalid = store.validate_code(&login_attempt_id, &invalid_code).await;
    assert_eq!(result_invalid.err().unwrap(), TwoFACodeStoreError::NotFoundCode);
    
    store.remove_code(&login_attempt_id).await.unwrap();
    let result_nonexistent = store.validate_code(&login_attempt_id, &code).await;
    assert_eq!(result_nonexistent.err().unwrap(), TwoFACodeStoreError::InvalidCode);
  } 
}