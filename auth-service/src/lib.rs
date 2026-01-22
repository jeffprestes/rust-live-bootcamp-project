use std::error::Error;

use axum::{Router};
use axum::serve::Serve;
use tokio::net::TcpListener;

pub mod routes;
pub mod models;
use routes::generate_routes;

#[derive(Debug)]
pub struct Application {
  server: Serve<TcpListener, Router, Router>,
  pub address: String,
}

impl Application {
  pub async fn build(address: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
    let router = generate_routes();
    let listener = tokio::net::TcpListener::bind(address).await?;
    let address = listener.local_addr()?.to_string();
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