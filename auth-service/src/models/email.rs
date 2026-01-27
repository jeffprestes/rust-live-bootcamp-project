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

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email {
    pub address: String,
}

impl Email {
    pub fn new(address: String) -> Result<Self, EmailError> {
      Email::validate(&address)?;
      Ok(Self { address })
    }

    pub fn validate(address: &str) -> Result<bool, EmailError> {
      if address.is_empty() {
        return Err(EmailError::InvalidEmail);
      }
      if address.len() < 5 || !address.contains('@') {
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