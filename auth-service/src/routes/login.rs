use axum::{
  Json,
  extract::State,
  http::StatusCode,
  response::IntoResponse
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use crate::{
  ErrorResponse, app_state::AppState, models::{
    data_store::UserStore as _, email::Email, error::AuthAPIError, login::LoginRequest, password::Password
  }, utils::auth::generate_auth_token_wrap_into_cookie
};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
  pub message: String,
  #[serde(rename = "loginAttemptId")]
  pub login_attempt_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginResponse {
  RegularAuth,
  TwoFactorAuth(TwoFactorAuthResponse),
}

async fn handle_2fa(jar: CookieJar)
-> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
  (
    jar,
    Ok((
      StatusCode::PARTIAL_CONTENT,
      Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA é necessária para este usuário. Por favor, verifique seu dispositivo de autenticação.".to_string(),
        login_attempt_id: uuid::Uuid::new_v4().to_string(),
      })),
    )),
  )
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
  
  match Password::validate(request.password.clone().as_str()) {
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
    let (jar, result) = handle_2fa(jar).await;
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