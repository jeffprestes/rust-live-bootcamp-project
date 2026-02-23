use redis::{Connection, Commands};
use tokio::sync::RwLock;
use std::sync::Arc;


use crate::{models::data_store::BannedTokenStore, utils::constants::BANNED_TOKEN_KEY_PREFIX};

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    } 
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {

  async fn ban_token(&mut self, token: &str) -> Option<()> {
    let banned_key = get_redis_banned_key(token);
    let mut conn = self.conn.write().await;
    let result: Result<(), redis::RedisError> = conn.set(&banned_key, "banned");
    match result {
        Ok(_) => Some(()),
        Err(_) => {
          eprintln!("RedisBannedTokenStore::ban_token => Error banning token in Redis: {:?}", result.err());
          None
        },
    }
  }

  async fn is_token_banned(&self, token: &str) -> bool {
    let banned_key = get_redis_banned_key(token);
    let mut conn = self.conn.write().await;
    let result: Result<Option<String>, redis::RedisError> = conn.get(&banned_key);
    match result {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(_) => {
          eprintln!("RedisBannedTokenStore::is_token_banned => Error checking banned token in Redis: {:?}", result.err());
          false
        },
    }
  }
}
  
pub fn get_redis_banned_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}