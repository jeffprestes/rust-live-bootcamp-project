use std::hash::Hash;

use uuid::Uuid;
use crate::models::{email::Email, password::HashedPassword};

#[derive(serde::Deserialize, Debug, PartialEq, Clone, Eq, Hash)]

pub struct User {
    pub id: u64,
    pub email: Email,
    pub password: HashedPassword,
    pub requires_2_fa: bool,
}

impl User {
    pub fn new(email: Email, password: String, requires_2_fa: bool) -> Self {
        Self {
            id: generate_random_id(),
            email,
            password: HashedPassword::new(password).unwrap(),
            requires_2_fa,
        }
    }
}

fn generate_random_id() -> u64 {
    // Generate a random u64 from a UUID v4
    let uuid = Uuid::new_v4();
    uuid.as_u128() as u64
}