#[derive(Debug, Clone)]
pub enum EmailClientError {
  SendError(String),
  InvalidRecipient,
  InternalServerError,
}
use super::email::Email;

#[async_trait::async_trait]
pub trait EmailClient {
  async fn send_email(&self, to: &Email, subject: &str, body: &str) -> Result<(), EmailClientError>;
} 
