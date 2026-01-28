use reqwest::StatusCode;
use axum::{response::IntoResponse, extract::State, Json};
use crate::{ErrorResponse, SignupResponse, app_state::AppState, models::{data_store::UserStore as _, password::Password, email::Email, login::LoginRequest}};
use std::sync::Arc;

pub async fn login(State(state): State<Arc<AppState>>, payload: axum::Json<LoginRequest>) -> impl IntoResponse {
  println!("routes::login -> Payload Recebido para login: {:?}", payload);
   
  match Email::validate(&payload.email.as_str()) {
    Ok(_) => (),
    Err(err) => {
      return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err.to_string() })).into_response();
    }
  };
  
  match Password::validate(payload.password.clone().as_str()) {
    Ok(_) => (),
    Err(err) => {
      return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err.to_string() })).into_response();
    }
  };
  
  let user_store = state.user_store.read().await;
  if user_store.get_user(payload.email.clone().as_str()).await.is_err() {
    return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Usuário não encontrado".to_string() })).into_response();
  }

  println!("routes::login -> Payload Validado para login: {:?}", payload);
  (StatusCode::OK, Json(SignupResponse { message: "Login bem-sucedido".to_string() })).into_response()

} 
