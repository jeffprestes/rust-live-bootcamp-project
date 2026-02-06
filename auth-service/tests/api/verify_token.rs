use auth_service::{
  models::{email::Email, verify_token::VerifyTokenRequest},
  utils::{auth::generate_auth_token, constants::JWT_COOKIE_NAME},
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

#[tokio::test]
async fn should_return_401_if_banned_token() {
  let app = TestApp::new().await; 
  let body = serde_json::json!({
    "email": "teste@email.com",
    "password": "password",
    "requires2FA": false
  });

  let response1 = app.post_signup(body.clone()).await;
  assert_eq!(response1.status().as_u16(), 201, "Usuário criado com sucesso");
  
  let mut token: String = String::new();
  let response2 = app.post_login(body.clone()).await;
  assert_eq!(response2.status(), 200);
  println!("should_return_401_if_banned_token -> Cookies after logout: {:?}", response2.cookies().collect::<Vec<_>>());
  assert_eq!(response2.cookies().count()>0, true);
  response2.cookies().for_each(|cookie| {
    println!("should_return_401_if_banned_token -> Cookie Name: {}, Cookie Value: {}", cookie.name(), cookie.value());
    if cookie.name() == JWT_COOKIE_NAME {
      token = cookie.value().to_string();      
    }
  });

  let response3 = app.post_logout().await;
  assert_eq!(response3.status().as_u16(), 200);
  assert_eq!(response3.cookies().count()>0, true);

  let token_request = VerifyTokenRequest {
    token: token,
  };
  let response4 = app.post_verify_token(token_request).await;
  assert_eq!(response4.status().as_u16(), 401); 
  
}