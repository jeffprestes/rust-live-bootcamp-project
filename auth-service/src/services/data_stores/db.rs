use sqlx::PgPool;

use crate::models::{data_store::{UserStore, UserStoreError}, email::Email, password::HashedPassword, user::User};
use sqlx::Row;

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
  pub fn new(pool: PgPool) -> Self {
      Self { pool }
  }

  pub async fn clear_users(&mut self) -> Result<(), UserStoreError> {
    let delete_query = "DELETE FROM users";
    sqlx::query(delete_query)
      .execute(&self.pool)
      .await
      .map_err(|e| {
        eprintln!("PostgresUserStore::clear_users -> Erro ao limpar usuários no banco: {:?}", e);
        UserStoreError::DatabaseError(e.to_string())
      })?;
    Ok(())
  }

  pub async fn clear_single_user(&mut self, email: &str) -> Result<(), UserStoreError> {
    let delete_query = "DELETE FROM users WHERE email = $1";
    sqlx::query(delete_query)
      .bind(email)
      .execute(&self.pool)
      .await
      .map_err(|e| {
        eprintln!("PostgresUserStore::clear_single_user -> Erro ao limpar usuário no banco: {:?}", e);
        UserStoreError::DatabaseError(e.to_string())
      })?;
    Ok(())
  }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
  async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
    let insert_query = "INSERT INTO users (id, email, password_hash, requires_2fa) VALUES ($1, $2, $3, $4)";
    sqlx::query(insert_query)
      .bind(user.id as i64) // Convert u64 to i64 for PostgreSQL
      .bind(user.email.address.clone())
      .bind(user.password.as_ref())
      .bind(user.requires_2_fa)
      .execute(&self.pool)
      .await
      .map_err(|e| {
        eprintln!("PostgresUserStore::add_user -> Erro ao inserir usuário no banco: {:?}", e);
        UserStoreError::DatabaseError(e.to_string())
      })?;
    Ok(())
  }

  async fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
    let select_query = "SELECT id, email, password_hash, requires_2fa FROM users WHERE email = $1";
    let row = sqlx::query(select_query)
      .bind(email)
      .fetch_one(&self.pool)
      .await
      .map_err(|e| {
        eprintln!("PostgresUserStore::get_user -> Erro ao buscar usuário no banco: {:?}", e);
        UserStoreError::DatabaseError(e.to_string())
      })?;
    
    let user = User {
      id: row.get::<i64, _>("id") as u64, // Convert i64 back to u64
      email: Email::new(row.get::<String, _>("email")).unwrap(),
      password: HashedPassword::parse_password_hash(row.get::<String, _>("password_hash")).unwrap(),
      requires_2_fa: row.get::<bool, _>("requires_2fa"),
    };
    
    Ok(Box::leak(Box::new(user))) // Return a reference to the user
  }

  async fn validate_user(&self, email: &str, raw_password: &str) -> Result<&User, UserStoreError> {
    let user = self.get_user(email).await?;
    user.password.verify_raw_password(raw_password).await.map_err(|_| UserStoreError::InvalidCredentials)?;
    Ok(user)
  }
}