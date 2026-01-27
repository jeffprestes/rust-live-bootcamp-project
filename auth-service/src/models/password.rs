#[derive(Debug, PartialEq, Eq)]
pub enum PasswordError {
    InvalidPassword,
}

impl PasswordError {
    pub fn to_string(&self) -> String {
        match self {
            PasswordError::InvalidPassword => "Senha inválida".to_string(),
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            PasswordError::InvalidPassword => "Senha inválida",
        }
    }
}

impl AsRef<str> for PasswordError {
    fn as_ref(&self) -> &str {
        &self.to_str()
    }
}


#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Password(String);

impl Password {
    pub fn new(password: String) -> Result<Self, PasswordError> {
        Password::validate(&password)?;
        Ok(Self(password))
    }

    pub fn validate(password: &str) -> Result<bool, PasswordError> {
        if password.len() < 8 {
            return Err(PasswordError::InvalidPassword);
        }
        Ok(true)
    }

    pub fn to_hash(&self) -> String {
      // TODO: Placeholder for password hashing logic
      format!("hashed_{}", self.0)
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}