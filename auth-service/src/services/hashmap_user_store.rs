use std::collections::HashMap;

use crate::models::user::User;

#[derive(Debug, PartialEq)]  
pub enum UserStoreError {
  UserAlreadyExists,
  UserNotFound,
  InvalidCredentials,
  UnexpectedError,
}

#[derive(Debug)]
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