use auth_service::{
    Application, configure_postgres, configure_redis, models::data_store::TwoFACodeStore as _, services::email_client::MockEmailClient, utils::{constants::prod, tracing::init_tracing}}
;

#[tokio::main]
#[warn(unused_variables)]
async fn main() {

    dotenvy::dotenv().ok();
    color_eyre::install().expect("Falha ao instalar color_eyre");
    init_tracing().expect("Erro ao inicializar o tracing...");

    let pg_pool = configure_postgres().await;
    let conn_redis = configure_redis();

    // let user_store = std::sync::Arc::new(tokio::sync::RwLock::new(
    //     auth_service::services::hashmap_user_store::HashMapUserStore::new(),
    // ));

    let user_store = std::sync::Arc::new(tokio::sync::RwLock::new(
        auth_service::services::data_stores::db::PostgresUserStore::new(pg_pool.clone()),
    ));

    let banned_token_store = std::sync::Arc::new(tokio::sync::RwLock::new(
        auth_service::services::redis_banner_token_store::RedisBannedTokenStore::new(
            std::sync::Arc::new(tokio::sync::RwLock::new(conn_redis)),
        ),
    ));

    let two_fa_code_store = std::sync::Arc::new(tokio::sync::RwLock::new(
        auth_service::services::redis_2fa_code_store::RedisTwoFACodeStore::new().await,
    ));

    let email_client: std::sync::Arc<dyn auth_service::models::email_client::EmailClient + Send + Sync> =
        std::sync::Arc::new(MockEmailClient {});

    let app_state = auth_service::app_state::AppState::new(user_store, banned_token_store, two_fa_code_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
    .await
    .expect("Falha ao subir a aplicação");

    tracing::info!("auth-service::main -> Servidor rodando em {}", app.address);
    tracing::info!("auth-service::main -> versao 20260223-11");
    tracing::info!("auth-service::main -> Pressione Ctrl+C para parar o servidor.");

    app.run_until_stopped()
    .await
    .expect("Falha ao rodar a aplicação");
}


