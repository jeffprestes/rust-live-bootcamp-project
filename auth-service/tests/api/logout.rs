use auth_service::utils::constants::JWT_COOKIE_NAME;
use reqwest::Url;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
  let app = TestApp::new().await; 

  let response3 = app.post_logout().await;
  assert_eq!(response3.status().as_u16(), 400);
  assert_eq!(response3.cookies().count()>0, false);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
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

  app.cookie_jar.add_cookie_str(
    &format!(
      "{}=invalid; HttpOnly; SameSite=Lax; Path=/; Max-Age=3600; Secure",
      JWT_COOKIE_NAME
    ),
    &Url::parse("http://127.0.0.1").expect("Falha ao parsear URL de teste."),
  );

  let response3 = app.post_logout().await;
  assert_eq!(response3.status().as_u16(), 401);

}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
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
  println!("Cookies after logout: {:?}", response2.cookies().collect::<Vec<_>>());
  assert_eq!(response2.cookies().count()>0, true);
  assert_eq!(response2.status(), 200);

  let response3 = app.post_logout().await;
  assert_eq!(response3.status().as_u16(), 200);
  assert_eq!(response3.cookies().count()>0, true);

}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
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
  println!("Cookies after logout: {:?}", response2.cookies().collect::<Vec<_>>());
  assert_eq!(response2.cookies().count()>0, true);
  assert_eq!(response2.status(), 200);

  let response3 = app.post_logout().await;
  assert_eq!(response3.status().as_u16(), 200);
  assert_eq!(response3.cookies().count()>0, true);

  let response4 = app.post_logout().await;
  assert_eq!(response4.status().as_u16(), 400);
  assert_eq!(response4.cookies().count()>0, false);
}