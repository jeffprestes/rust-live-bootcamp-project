use auth_service::Application;

#[tokio::main]
async fn main() {
    let app = Application::build("0.0.0.0:3000")
    .await
    .expect("Falha ao subir a aplicação");

    app.run_until_stopped()
    .await
    .expect("Falha ao rodar a aplicação");
}


