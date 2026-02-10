use crate::models::{email::Email, email_client::{EmailClient, EmailClientError}};

#[derive(Debug)]
pub struct MockEmailClient;

#[async_trait::async_trait]
impl EmailClient for MockEmailClient {
  async fn send_email(&self, to: &Email, subject: &str, body: &str) 
  -> Result<(), EmailClientError> {
    println!("Mock send email to: {:?} with subject: {} and body: {}", to, subject, body);

    Ok(())
  }
}