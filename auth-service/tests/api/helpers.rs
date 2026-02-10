use auth_service::models::data_store::TwoFACodeStore;
use auth_service::{Application, utils::constants::dev};
use reqwest::cookie::Jar;
use std::sync::Arc;
use auth_service::services::*;
use auth_service::app_state::AppState;

pub struct TestApp {
  pub address: String,
  pub http_client: reqwest::Client,
  pub cookie_jar: Arc<Jar>,
  pub app_state: auth_service::app_state::AppState,
}

impl TestApp {
  pub async fn new() -> Self {

    let user_state = Arc::new(tokio::sync::RwLock::new(
      hashmap_user_store::HashMapUserStore::new(),
    ));

    let banned_token_store = Arc::new(tokio::sync::RwLock::new(
      hashmap_banned_token_store::HashsetBannedTokenStore::new(),
    ));

    let two_fa_code_store = Arc::new(tokio::sync::RwLock::new(
      hashmap_2fa_code_store::HashMapTwoFACodeStore::new().await,
    ));

    let email_client = Arc::new(email_client::MockEmailClient {});

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

  pub async fn post_verify_token<Body>(&self, body: Body) -> reqwest::Response where Body: serde::Serialize {
    self.http_client
      .post(&format!("{}/verify-token", &self.address))
      .json(&body)
      .send()
      .await
      .expect("Falha ao executar requisição POST para /verify-token.")
  }
  // TODO: Implement helper functions for all other routes (verify-2fa)
}  

pub fn get_random_email() -> String {
  format!("{}@example.com", uuid::Uuid::new_v4())
}
