use auth_service::models::data_store::TwoFACodeStore;
use auth_service::{Application, utils::constants::dev};
use reqwest::cookie::Jar;
use std::sync::Arc;
use auth_service::services::*;
use auth_service::app_state::AppState;
use sqlx::postgres::{PgConnectOptions, PgConnection};
use std::str::FromStr;
use sqlx::Connection;
use sqlx::Executor;

pub struct TestApp {
  pub address: String,
  pub http_client: reqwest::Client,
  pub cookie_jar: Arc<Jar>,
  pub app_state: auth_service::app_state::AppState,
}

impl TestApp {
  pub async fn new() -> Self {

    dotenvy::dotenv().ok();

    let pg_pool = auth_service::configure_postgres().await;

    let user_state = Arc::new(tokio::sync::RwLock::new(
      data_stores::db::PostgresUserStore::new(pg_pool.clone()),
    ));

    let banned_token_store = Arc::new(tokio::sync::RwLock::new(
      hashmap_banned_token_store::HashsetBannedTokenStore::new(),
    ));

    let two_fa_code_store = Arc::new(tokio::sync::RwLock::new(
      hashmap_2fa_code_store::HashMapTwoFACodeStore::new().await,
    ));

    let email_client = Arc::new(email_client::MockEmailClient{});

    let app_state = auth_service::app_state::AppState::new(user_state, banned_token_store, two_fa_code_store, email_client);

    let app = Application::build(app_state.clone(), dev::APP_ADDRESS)
    .await  
    .expect("Falha ao criar aplicação.");

    let address = format!("http://{}", app.address.clone());

    //Executando o auth_service em segundo plano numa tarefa assincrona para 
    //evitar bloquear a tarefa principal de teste.
    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(app.run_until_stopped());

    let cookie_jar = Arc::new(Jar::default());

    let http_client = reqwest::Client::builder()
      .redirect(reqwest::redirect::Policy::none())
      .cookie_provider(cookie_jar.clone())
      .build()
      .expect("Falha ao criar cliente HTTP.");


    Self { address, http_client, cookie_jar, app_state }
  
  }

  #[allow(dead_code)]
  pub async fn clean_up(&self) {
    self.get_cleanup().await;
  }

  pub async fn app_state(&self) -> &AppState  {
    &self.app_state
  }

  pub async fn get_root(&self) -> reqwest::Response {
    self.http_client
      .get(&format!("{}/", &self.address))
      .send()
      .await
      .expect("Falha ao executar requisição GET para raiz.")
  }

  pub async fn post_signup<Body>(&self, body: Body) -> reqwest::Response where Body: serde::Serialize {
    self.http_client
      .post(&format!("{}/signup", &self.address))
      .json(&body)
      .send()
      .await
      .expect("Falha ao executar requisição POST para /signup.")
  }

  pub async fn post_login<Body>(&self, body: Body) -> reqwest::Response where Body: serde::Serialize {
    self.http_client
      .post(&format!("{}/login", &self.address))
      .json(&body)
      .send()
      .await
      .expect("Falha ao executar requisição POST para /login.")
  }

  pub async fn post_logout(&self) -> reqwest::Response {
    self.http_client
      .post(&format!("{}/logout", &self.address))
      .send()
      .await
      .expect("Falha ao executar requisição POST para /logout.")
  }

  pub async fn get_cleanup(&self) -> reqwest::Response {
    self.http_client
      .get(&format!("{}/cleanup", &self.address))
      .send()
      .await
      .expect("Falha ao executar requisição GET para /cleanup.")
  }

  pub async fn get_cleanup_single_user(&self, email: String) -> reqwest::Response {
    self.http_client
      .get(&format!("{}/cleanup/{}", &self.address, email))
      .send()
      .await
      .expect("Falha ao executar requisição GET para /cleanup/:email.")
  }

  pub async fn post_verify_token<Body>(&self, body: Body) -> reqwest::Response where Body: serde::Serialize {
    self.http_client
      .post(&format!("{}/verify-token", &self.address))
      .json(&body)
      .send()
      .await
      .expect("Falha ao executar requisição POST para /verify-token.")
  }

  pub async fn post_2fa_verify<Body>(&self, body: Body) -> reqwest::Response where Body: serde::Serialize {
    self.http_client
      .post(&format!("{}/verify-2fa", &self.address))
      .json(&body)
      .send()
      .await
      .expect("Falha ao executar requisição POST para /verify-2fa .")
  }
}  


pub fn get_random_email() -> String {
  format!("{}@example.com", uuid::Uuid::new_v4())
}

#[allow(dead_code)]
pub async fn delete_database (db_name: &str) {
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL não definido");
  let connection_options = PgConnectOptions::from_str(&database_url)
    .expect("Falha ao parsear DATABASE_URL");
  let mut connection = PgConnection::connect_with(&connection_options)
    .await
    .expect("Falha ao conectar ao banco de dados para limpeza");

  connection.execute(format!(r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                  AND pid <> pg_backend_pid();
        "#, db_name).as_str())
    .await
    .expect("Falha ao deletar banco de dados de teste");
  connection
        .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to drop the database.");
}