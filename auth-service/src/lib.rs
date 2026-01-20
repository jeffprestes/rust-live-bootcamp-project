use std::error::Error;

use axum::Router;
use axum::response::Html;
use axum::routing::get;
use axum::serve::Serve;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Debug)]
pub struct Application {
  server: Serve<TcpListener, Router, Router>,
  pub address: String,
}

impl Application {
  pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
    let assets_dir = ServeDir::new("assets");
    let router = Router::new()
        .fallback_service(assets_dir)
        .route("/heartbeat", get(heartbeat_handler));
    let listener = tokio::net::TcpListener::bind(address).await?;
    let address = listener.local_addr()?.to_string();
    let server = axum::serve(listener, router);
    Ok(Self { server, address })
  }

  pub async fn run_until_stopped(self) -> Result<(), Box<dyn Error>> {
    println!("listening on {}", self.address);
    self.server.await?;
    Ok(())
  }
}

async fn heartbeat_handler() -> Html<&'static str> {
    // Done: Update this to a custom message! [done]
    Html("online")
}