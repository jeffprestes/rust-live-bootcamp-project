use secrecy::{ExposeSecret, SecretString};
use reqwest::Client;
use color_eyre::eyre::Result; 
use crate::{models::{email::{Email, SendEmailRequest}, email_client::{EmailClient, EmailClientError}}, utils::constants::{MESSAGE_STREAM, POSTMARK_AUTH_HEADER}};

pub struct PostmarkEmailClient {
  auth_token: SecretString,
  client: Client,
  sender: Email,
  base_url: String,
}

impl PostmarkEmailClient {
    pub fn new(auth_token: SecretString, sender: Email, base_url: Option<String>, client: Client) -> Self {
        let base_url = base_url.unwrap_or_else(|| "https://api.postmarkapp.com".to_string());
        Self { auth_token, client, sender, base_url }
    }
}

#[async_trait::async_trait]
impl EmailClient for PostmarkEmailClient {

    #[tracing::instrument(name = "Sending email", skip_all)] 
    async fn send_email(&self, recipient: &Email, subject: &str, html_body: &str) -> Result<(), EmailClientError> {
        let url = format!("{}/email", self.base_url);
        let temp_to = "jeffprestes@novatrix.com.br";
        let request_body = SendEmailRequest{
            from: self.sender.address.expose_secret().as_ref(),
            to: temp_to,
            subject: subject,
            html_body: html_body,
            text_body: html_body, // Postmark recomenda incluir uma versão em texto para melhor entregabilidade
            message_stream: MESSAGE_STREAM
        };
        
        let request = self.client.post(&url)
        .header(POSTMARK_AUTH_HEADER, self.auth_token.expose_secret())
        .json(&request_body);
      
        tracing::info!("postmark_email_client::send_email -> Enviando email para {}\n com assunto '{}'\n corpo: {:?} \n e requisicao '{:?}'", 
          recipient.address.expose_secret(), 
          subject, 
          request_body,
          request
        );

        let response = request.send().await.map_err(|e| {
            tracing::error!("postmark_email_client::send_email -> Erro ao enviar email: {}", e.to_string());
            EmailClientError::SendError(format!("postmark_email_client::send_email -> Erro ao enviar email: {}", e.to_string()))
        })?;


        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await.unwrap_or_else(|e| "postmark_email_client::send_email -> Erro desconhecido ao ler a resposta".to_string() + " " + &e.to_string());
            Err(EmailClientError::SendError(format!("postmark_email_client::send_email -> Falha ao enviar o email. Erro: {}", error_text)))
        }     
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::constants::POSTMARK_TIMEOUT_SECONDS;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use wiremock::matchers::{any, header, header_exists, method, path};
    use wiremock::{Mock, MockServer, Request, ResponseTemplate};

    use super::PostmarkEmailClient;

    // Helper function to generate a test subject
    fn subject() -> String {
        Sentence(1..2).fake()
    }

    // Helper function to generate test content
    fn content() -> String {
        Paragraph(1..10).fake()
    }

    // Helper function to generate a test email
    fn email() -> Email {
        Email::new(SecretString::new(SafeEmail().fake::<String>().into_boxed_str())).unwrap()
    }

    // Helper function to create a test email client
    fn email_client(base_url: String) -> PostmarkEmailClient {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(POSTMARK_TIMEOUT_SECONDS))
            .build()
            .unwrap();
        PostmarkEmailClient::new(SecretString::new(Faker.fake::<String>().into_boxed_str()), email(), Some(base_url), http_client)
    }

    // Custom matcher to validate the email request body
    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("TextBody").is_some()
                    && body.get("MessageStream").is_some()
            } else {
                false
            }
        }
    }

    // Test to ensure the email client sends the expected request
    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        // Set up the mock server to expect a specific request
        Mock::given(header_exists(POSTMARK_AUTH_HEADER))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Execute the send_email function and check the outcome
        let outcome = email_client
            .send_email(&email(), &subject(), &content())
            .await;

        assert!(outcome.is_ok());
    }

    // Test to handle server error responses
    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        // Set up the mock server to respond with a 500 error
        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Execute the send_email function and check the outcome
        let outcome = email_client
            .send_email(&email(), &subject(), &content())
            .await;

        assert!(outcome.is_err());
    }

    // Test to handle request timeouts
    // #[tokio::test]
    // async fn send_email_times_out_if_the_server_takes_too_long() {
    //     let mock_server = MockServer::start().await;
    //     let email_client = email_client(mock_server.uri());

    //     // Set up the mock server to delay the response
    //     let response = ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(70)); // 70 seconds delay
    //     Mock::given(any())
    //         .respond_with(response)
    //         .expect(1)
    //         .mount(&mock_server)
    //         .await;

    //     // Execute the send_email function and check the outcome
    //     let outcome = email_client
    //         .send_email(&email(), &subject(), &content())
    //         .await;

    //     assert!(outcome.is_err());
    // }
}