use std::collections::HashSet;

use crate::models::data_store::BannedTokenStore;

#[derive(Debug, Default)]
pub struct HashsetBannedTokenStore {
  banned_tokens: HashSet<String>,
}

impl HashsetBannedTokenStore {
  pub fn new() -> Self {
    Self {
      banned_tokens: HashSet::new(),
    }
  }

}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
  async fn ban_token(&mut self,token: &str) -> Option<()>  {
      self.banned_tokens.insert(token.to_string());
      Some(())
  }

  async fn is_token_banned(&self,token: &str) -> bool {
      self.banned_tokens.contains(token)
  }
} 
