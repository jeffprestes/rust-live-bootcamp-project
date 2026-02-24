use reqwest::StatusCode;
use axum::{response::IntoResponse, extract::State, Json};
use crate::{ErrorResponse, SignupResponse, app_state::AppState, models::{data_store::UserStore as _, password::HashedPassword, email::Email, signup::SignupRequest, user::User}};
use std::sync::Arc;

pub async fn signup(State(state): State<Arc<AppState>>, payload: axum::Json<SignupRequest>) -> impl IntoResponse {
  tracing::info!("routes::signup -> Payload Recebido para signup: {:?}", payload);
   
  let new_email_result = Email::new(payload.email.clone());
  if new_email_result.is_err() {
    let err_msg = new_email_result.err().unwrap().to_string();
    tracing::error!("routes::signup -> Erro ao criar email: {:?}", err_msg);
    return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err_msg })).into_response();
  }
  let new_email = new_email_result.unwrap();

  let password_obj = match HashedPassword::new(payload.password.clone()) {
    Ok(password) => password,
    Err(err) => {
      tracing::error!("routes::signup -> Erro ao criar hash da senha: {:?}", err);
      return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err.to_string() })).into_response();
    }
  };
  
  let mut user_store = state.user_store.write().await;
  if user_store.get_user(new_email.as_ref()).await.is_ok() {
    tracing::error!("routes::signup -> Usuário já existe: {:?}", new_email.as_ref());
    return (StatusCode::CONFLICT, Json(ErrorResponse { error: "Usuário já existe".to_string() })).into_response();
  }

  let user = User::new(new_email, password_obj.to_hash(), payload.requires_2_fa);
  let user_for_debug = user.clone();
  let resultado = user_store.add_user(user).await;
  if let Err(e) = resultado {
    tracing::error!("routes::signup -> Erro ao adicionar usuário: {:?}", e);
    return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "Erro ao criar usuário".to_string() })).into_response();
  }
  tracing::info!("routes::signup -> Payload Validado. Usuario criado: {:?}", user_for_debug);
  (StatusCode::CREATED, Json(SignupResponse { message: "Usuário criado com sucesso".to_string() })).into_response()

} 

