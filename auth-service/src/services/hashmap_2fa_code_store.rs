use std::collections::HashMap;
use crate::models::data_store::{TwoFACodeStore, TwoFACodeStoreError, LoginAttemptId, TwoFACode};
use crate::models::email::Email;

#[derive(Debug, Default)]
pub struct HashMapTwoFACodeStore {
  codes: HashMap<LoginAttemptId, (Email, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashMapTwoFACodeStore {

  async fn new() -> Self {
    HashMapTwoFACodeStore {
      codes: HashMap::new(),
    }
  }

  async fn add_code(
    &mut self, 
    email: Email, 
    login_attempt_id: LoginAttemptId,
    code: TwoFACode, 
  ) -> Result<(), TwoFACodeStoreError> {
    if self.codes.contains_key(&login_attempt_id) {
      return Err(TwoFACodeStoreError::CodeAlreadyExists);
    }
    self.codes.insert(login_attempt_id, (email, code));
    Ok(())
  }

  async fn validate_code(
    &mut self, 
    login_attempt_id: &LoginAttemptId, 
    code: &TwoFACode
  ) -> Result<Email, TwoFACodeStoreError> {
    match self.codes.remove(login_attempt_id) {
      Some((email, stored_code)) if stored_code == *code => Ok(email),
      Some((email, stored_code)) => {
        self.codes.insert(login_attempt_id.clone(), (email, stored_code));
        Err(TwoFACodeStoreError::InvalidCode)
      }
      None => Err(TwoFACodeStoreError::InvalidCode),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_add_and_validate_code() {
    let mut store = HashMapTwoFACodeStore::default();
    let email = Email::new("test@example.com".to_string()).unwrap();  
    let login_attempt_id = LoginAttemptId::parse("attempt1".to_string()).unwrap();
    let code = TwoFACode::parse("123456".to_string()).unwrap(); 

    store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await.unwrap();
    let result = store.validate_code(&login_attempt_id, &code).await;
    assert_eq!(result.unwrap(), email);

    let invalid_code = TwoFACode::parse("654321".to_string()).unwrap();
    let result_invalid = store.validate_code(&login_attempt_id, &invalid_code).await;
    assert_eq!(result_invalid.err().unwrap(), TwoFACodeStoreError::InvalidCode);

    let result_nonexistent = store.validate_code(&login_attempt_id, &code).await;
    assert_eq!(result_nonexistent.err().unwrap(), TwoFACodeStoreError::InvalidCode);
  } 
}