use auth_service::{
  models::email::Email,
  utils::auth::generate_auth_token,
  models::verify_token::VerifyTokenRequest,
};

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_401_if_invalid_jwt_cookie() {
  let app = TestApp::new().await; 

  let token = VerifyTokenRequest {
    token: "invalid_token".to_string(),
  };
  let response1 = app.post_verify_token(token).await;
  assert_eq!(response1.status().as_u16(), 401); 
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
  let app = TestApp::new().await; 

  let body = serde_json::json!({
    "invalid_field": "some_value" 
  });
  let response1 = app.post_verify_token(body.clone()).await;
  assert_eq!(response1.status().as_u16(), 422); 
} 

#[tokio::test]
async fn should_return_200_if_valid_token() {
  let app = TestApp::new().await;
  let email = Email {
    address: "teste@gmail.com".to_string(),
  }; 
  let valid_token = generate_auth_token(&email).unwrap();
  let token = VerifyTokenRequest {
    token: valid_token,
  };
  let response1 = app.post_verify_token(token).await;
  assert_eq!(response1.status().as_u16(), 200); 
}