use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{models::email::Email, utils::constants::{JWT_SECRET, JWT_COOKIE_NAME}};
pub const TOKEN_TTL_SECONDS: i64 = 600; // 24 horas

#[derive(Debug)]
pub enum GenerateTokenError {
  TokenError(jsonwebtoken::errors::Error),
  UnexpectedError,
}

impl GenerateTokenError {
    pub fn to_string(&self) -> String {
        match self {
            GenerateTokenError::TokenError(err) => format!("Erro na geração do token: {}", err),
            GenerateTokenError::UnexpectedError => "Erro inesperado durante a geração do token".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  sub: String,
  exp: usize,
}

pub fn generate_auth_token(email: &Email) -> Result<Cookie<'static>, GenerateTokenError> {
  let delta = chrono::Duration::try_seconds(TOKEN_TTL_SECONDS)
    .ok_or(GenerateTokenError::UnexpectedError)?;

  let expiration = Utc::now()
    .checked_add_signed(delta)
    .ok_or(GenerateTokenError::UnexpectedError)?
    .timestamp();

  let expiration: usize = expiration
    .try_into()
    .map_err(|_| GenerateTokenError::UnexpectedError)?;

  let claims = Claims {
    sub: email.address.clone(),
    exp: expiration,
  };

  let token = encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(JWT_SECRET.as_ref()),
  ).map_err(GenerateTokenError::TokenError)?;

  let cookie = create_auth_cookie(token);

  Ok(cookie)

}

fn create_auth_cookie(token: String) -> Cookie<'static> {
  let cookie = Cookie::build((JWT_COOKIE_NAME, token))
    .path("/")
    .http_only(true)
    .secure(false) // TODO: Mudar para true em produção
    .same_site(SameSite::Lax)
    .build();

  cookie
}

pub async fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
  use jsonwebtoken::{decode, DecodingKey, Validation};

  let token_data = decode::<Claims>(
    token,
    &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
    &Validation::default(),
  )
  .map(|data| data.claims)?;

  Ok(token_data)
}

pub fn create_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
  let token = encode(
    &Header::default(),
    claims,
    &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
  )?;

  Ok(token)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_generate_auth_cookie() {
    let email = Email {
      address: "teste@example.com".to_string(),
    };

    let cookie = generate_auth_token(&email).unwrap();
    assert_eq!(cookie.name(), JWT_COOKIE_NAME);
    assert_eq!(cookie.value().len() > 0, true);
    assert_eq!(cookie.value().split('.').count() == 3, true);
    assert_eq!(cookie.http_only(), Some(true));
    assert_eq!(cookie.same_site(), Some(SameSite::Lax));
  }

  #[tokio::test]
  async fn test_create_auth_cookie() {
    let token = "teste_token".to_string();
    let cookie = create_auth_cookie(token.clone());
    assert_eq!(cookie.name(), JWT_COOKIE_NAME);
    assert_eq!(cookie.value().len() > 0, true);
    assert_eq!(cookie.value().split('.').count()> 0, true);
    assert_eq!(cookie.http_only(), Some(true));
    assert_eq!(cookie.same_site(), Some(SameSite::Lax));
  }

  #[tokio::test]
  async fn test_generate_auth_token() {
    let email = Email {
      address: "teste@example.com".to_string(),
    };

    let cookie = generate_auth_token(&email).unwrap();
    assert_eq!(cookie.name(), JWT_COOKIE_NAME);
    assert_eq!(cookie.value().len() > 0, true);
    assert_eq!(cookie.value().split('.').count() == 3, true);
    assert_eq!(cookie.http_only(), Some(true));
    assert_eq!(cookie.same_site(), Some(SameSite::Lax));
  }

  #[tokio::test]
  async fn test_validate_token_with_valid_token() {
    let email = Email {
      address: "teste@example.com".to_string(),
    };

    let cookie = generate_auth_token(&email).unwrap();
    let resultado = validate_token(cookie.value()).await;
    assert!(resultado.is_ok());

    let claims = resultado.unwrap();
    assert_eq!(claims.sub, email.address);

    let experiracao = Utc::now()
    .checked_add_signed(chrono::Duration::try_minutes(9).expect("valid duration"))
    .expect("valid timestamp")
    .timestamp();

    assert!(claims.exp > experiracao as usize);
  }

  #[tokio::test]
  async fn test_validate_token_with_invalid_token() {
    let invalid_token = "token_invalido";
    let resultado = validate_token(invalid_token).await;
    assert!(resultado.is_err());
  }

}