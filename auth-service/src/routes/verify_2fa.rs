use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use crate::models::data_store::TwoFACode;
use crate::models::data_store::TwoFACodeStoreError;
use crate::models::data_store::TwoFACodeStore;

use crate::utils::auth::generate_auth_token_wrap_into_cookie;
use crate::{
    AppState, ErrorResponse, models::{data_store::UserStore, email::Email, verify_2fa::{Verify2FARequest, Verify2FAResponse}}
};

pub async fn verify_2fa(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> impl IntoResponse {
  println!("routes::verify_2fa -> Payload Recebido para verify_2fa: {:?}", request);
  let token = request.token.trim();
  if token.is_empty() {
    return (
      StatusCode::BAD_REQUEST,
      Json(Verify2FAResponse { message: "Token ausente".to_string() }),
    )
      .into_response();
  }

  let two_fa_code_parsed = TwoFACode::parse(token.to_string());
  let two_fa_code = match two_fa_code_parsed {
    Ok(code) => code,
    Err(err) => {
      return (StatusCode::BAD_REQUEST, Json(Verify2FAResponse { message: format!("Token inválido: {}", err) })).into_response();
    }
  };

  match Email::validate(&request.email.address.as_str()) {
    Ok(_) => (),
    Err(err) => {
      return (jar, (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err.to_string() }))).into_response();
    }
  };

  let user_store = state.user_store.read().await;
  match user_store.get_user(request.email.address.as_str()).await {
    Ok(user) => user,
    Err(_) => {
      return (jar, (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "Usuário não encontrado".to_string() }))).into_response();
    }
  };

  let mut two_fa_code_store = state.two_fa_code_store.write().await;

  match two_fa_code_store.validate_code(&request.login_attempt_id, &two_fa_code).await {
    Ok(_) => (),
    Err(err) => {
      let error_message = match err {
        TwoFACodeStoreError::InvalidCode => "Código 2FA inválido".to_string(),
        TwoFACodeStoreError::ExpiredCode => "Código 2FA expirado".to_string(),
        TwoFACodeStoreError::NotFoundCode => "Código 2FA não encontrado".to_string(),
        _ => "Erro ao validar código 2FA".to_string(),
      };
      return (jar, (StatusCode::UNAUTHORIZED, Json(ErrorResponse { error: error_message }))).into_response();
    }
  }

  let login_attempt_id = request.login_attempt_id.clone();
  let obj_email = match Email::new(request.email.address) {
    Ok(email) => email,
    Err(err) => {
      return (jar, (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err.to_string() }))).into_response();
    }
  };

  
  let auth_cookie = match generate_auth_token_wrap_into_cookie(&obj_email) {
    Ok(cookie) => cookie,
    Err(err) => {
      return (jar, (
        StatusCode::INTERNAL_SERVER_ERROR, 
        Json(
          ErrorResponse { 
            error: format!("Erro ao gerar token de autenticação: {}", err.to_string()) 
          }
        )
      )).into_response();
    }
  };
  
  match two_fa_code_store.remove_code(&login_attempt_id).await {
    Ok(_) => (),
    Err(err) => {
      return (jar, (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("Erro ao remover código 2FA: {err:?}") }))).into_response();
    }
  };

  let updated_jar = jar.add(auth_cookie);

  (updated_jar, (StatusCode::OK, Json(Verify2FAResponse  { message: "Token válido".to_string() }))).into_response()
}


