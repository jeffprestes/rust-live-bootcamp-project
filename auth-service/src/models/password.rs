use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Algorithm, Argon2, Params, PasswordHash, Version};
use argon2::PasswordVerifier;
use argon2::PasswordHasher;
use color_eyre::eyre::{Context, Result, eyre};


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
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn new(password: String) -> Result<Self, PasswordError> {
        HashedPassword::validate(&password)?;
        Ok(Self(password))
    }

    pub fn validate(password: &str) -> Result<bool, PasswordError> {
        if password.len() < 8 {
            return Err(PasswordError::InvalidPassword);
        }
        Ok(true)
    }

    pub async fn parse(password: String) -> Result<Self, String> {
        Self::validate(&password).map_err(|e| e.to_string())?;
        Ok(Self(password))
    }

    pub fn parse_password_hash(password_hash: String) 
    -> Result<HashedPassword, String> {
        Ok(Self(password_hash))
    }

    pub fn to_hash(&self) -> String {
      // TODO: Placeholder for password hashing logic
      format!("hashed_{}", self.0)
    }

    #[tracing::instrument(name = "Verificando senha crua", skip_all)]
    pub async fn verify_raw_password(&self, password: &str) -> Result<()> {
        let current_span: tracing::Span = tracing::Span::current();
        let password_hash = self.as_ref();
        let password = password.to_owned();

        current_span.in_scope(|| {
            let expected_password_hash = PasswordHash::new(password_hash)?;
            
            Argon2::default()
                .verify_password(password.as_bytes(),&expected_password_hash)
                .wrap_err("Falha ao verificar senha crua")?;
            Ok(())
        })
    }

    #[tracing::instrument(name = "Computando hash da senha", skip_all)]
    pub async fn compute_password_hash(password: &str) -> Result<String> {
        let current_span: tracing::Span = tracing::Span::current();
        let password = password.to_owned();
        current_span.in_scope(|| {
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
            .hash_password(password.as_bytes(), &salt).map_err(|e| eyre!("Falha ao computar hash da senha: {}", e))?
            .to_string();
            Ok(password_hash)
        })
    }

}

impl AsRef<str> for HashedPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;

    #[tokio::test]
    async fn empty_string_is_rejected() {
        let result = HashedPassword::new("".to_string());
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn string_less_than_8_characters_is_rejected() {
        let password = "short".to_owned();
        let result = HashedPassword::parse(password).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn can_parse_valid_argon2_hash() {
        let raw_password = "validPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        );
        let hash_string = argon2.hash_password(raw_password.as_bytes(), &salt).unwrap().to_string();
        let hash_password = HashedPassword::parse_password_hash(hash_string.clone()).unwrap();
        assert_eq!(hash_password.as_ref(), hash_string.as_str());
        assert!(hash_password.as_ref().starts_with("$argon2id$v=19$"));
    }

    #[tokio::test]
    async fn can_verify_raw_password() {
        let raw_password = "validPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        );
        let hash_string = argon2.hash_password(raw_password.as_bytes(), &salt).unwrap().to_string();
        let hash_password = HashedPassword::parse_password_hash(hash_string.clone()).unwrap();
        assert_eq!(hash_password.as_ref(), hash_string.as_str());
        assert!(hash_password.as_ref().starts_with("$argon2id$v=19$"));
        let result= hash_password.verify_raw_password(raw_password).await;
        assert!(result.is_ok());        
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let seed = g.size() as u64;
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let password_fake = FakePassword(8..30).fake_with_rng(&mut rng);
            Self(password_fake)
        }
    }

    #[tokio::test]
    #[quickcheck_macros::quickcheck]
    async fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        let result = HashedPassword::parse(valid_password.0.clone()).await;
        assert!(result.is_ok());
        true
    }
}