use axum::{
  http::StatusCode,
  response::IntoResponse
};
use axum_extra::extract::{
  CookieJar,
  cookie::Cookie
};

use crate::models::error::AuthAPIError;
use crate::utils::auth::validate_token;
use crate::utils::constants::JWT_COOKIE_NAME;


pub async fn logout (jar: CookieJar) -> impl IntoResponse {
  let cookie = jar.get(JWT_COOKIE_NAME);
  if cookie.is_none() {
    return AuthAPIError::MissingToken.into_response();
  }
  
  let token_string = cookie.unwrap().value().to_owned();
  
  let resultado = validate_token(token_string.as_str()).await;
  match resultado {
      Err(err) => {
        eprintln!("routes::logout -> Erro ao validar token durante logout: {:?}", err);
        return AuthAPIError::InvalidToken.into_response();
      },
      Ok(_) => { () },
  }

  let cleared_cookie = Cookie::build((JWT_COOKIE_NAME, ""))
    .path("/")
    .build();

  let cleared_jar = jar.remove(cleared_cookie);

  (cleared_jar, StatusCode::OK).into_response()
}