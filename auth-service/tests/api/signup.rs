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
