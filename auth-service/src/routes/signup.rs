use reqwest::StatusCode;
use axum::{response::IntoResponse, extract::State, Json};
use crate::{models::{signup::SignupRequest, user::User}, app_state::AppState, ErrorResponse};
use std::sync::Arc;


pub async fn signup(State(state): State<Arc<AppState>>, payload: axum::Json<SignupRequest>) -> impl IntoResponse {
  println!("routes::signup -> Payload Recebido para signup: {:?}", payload);
  
  let payload_for_check = SignupRequest {
    email: payload.email.clone(),
    password: payload.password.clone(),
    requires_2_fa: payload.requires_2_fa,
  };
  
  let (mut is_valid, mut checked_payload) = is_valid_email_credentials(payload_for_check);
  if !is_valid {
    return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Formato de email inválido".to_string() })).into_response();
  }
  
  (is_valid, checked_payload) = is_valid_password_credentials(checked_payload);
  if !is_valid {
    return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Senha inválida".to_string() })).into_response();
  }
  
  if checked_payload.requires_2_fa {  
    return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "2FA não suportado".to_string() })).into_response();
  } 
  
  let mut user_store = state.user_store.write().await;
  if user_store.get_user(&checked_payload.email).is_ok() {
    return (StatusCode::CONFLICT, Json(ErrorResponse { error: "Usuário já existe".to_string() })).into_response();
  }

  let user = User::new(checked_payload.email, checked_payload.password, checked_payload.requires_2_fa);
  let resultado = user_store.add_user(user);
  if resultado.is_err() {
    eprintln!("routes::signup -> Erro ao adicionar usuário: {:?}", resultado.err());
    return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "Erro ao criar usuário".to_string() })).into_response();
  }

  (StatusCode::CREATED, Json(ErrorResponse { error: "Usuário criado com sucesso".to_string() })).into_response()
} 

fn is_valid_password_credentials(payload: SignupRequest) -> (bool, SignupRequest) {
  if payload.password.len() < 8 {
    return (false, payload);
  }
  (true, payload)
}

fn is_valid_email_credentials(payload: SignupRequest) -> (bool, SignupRequest) {
  if payload.email.is_empty() || payload.password.is_empty() {
    return (false, payload);
  }
  if payload.email.len() < 5 || !payload.email.contains('@') {
    return (false, payload);
  }
  if payload.email.len() > 254 {
    return (false, payload);
  }
  if payload.email.chars().any(|c| !c.is_ascii()) {
    return (false, payload);
  }
  (true, payload)
}