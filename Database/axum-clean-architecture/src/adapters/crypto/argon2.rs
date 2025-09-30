use crate::app_error::{AppError, AppResult};
use crate::use_cases::user::UserCredentialsHasher;
use argon2::Argon2;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHasher, SaltString};

#[derive(Default)]
pub struct ArgonPasswordHasher {
    hasher: Argon2<'static>,
}

impl UserCredentialsHasher for ArgonPasswordHasher {
    fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .hasher
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::Internal("Password hashing failed.".into()))?
            .to_string();

        Ok(hash)
    }
}
