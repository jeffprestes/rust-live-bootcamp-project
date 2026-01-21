use auth_service::Application;

pub struct TestApp {
  pub address: String,
  pub http_client: reqwest::Client,
}

impl TestApp {
  pub async fn new() -> Self {
    let app = Application::build("127.0.0.1:3000")
    .await
    .expect("Falha ao criar aplicação.");

    let address = format!("http://{}", app.address.clone());

    //Executando o auth_service em segundo plano numa tarefa assincrona para 
    //evitar bloquear a tarefa principal de teste.
    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(app.run_until_stopped());
    let http_client = reqwest::Client::builder()
      .redirect(reqwest::redirect::Policy::none())
      .build()
      .expect("Falha ao criar cliente HTTP.");
    Self { address, http_client }
  }

  pub async fn get_root(&self) -> reqwest::Response {
    self.http_client
      .get(&format!("{}/", &self.address))
      .send()
      .await
      .expect("Falha ao executar requisição GET para raiz.")
  }

  // TODO: Implement helper functions for all other routes (signup, login, logout, verify-2fa, and verify-token)
}  