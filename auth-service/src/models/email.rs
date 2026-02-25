use secrecy::SecretString;
use secrecy::ExposeSecret;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq, Eq)]
pub enum EmailError {
    InvalidEmail,
}

impl EmailError {
    pub fn to_string(&self) -> String {
      match self {
        EmailError::InvalidEmail => "Formato de email inválido".to_string()
      }
    }
    pub fn to_str(&self) -> &str {
      match self {
        EmailError::InvalidEmail => "Formato de email inválido"
      }
    }
}

#[derive(Debug, Clone)]
pub struct Email {
    pub address: SecretString,
}

impl Serialize for Email {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.address.expose_secret())
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let secret = SecretString::new(s.into());
        Email::new(secret).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl Email {
    pub fn new(address: SecretString) -> Result<Self, EmailError> {
      Email::validate(&address)?;
      Ok(Self { address })
    }

    pub fn validate_string(address: String) -> Result<bool, EmailError> {
      let secret_address = SecretString::new(address.into());
      Email::validate(&secret_address)
    }

    pub fn validate(address: &SecretString) -> Result<bool, EmailError> {
      let address = address.expose_secret().to_string();
      if address.is_empty() {
        return Err(EmailError::InvalidEmail);
      }
      if address.len() < 5 || !address.contains('@') || !address.contains('.') {
        return Err(EmailError::InvalidEmail);
      }
      if address.len() > 254 {
        return Err(EmailError::InvalidEmail);
      }
      if address.chars().any(|c| !c.is_ascii()) {
        return Err(EmailError::InvalidEmail);
      }
      Ok(true)  
    }
}

impl AsRef<SecretString> for Email {
    fn as_ref(&self) -> &SecretString {
        &self.address
    }
}

// Define the structure of the email request body
// For more information about the request structure, see the API docs: https://postmarkapp.com/developer/user-guide/send-email-with-api
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SendEmailRequest<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub subject: &'a str,
    pub html_body: &'a str,
    pub text_body: &'a str,
    pub message_stream: &'a str,
}