use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Algorithm, Argon2, Params, PasswordHash, Version};
use argon2::PasswordVerifier;
use argon2::PasswordHasher;
use color_eyre::eyre::{Context, Result, eyre};
use secrecy::{ExposeSecret, SecretString};

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


#[derive(serde::Deserialize, Debug, Clone)]
pub struct HashedPassword(SecretString);

impl PartialEq for HashedPassword {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl HashedPassword {
    pub fn new(password: SecretString) -> Result<Self, PasswordError> {
        HashedPassword::validate(&password)?;
        Ok(Self(password))
    }

    pub fn validate(password: &SecretString) -> Result<bool, PasswordError> {
        if password.expose_secret().len() < 8 {
            return Err(PasswordError::InvalidPassword);
        }
        Ok(true)
    }

    #[tracing::instrument(name = "HashedPassword Parse", skip_all)]
    pub async fn parse(password: SecretString) -> Result<SecretString> {
        Self::validate(&password).map_err(|e| eyre!(e.to_string()))?;
        Ok(password)
    }

    pub fn parse_password_hash(password_hash: SecretString) 
    -> Result<HashedPassword, String> {
        Ok(Self(password_hash)) 
    }

    pub fn to_hash(&self) -> SecretString {
      // TODO: Placeholder for password hashing logic
      SecretString::new(format!("hashed_{}", self.0.expose_secret()).into_boxed_str())
    }

    #[tracing::instrument(name = "Verificando senha crua", skip_all)]
    pub async fn verify_raw_password(&self, password: &str) -> Result<()> {
        let current_span: tracing::Span = tracing::Span::current();
        let password_hash = self.as_ref();
        let password = password.to_owned();

        current_span.in_scope(|| {
            let expected_password_hash = PasswordHash::new(password_hash.expose_secret())?;
            
            Argon2::default()
                .verify_password(password.as_bytes(),&expected_password_hash)
                .wrap_err("Falha ao verificar senha crua")?;
            Ok(())
        })
    }

    #[tracing::instrument(name = "Computando hash da senha", skip_all)]
    pub async fn compute_password_hash(password: &SecretString) -> Result<SecretString> {
        let current_span: tracing::Span = tracing::Span::current();
        let password = password.expose_secret().to_owned();
        current_span.in_scope(|| {
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
            .hash_password(password.as_bytes(), &salt).map_err(|e| eyre!("Falha ao computar hash da senha: {}", e))?
            .to_string();
            Ok(SecretString::new(password_hash.into_boxed_str()))

        })
    }

}

impl AsRef<SecretString> for HashedPassword {
    fn as_ref(&self) -> &SecretString {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Gen;
    use rand::SeedableRng;
    use fake::Fake;
    use fake::faker::internet::en::Password as FakePassword;
    use secrecy::SecretString;

    #[tokio::test]
    async fn empty_string_is_rejected() {
        let password = SecretString::new("".to_string().into_boxed_str());
        let result = HashedPassword::parse(password).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn string_less_than_8_characters_is_rejected() {
        let password = SecretString::new("short".to_string().into_boxed_str());
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
        let hash_password = HashedPassword::parse_password_hash(
            SecretString::new(
                hash_string
                    .clone()
                    .into_boxed_str()
                )
        ).unwrap();
        assert_eq!(hash_password.as_ref().expose_secret(), hash_string.as_str());
        assert!(hash_password.as_ref().expose_secret().starts_with("$argon2id$v=19$"));
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub SecretString); // Updated!

    impl quickcheck::Arbitrary for ValidPasswordFixture {
      fn arbitrary(g: &mut Gen) -> Self {
       let seed: u64 = g.size() as u64;
       let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
      let password: String = FakePassword(8..30).fake_with_rng(&mut rng);
       Self(SecretString::new(password.into_boxed_str())) // Updated!
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        let result = tokio_test::block_on(HashedPassword::parse(valid_password.0.clone()));
        result.is_ok()
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
        let hash_password = HashedPassword::parse_password_hash(
            SecretString::new(hash_string.clone().into_boxed_str())
        ).unwrap();
        assert_eq!(hash_password.as_ref().expose_secret(), hash_string.as_str());
        assert!(hash_password.as_ref().expose_secret().starts_with("$argon2id$v=19$"));
        let result= hash_password.verify_raw_password(raw_password).await;
        assert!(result.is_ok());        
    }

}