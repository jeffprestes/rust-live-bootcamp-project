use dotenvy::dotenv;
use lazy_static::lazy_static;
use secrecy::SecretString; 

pub const JWT_COOKIE_NAME: &str = "jwt-rust-live-bootcamp";
pub const TEN_MINUTES_IN_SECONDS: u64 = 600;
pub const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";
pub const TWO_FA_CODE_KEY_PREFIX: &str = "two_fa_code:";
pub const MESSAGE_STREAM: &str = "test";
pub const POSTMARK_AUTH_HEADER: &str = "X-Postmark-Server-Token";
pub const POSTMARK_TIMEOUT_SECONDS: u64 = 60;

lazy_static! {
  pub static ref JWT_SECRET: String = set_jwt_secret_from_env();
  pub static ref DATABASE_URL: String = set_database_url_from_env();
  pub static ref REDIS_URL: String = set_redis_url_from_env();
  pub static ref POSTMARK_AUTH_TOKEN: SecretString = set_postmark_auth_token(); 
}

fn set_jwt_secret_from_env() -> String {
  dotenv().ok();
  let secret = std::env::var(env::JWT_SECRET_ENV_VAR).expect("O token para o JWT deve ser definido");
  if secret.is_empty() {
    panic!("O token para o JWT não pode ser vazio");
  }
  secret
}

fn set_database_url_from_env() -> String {
  dotenv().ok();
  let database_url = std::env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL deve ser definido");
  if database_url.is_empty() {
    panic!("DATABASE_URL não pode ser vazio");
  }
  database_url
}

fn set_redis_url_from_env() -> String {
  dotenv().ok();
  let redis_url = std::env::var(env::REDIS_URL_ENV_VAR).unwrap_or(dev::DEFAULT_REDIS_URL.to_string());
  if redis_url.is_empty() {
    panic!("REDIS_URL não pode ser vazio");
  }
  redis_url
}

fn set_postmark_auth_token() -> SecretString {
  dotenv().ok();
  let token = std::env::var(env::POSTMARK_AUTH_TOKEN_ENV_VAR)
    .expect("POSTMARK_AUTH_TOKEN deve ser definido");
  if token.is_empty() {
    panic!("POSTMARK_AUTH_TOKEN não pode ser vazio");
  }
  SecretString::new(token.into())
}

pub mod prod {
  pub const APP_ADDRESS: &str = "0.0.0.0:3000";
  pub const DEFAULT_REDIS_URL: &str = "redis://redis:6379";
  pub mod email_client {
        use std::time::Duration;
        pub const BASE_URL: &str = "https://api.postmarkapp.com";
        // If you created your own Postmark account, make sure to use your email address!
        pub const SENDER: &str = "jeffprestes@novatrix.com.br";
        pub const TIMEOUT: Duration = std::time::Duration::from_secs(60);
    }
}
pub mod dev {
  pub const DEFAULT_REDIS_URL: &str = "redis://localhost:6379";
  pub const APP_ADDRESS: &str = "127.0.0.1:0";
  pub mod email_client {
        use std::time::Duration;

        pub const SENDER: &str = "test@email.com";
        pub const TIMEOUT: Duration = std::time::Duration::from_millis(200);
    }
}

pub mod env {
  pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
  pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
  pub const REDIS_URL_ENV_VAR: &str = "REDIS_URL";
  pub const POSTMARK_AUTH_TOKEN_ENV_VAR: &str = "POSTMARK_AUTH_TOKEN";
}
