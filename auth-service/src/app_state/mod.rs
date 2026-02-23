use std::sync::Arc;

use tokio::sync::RwLock;

use crate::models::data_store::{BannedTokenStore, TwoFACodeStore};
use crate::services::data_stores::db::PostgresUserStore;
use crate::models::email_client::EmailClient;

pub type UserStoreType = Arc<RwLock<PostgresUserStore>>;
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;
pub type TwoFACodeStoreType = Arc<RwLock<dyn TwoFACodeStore + Send + Sync>>;
pub type EmailClientType = Arc<dyn EmailClient + Send + Sync>;


#[derive(Clone)]
pub struct AppState {
  pub user_store: UserStoreType,
  pub banned_token_store: BannedTokenStoreType,
  pub two_fa_code_store: TwoFACodeStoreType,
  pub email_client: EmailClientType,
}

impl AppState {
  pub fn new(
    user_store: UserStoreType, 
    banned_token_store: BannedTokenStoreType, 
    two_fa_code_store: TwoFACodeStoreType,
    email_client: EmailClientType,
  ) -> Self {
    Self { user_store, banned_token_store, two_fa_code_store, email_client }
  }
}