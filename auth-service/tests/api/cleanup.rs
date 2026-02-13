use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_200_if_valid_login() {
  let app = TestApp::new().await; 
  let random_email = get_random_email();

  let body = serde_json::json!({
    "email": random_email,
    "password": "password1234",
    "requires2FA": false
  });
  // First signup attempt
  let response1 = app.post_signup(body.clone()).await;
  assert_eq!(response1.status().as_u16(), 201, "Usuário criado com sucesso");
  
  let response2 = app.post_login(body.clone()).await;
  assert_eq!(response2.status(), 200);

  let response4 = app.get_cleanup().await;
  assert_eq!(response4.status().as_u16(), 200);

}