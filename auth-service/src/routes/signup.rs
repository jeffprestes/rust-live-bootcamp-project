use reqwest::StatusCode;
use axum::{response::IntoResponse, extract::State, Json};
use crate::{ErrorResponse, SignupResponse, app_state::AppState, models::{data_store::UserStore as _, email::Email, signup::SignupRequest, user::User}};
use std::sync::Arc;


pub async fn signup(State(state): State<Arc<AppState>>, payload: axum::Json<SignupRequest>) -> impl IntoResponse {
  println!("routes::signup -> Payload Recebido para signup: {:?}", payload);
   
  let new_email_result = Email::new(payload.email.clone());
  if new_email_result.is_err() {
    return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: new_email_result.err().unwrap().to_string() })).into_response();
  }
  let new_email = new_email_result.unwrap();

  let mut is_valid = match Email::validate(&new_email.address) {
    Ok(valid) => valid,
    Err(_) => false,
  };
  if !is_valid {
    return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Formato de email inválido".to_string() })).into_response();
  }
  
  is_valid = is_valid_password_credentials(payload.password.clone());
  if !is_valid {
    return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "Senha inválida".to_string() })).into_response();
  }
  
  if payload.requires_2_fa {  
    return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "2FA não suportado".to_string() })).into_response();
  } 
  
  let mut user_store = state.user_store.write().await;
  if user_store.get_user(&new_email.address).await.is_ok() {
    return (StatusCode::CONFLICT, Json(ErrorResponse { error: "Usuário já existe".to_string() })).into_response();
  }

  let user = User::new(new_email, payload.password.clone(), payload.requires_2_fa);
  let resultado = user_store.add_user(user).await;
  if let Err(e) = resultado {
    eprintln!("routes::signup -> Erro ao adicionar usuário: {:?}", e);
    return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "Erro ao criar usuário".to_string() })).into_response();
  }

  (StatusCode::CREATED, Json(SignupResponse { message: "Usuário criado com sucesso".to_string() })).into_response()
} 

fn is_valid_password_credentials(password: String) -> bool {
  if password.len() < 8 {
    return false;
  }
  true
}
