use std::sync::Arc;

use axum::extract::State;
use axum::{
  http::StatusCode,
  response::IntoResponse
};
use axum_extra::extract::cookie;
use axum_extra::extract::{
  CookieJar,
  cookie::Cookie,
};

use crate::app_state::AppState;
use crate::models::error::AuthAPIError;
use crate::utils::auth::validate_token;
use crate::utils::constants::JWT_COOKIE_NAME;


pub async fn logout (
    State(app_state): State<Arc<AppState>>, 
    jar: CookieJar,
  ) -> impl IntoResponse {
  let cookie = jar.get(JWT_COOKIE_NAME);
  if cookie.is_none() {
    return AuthAPIError::MissingToken.into_response();
  }

  let token_string = cookie.unwrap().value().to_owned();
  
  let resultado = validate_token(app_state.as_ref(), token_string.as_str()).await;
  match resultado {
      Err(err) => {
        eprintln!("routes::logout -> Erro ao validar token durante logout: {:?}", err);
        return AuthAPIError::InvalidToken.into_response();
      },
      Ok(_) => { () },
  }

  let ban_result = app_state.banned_token_store
    .write()
    .await
    .ban_token(&token_string).await;

  if ban_result.is_none() {
    eprintln!("routes::logout -> Erro ao banir token durante logout");
    return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
  }

  println!("routes::logout -> Token banido com sucesso durante logout: {:?}", token_string);

  let cleared_cookie = Cookie::build((JWT_COOKIE_NAME, ""))
    .path("/")
    .http_only(true)
    .same_site(cookie::SameSite::Lax)
    .secure(true);

  let cleared_jar = jar.remove(cleared_cookie);

  (cleared_jar, StatusCode::OK).into_response()
}