use uuid::Uuid;

#[derive(serde::Deserialize, Debug, PartialEq, Clone, Eq, Hash)]

pub struct User {
    pub id: u64,
    pub email: String,
    pub password_hash: String,
    pub requires_2_fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2_fa: bool) -> Self {
        Self {
            id: generate_random_id(),
            email,
            password_hash: hash_password(&password),
            requires_2_fa,
        }
    }
}

fn hash_password(password: &str) -> String {
    // Placeholder for password hashing logic
    format!("hashed_{}", password)
}

fn generate_random_id() -> u64 {
    // Generate a random u64 from a UUID v4
    let uuid = Uuid::new_v4();
    uuid.as_u128() as u64
}