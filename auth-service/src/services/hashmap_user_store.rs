use std::collections::HashMap;

use crate::models::user::User;
use crate::models::data_store::UserStoreError;
use crate::models::data_store::UserStore;

#[derive(Debug, Default)]
pub struct HashMapUserStore {
  users: HashMap<String, User>,
}

impl HashMapUserStore {
  pub fn new() -> Self {
    Self {
      users: HashMap::new(),
    }
  }
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {

  async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
    if self.users.contains_key(&user.email.address) {
      return Err(UserStoreError::UserAlreadyExists);
    }
    self.users.insert(user.email.address.clone(), user);
    Ok(())
  }

  async fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
    self.users.get(email).ok_or(UserStoreError::UserNotFound)
  }

  async fn validate_user(&self, email: &str, raw_password: &str) -> Result<&User, UserStoreError> {
    let user = self.users.get(email).ok_or(UserStoreError::UserNotFound)?;
    user.password.verify_raw_password(raw_password).await.map_err(|_| UserStoreError::InvalidCredentials)?;
    Ok(user)
  }
}

#[cfg(test)]
mod tests {

use crate::models::email::Email;
use crate::models::password::HashedPassword;

use super::*;

  #[tokio::test]
  async fn test_add_user() {
    let mut store = HashMapUserStore::new();
    let user_mail = Email::new("test@example.com".to_string()).unwrap();
    let user = User::new(user_mail, "password".to_string(), false);
    let result = store.add_user(user.clone()).await;
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_get_user() {
    let mut store = HashMapUserStore::new();
    let user_mail = Email::new("test@example.com".to_string()).unwrap();
    let user = User::new(user_mail, "password".to_string(), false);
    let result: Result<(), UserStoreError> = store.add_user(user.clone()).await;
    assert!(result.is_ok());
    let retrieved_user: Result<&User, UserStoreError> = store.get_user(&user.email.address).await;
    assert_eq!(retrieved_user.unwrap(), &user);
  }

  #[tokio::test]
  async fn test_validate_user() {
    let mut store = HashMapUserStore::new();
    let user_mail = Email::new("test@example.com".to_string()).unwrap();
    let password = "password".to_string();
    let password_hash = HashedPassword::compute_password_hash(&password)
      .await
      .expect("Failed to hash password");
    let hashed_password = HashedPassword::parse_password_hash(password_hash)
      .expect("Failed to parse password hash");
    let user = User {
      id: 1,
      email: user_mail,
      password: hashed_password,
      requires_2_fa: false,
    };
    let result: Result<(), UserStoreError> = store.add_user(user.clone()).await;
    assert!(result.is_ok());
    let retrieved_user: Result<&User, UserStoreError> = store.validate_user(&user.email.address, &password).await;
    assert_eq!(retrieved_user.unwrap().password.as_ref(), user.password.as_ref());
  }
}
