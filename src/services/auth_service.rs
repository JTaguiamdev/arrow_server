use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};

struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        AuthService
    }

    pub async fn hash_password(&self, password: &str) -> Result<String, AuthServiceError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => Ok(hash.to_string()),
            Err(_) => Err(AuthServiceError::HashingError),
        }
    }

    pub async fn verify_password(
        &self,
        password: &str,
        hash: &str,
    ) -> Result<bool, AuthServiceError> {
        let parsed_hash = match argon2::password_hash::PasswordHash::new(hash) {
            Ok(h) => h,
            Err(_) => return Err(AuthServiceError::VerificationError),
        };

        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(_) => Err(AuthServiceError::VerificationError),
        }
    }
}

pub enum AuthServiceError {
    HashingError,
    VerificationError,
}

impl std::fmt::Display for AuthServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthServiceError::HashingError => write!(f, "Password hashing failed"),
            AuthServiceError::VerificationError => write!(f, "Password verification failed"),
        }
    }
}
