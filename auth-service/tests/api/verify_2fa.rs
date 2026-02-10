use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
  let app = TestApp::new().await; 

  let body = serde_json::json!({
    "invalid_field": "some_value" 
  });
  let response1 = app.post_2fa_verify(body.clone()).await;
  assert_eq!(response1.status().as_u16(), 422); 
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
  let app = TestApp::new().await; 

  let body = serde_json::json!({
    "email": "teste@gmail.com",
    "login_attempt_id": uuid::Uuid::new_v4().to_string(),
    "2FACode": "123"
  });
  let response1 = app.post_2fa_verify(body.clone()).await;
  assert_eq!(response1.status().as_u16(), 400); 
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
  let app = TestApp::new().await; 

  let signup_body = serde_json::json!({
    "email": "teste@gmail.com",
    "password": "password123",
    "requires2FA": true
  });
  let response = app.post_signup(signup_body).await;
  assert_eq!(response.status().as_u16(), 201);

  let body = serde_json::json!({
    "email": "teste@gmail.com",
    "login_attempt_id": uuid::Uuid::new_v4().to_string(),
    "2FACode": "123456"
  });
  let response1 = app.post_2fa_verify(body.clone()).await;
  assert_eq!(response1.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
  let app = TestApp::new().await; 

  let signup_body = serde_json::json!({
    "email": "teste@gmail.com",
    "password": "password123",
    "requires2FA": true
  });
  let response = app.post_signup(signup_body).await;
  assert_eq!(response.status().as_u16(), 201);

  let body = serde_json::json!({
    "email": "teste@gmail.com",
    "login_attempt_id": uuid::Uuid::new_v4().to_string(),
    "2FACode": "123456"
  });
  let response1 = app.post_2fa_verify(body.clone()).await;
  assert_eq!(response1.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
  let app = TestApp::new().await; 

  let signup_body = serde_json::json!({
    "email": "teste@gmail.com",
    "password": "password123",
    "requires2FA": true
  });
  let response1 = app.post_signup(signup_body).await;
  assert_eq!(response1.status().as_u16(), 201);

  let login_body = serde_json::json!({
    "email": "teste@gmail.com",
    "password": "password123",
  });
  let response2= app.post_login(login_body).await;
  assert_eq!(response2.status().as_u16(), 206);

  let local_two_fa_code_store = app.app_state().await.two_fa_code_store.read().await;
  let login_attempt_id = local_two_fa_code_store.codes.keys().next().unwrap().clone();
  let code = local_two_fa_code_store.codes.get(&login_attempt_id).unwrap().1.clone();
  drop(local_two_fa_code_store); // Liberar o bloqueio de leitura

  let body = serde_json::json!({
    "email": "teste@gmail.com",
    "login_attempt_id": login_attempt_id.as_ref().to_string(),
    "2FACode": code.as_ref().to_string()
  });
  let response3 = app.post_2fa_verify(body.clone()).await;
  assert_eq!(response3.status().as_u16(), 200);

}