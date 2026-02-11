use axum::{
  Json,
  extract::State,
  http::StatusCode,
  response::IntoResponse
};
use axum_extra::extract::CookieJar;
use crate::{
  ErrorResponse, app_state::AppState, 
  models::{
    data_store::UserStore as _, 
    data_store::{TwoFACodeStore, LoginAttemptId, TwoFACode},
    email::Email,
    error::AuthAPIError, 
    login::{LoginRequest, LoginResponse, TwoFactorAuthResponse}, 
    password::HashedPassword,
  }, 
  utils::auth::generate_auth_token_wrap_into_cookie
};
use std::sync::Arc;
use rand::Rng;


async fn handle_2fa(jar: CookieJar, email: &Email, state: &AppState)
-> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {

  let login_attempt_id = LoginAttemptId::parse(uuid::Uuid::new_v4().to_string()).unwrap();
  let two_fa_code = TwoFACode::parse(format!("{:06}", rand::rng().random_range(0..=999999))).unwrap();
  let mut two_fa_code_store = state.two_fa_code_store.write().await;
  match two_fa_code_store.add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone()).await {
    Ok(_) => (),
    Err(err) => {
      return (jar, Err(AuthAPIError::InternalError(format!("Erro ao armazenar código 2FA: {err:?}"))));
    }
  };

  let msg_body_content = format!("Seu código de autenticação de dois fatores é: {}", two_fa_code.as_ref());
  let msg_body_content_str = msg_body_content.as_str();
  match state.email_client.send_email(
    &email, 
    "Código de Autenticação de Dois Fatores", 
    msg_body_content_str
  ).await {
    Ok(_) => (),
    Err(err) => {
      return (jar, Err(AuthAPIError::InternalError(format!("Erro ao enviar email com código 2FA: {err:?}"))));
    }
  };

  let json_body = TwoFactorAuthResponse {
    message: "2FA é necessária para este usuário. Por favor, verifique seu dispositivo de autenticação.".to_string(),
    login_attempt_id: login_attempt_id.as_ref().to_string(),
  };

  (jar, Ok((StatusCode::PARTIAL_CONTENT, Json(LoginResponse::TwoFactorAuthRequired(json_body)))))
}

async fn handle_no_2fa(email: &Email, jar: CookieJar) 
  -> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
  let auth_cookie = match generate_auth_token_wrap_into_cookie(email) {
    Ok(cookie) => cookie,
    Err(err) => {
      return (jar, Err(AuthAPIError::InternalError(format!("Erro ao gerar token de autenticação: {}", err.to_string()))));
    }
  };

  let updated_jar = jar.add(auth_cookie);

  (updated_jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))
}


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
  
  match HashedPassword::validate(request.password.clone().as_str()) {
    Ok(_) => (),
    Err(err) => {
      return (jar, (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err.to_string() }))).into_response();
    }
  };

  let user_store = state.user_store.read().await;
  let user = match user_store.get_user(request.email.clone().as_str()).await {
    Ok(user) => user,
    Err(_) => {
      return (jar, (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Usuário não encontrado".to_string() }))).into_response();
    }
  };

  println!("routes::login -> Payload Validado para login: {:?}", request);
  let email = match Email::new(request.email.clone()) {
    Ok(email) => email,
    Err(err) => {
      return (jar, (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err.to_string() }))).into_response();
    }
  };

  if user.requires_2_fa {
    let (jar, result) = handle_2fa(jar, &email, &state).await;
    return match result {
      Ok((status, body)) => (jar, (status, body)).into_response(),
      Err(err) => (jar, (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: err.to_string() }))).into_response(),
    };
  }

  let (jar, result) = handle_no_2fa(&email, jar).await;
  match result {
    Ok((status, body)) => (jar, (status, body)).into_response(),
    Err(err) => (jar, (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: err.to_string() }))).into_response(),
  }


}