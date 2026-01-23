use std::collections::HashMap;

use crate::models::user::User;

#[derive(Debug, PartialEq)]  
pub enum UserStoreError {
  UserAlreadyExists,
  UserNotFound,
  InvalidCredentials,
  UnexpectedError,
}

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

  pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
    if self.users.contains_key(&user.email) {
      return Err(UserStoreError::UserAlreadyExists);
    }
    self.users.insert(user.email.clone(), user);
    Ok(())
  }

  pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
    self.users.get(email).ok_or(UserStoreError::UserNotFound)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_add_user() {
    let mut store = HashMapUserStore::new();
    let user = User::new("test@example.com".to_string(), "password".to_string(), false);
    let result = store.add_user(user.clone());
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_get_user() {
    let mut store = HashMapUserStore::new();
    let user = User::new("test@example.com".to_string(), "password".to_string(), false);
    let result = store.add_user(user.clone());
    assert!(result.is_ok());
    let retrieved_user = store.get_user(&user.email);
    assert_eq!(retrieved_user.unwrap(), &user);
  }

  #[tokio::test]
  async fn test_validate_user() {
    let mut store = HashMapUserStore::new();
    let user = User::new("test@example.com".to_string(), "password".to_string(), false);
    let result = store.add_user(user.clone());
    assert!(result.is_ok());
    let mut retrieved_user = store.get_user(&user.email);
    assert_eq!(retrieved_user.unwrap(), &user);
    retrieved_user = store.get_user(&user.email);
    assert_eq!(retrieved_user.unwrap().password_hash, user.password_hash);
  }
}
