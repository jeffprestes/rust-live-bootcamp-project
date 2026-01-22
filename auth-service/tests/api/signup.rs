use crate::helpers::{get_random_email, TestApp};

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
    let response = app.post_signup(body.clone()).await;
    assert_eq!(response.status().as_u16(), 422, "Falha para o payload: {}", body);
  }
  
}

#[tokio::test]
async fn should_return_401_if_invalid_credentials() {
  let app = TestApp::new().await; 
  let test_cases = vec![
    // Invalid email format
    serde_json::json!({
      "email": "invalidemailformat@teste.com",
      "password": "validpassword123",
      "requires2FA": false
    }),
    serde_json::json!({
      "email": get_random_email(),
      "password": "short789456",
      "requires2FA": false
    }),
  ];
  for body in test_cases {
    let response = app.post_signup(body.clone()).await;
    assert_eq!(response.status().as_u16(), 401, "Falha para o payload: {}", body);
  }
}

#[tokio::test]
async fn should_return_201_on_successful_signup() {
  let app = TestApp::new().await; 
  let body = serde_json::json!({
    "email": "tes@email.com",
    "password": "password",
    "requires2FA": false
  });
  let response = app.post_signup(body.clone()).await;
  assert_eq!(response.status().as_u16(), 201, "User created successfully");
}