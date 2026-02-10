use auth_service::{Application, services::email_client::MockEmailClient, utils::constants::prod};

#[tokio::main]
async fn main() {

    let user_store = std::sync::Arc::new(tokio::sync::RwLock::new(
        auth_service::services::hashmap_user_store::HashMapUserStore::new(),
    ));

    let banned_token_store = std::sync::Arc::new(tokio::sync::RwLock::new(
        auth_service::services::hashmap_banned_token_store::HashsetBannedTokenStore::new(),
    ));

    let two_fa_code_store = std::sync::Arc::new(tokio::sync::RwLock::new(
        auth_service::services::hashmap_2fa_code_store::HashMapTwoFACodeStore::default(),
    ));

    let email_client: std::sync::Arc<dyn auth_service::models::email_client::EmailClient + Send + Sync> =
        std::sync::Arc::new(MockEmailClient {});


    let app_state = auth_service::app_state::AppState::new(user_store, banned_token_store, two_fa_code_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
    .await
    .expect("Falha ao subir a aplicação");

    app.run_until_stopped()
    .await
    .expect("Falha ao rodar a aplicação");
}


