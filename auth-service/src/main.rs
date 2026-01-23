use auth_service::Application;

#[tokio::main]
async fn main() {

    let user_store = std::sync::Arc::new(tokio::sync::RwLock::new(
        auth_service::services::hashmap_user_store::HashMapUserStore::new(),
    ));

    let app_state = auth_service::app_state::AppState::new(user_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
    .await
    .expect("Falha ao subir a aplicação");

    app.run_until_stopped()
    .await
    .expect("Falha ao rodar a aplicação");
}


