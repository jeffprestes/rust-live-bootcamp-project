use crate::helpers::{get_random_email, TestApp};
use serde::Deserialize;
use auth_service::{utils::constants::JWT_COOKIE_NAME};

#[derive(Deserialize)]
pub struct ErrorResponse {
    error: String,
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
  let app = TestApp::new().await;

  let random_email = get_random_email();

  let test_cases = vec![
    // Missing password
    serde_json::json!({
      "email": random_email,
      "requires2FA": true
    }),
    // Missing email
    serde_json::json!({
      "password": "validpassword123"
    }),
    // Empty payload
    serde_json::json!({}),
  ];
  for body in test_cases {
    let response = app.post_login(body.clone()).await;
    assert_eq!(response.status().as_u16(), 422, "Falha para o payload: {}", body);
  }
  
}

// #[tokio::test]
// async fn should_return_401_if_invalid_credentials() {
//   let app = TestApp::new().await; 
//   let test_cases = vec![
//     // Invalid email format
//     serde_json::json!({
//       "email": "invalidemailformat@teste.com",
//       "password": "validpassword123",
//       "requires2FA": false
//     }),
//     serde_json::json!({
//       "email": get_random_email(),
//       "password": "short789456",
//       "requires2FA": false
//     }),
//   ];
//   for body in test_cases {
//     let response = app.post_login(body.clone()).await;
//     assert_eq!(response.status().as_u16(), 401, "Falha para o payload: {}", body);
//   }
// }


#[tokio::test]
async fn should_return_400_if_invalid_input() {
  let app = TestApp::new().await; 
  let body = serde_json::json!({
    "email": "invalid-email-format",
    "password": "short",
    "requires2FA": false
  });
  let response = app.post_login(body.clone()).await;
  assert_eq!(response.status().as_u16(), 400, "Input inválido para {:?}", body);
  let response_text = response.text().await.expect("Falha ao ler resposta");
  println!("Response Text: {:?}", response_text);
  let error_response: ErrorResponse = serde_json::from_str(&response_text).expect("Falha ao desserializar resposta de erro");
  assert_eq!(error_response.error, "Formato de email inválido");   
}

#[tokio::test]
async fn should_return_404_user_not_found() {
  let app = TestApp::new().await; 
  let body = serde_json::json!({
    "email": "teste@email.com",
    "password": "password",
    "requires2FA": false
  });
  // First login attempt
  let response = app.post_login(body.clone()).await;
  assert_eq!(response.status().as_u16(), 404, "Usuário não encontrado");
}

#[tokio::test]
async fn should_return_200_login_success() {
  let app = TestApp::new().await; 
  let body = serde_json::json!({
    "email": "teste@email.com",
    "password": "password",
    "requires2FA": false
  });
  // First signup attempt
  let response1 = app.post_signup(body.clone()).await;
  assert_eq!(response1.status().as_u16(), 201, "Usuário criado com sucesso");
  // Second signup attempt with the same email
  // First login attempt
  let response2 = app.post_login(body.clone()).await;
  assert_eq!(response2.status(), 200);

  let auth_cookie = response2
    .cookies()
    .find(|c| c.name() == JWT_COOKIE_NAME)
    .expect("Cookie de login não encontrado");
  
  assert!(!auth_cookie.value().is_empty(), "Valor do cookie de login está vazio");
}