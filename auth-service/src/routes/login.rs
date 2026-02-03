use axum::{
  Json,
  extract::State,
  http::StatusCode,
  response::IntoResponse
};
use axum_extra::extract::CookieJar;
use crate::{
  ErrorResponse, 
  SignupResponse, 
  app_state::AppState, 
  utils::auth::generate_auth_token,
  models::{
    data_store::UserStore as _, 
    email::Email, 
    login::LoginRequest, 
    password::Password
  }
};
use std::sync::Arc;

// ...existing code...
pub async fn login(
    State(state): State<Arc<AppState>>, 
    jar: CookieJar,
    Json(request): Json<LoginRequest>
  ) -> impl IntoResponse {
  println!("routes::login -> Payload Recebido para login: {:?}", request);
   
  match Email::validate(&request.email.as_str()) {
    Ok(_) => (),
    Err(err) => {
      return (jar, (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err.to_string() }))).into_response();
    }
  };
  
  match Password::validate(request.password.clone().as_str()) {
    Ok(_) => (),
    Err(err) => {
      return (jar, (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err.to_string() }))).into_response();
    }
  };

  let user_store = state.user_store.read().await;
  if user_store.get_user(request.email.clone().as_str()).await.is_err() {
    return (jar, (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Usuário não encontrado".to_string() }))).into_response();
  }

  println!("routes::login -> Payload Validado para login: {:?}", request);
  let email = match Email::new(request.email.clone()) {
    Ok(email) => email,
    Err(err) => {
      return (jar, (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err.to_string() }))).into_response();
    }
  };

  let auth_cookie = match generate_auth_token(&email) {
    Ok(cookie) => cookie,
    Err(err) => {
      return (jar, (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("Erro ao gerar token de autenticação: {}", err.to_string()) }))).into_response();
    }
  };

  let updated_jar = jar.add(auth_cookie);

  (updated_jar, (StatusCode::OK, Json(SignupResponse { message: "Login bem-sucedido".to_string() }))).into_response()

}