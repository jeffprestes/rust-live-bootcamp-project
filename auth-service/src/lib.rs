use std::error::Error;

use axum::{Router};
use axum::serve::Serve;
use tokio::net::TcpListener;

pub mod routes;
pub mod models;
pub mod app_state;
pub mod services;
use routes::generate_routes;
use app_state::AppState;

#[derive(Debug)]
pub struct Application {
  server: Serve<TcpListener, Router, Router>,
  pub address: String,
}

impl Application {
  pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
    let listener = tokio::net::TcpListener::bind(address).await?;
    let address = listener.local_addr()?.to_string();
    let router = generate_routes(app_state);
    let server = axum::serve(listener, router);
    Ok(Self { server, address })
  }

  pub async fn run_until_stopped(self) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("listening on {}", self.address);
    self.server.await?;
    Ok(())
  }
}





//TODO: Implement handlers for login, logout, verify-2fa, and verify-token