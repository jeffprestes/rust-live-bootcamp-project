use reqwest::StatusCode;
use axum::response::IntoResponse;
use crate::models::signup::SignupRequest;


pub async fn signup(payload: axum::Json<SignupRequest>) -> impl IntoResponse {
  println!("routes::signup -> Payload Recebido para signup: {:?}", payload);
  
  let payload_for_check = SignupRequest {
    email: payload.email.clone(),
    password: payload.password.clone(),
    requires_2_fa: payload.requires_2_fa,
  };
  let (is_valid, checked_payload) = is_valid_credentials(payload_for_check);
  if !is_valid {
    return StatusCode::UNPROCESSABLE_ENTITY.into_response();
  }
  if checked_payload.requires_2_fa {
    return StatusCode::UNPROCESSABLE_ENTITY.into_response();
  } 
  if checked_payload.password != "password" { 
    return StatusCode::UNAUTHORIZED.into_response();
  }
  if checked_payload.email != "tes@email.com" {
    return StatusCode::UNAUTHORIZED.into_response();
  }
  (StatusCode::CREATED, "User created successfully").into_response()
} 

fn is_valid_credentials(payload: SignupRequest) -> (bool, SignupRequest) {
  if payload.email.is_empty() || payload.password.is_empty() {
    return (false, payload);
  }
  if payload.password.len() < 8 {
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