use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use reqwest::StatusCode;
use axum_extra::extract::{CookieJar, cookie::Cookie};

use crate::{app_state::AppState, utils::constants::JWT_COOKIE_NAME};

#[warn(dead_code)]
pub async fn cleanup (
    State(app_state): State<Arc<AppState>>, 
    jar: CookieJar,
  ) -> impl IntoResponse {

    let mut user_store = app_state.user_store.write().await;
    let ban_result = user_store.clear_users().await;
    if let Err(e) = ban_result {
      eprintln!("routes::cleanup -> Erro ao limpar usuários: {:?}", e);
      return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
    println!("routes::cleanup -> Usuários limpos com sucesso");
    let jar = jar.remove(Cookie::build(JWT_COOKIE_NAME).build());
    
    (jar, StatusCode::OK).into_response()
}
