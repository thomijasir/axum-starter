use argon2::{
  Argon2,
  password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use std::fmt;

/// Custom error type for password hashing operations
#[derive(Debug)]
pub enum PasswordError {
  HashingError(String),
  VerificationError(String),
  HashingFailed,
  HashingInvalid,
}

impl fmt::Display for PasswordError {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>,
  ) -> fmt::Result {
    match self {
      PasswordError::HashingInvalid => write!(f, "PASSWORD_HASHING_INVALID"),
      PasswordError::HashingFailed => write!(f, "PASSWORD_HASHING_FAILED"),
      PasswordError::HashingError(msg) => write!(f, "PASSWORD_HASHING_ERROR: {}", msg),
      PasswordError::VerificationError(msg) => {
        write!(f, "PASSWORD_VERIFICATION_ERROR: {}", msg)
      }
    }
  }
}

impl std::error::Error for PasswordError {}

pub fn hash(password: &str) -> Result<String, PasswordError> {
  let salt = SaltString::generate(&mut OsRng);
  let hashed_password = Argon2::default()
    .hash_password(password.as_bytes(), &salt)
    .map_err(|_| PasswordError::HashingFailed)?
    .to_string();
  Ok(hashed_password)
}

pub fn verify(
  password: &str,
  hashed_password: &str,
) -> Result<bool, PasswordError> {
  let parsed_hash =
    PasswordHash::new(hashed_password).map_err(|_| PasswordError::HashingInvalid)?;
  let password_matches = Argon2::default()
    .verify_password(password.as_bytes(), &parsed_hash)
    .is_ok();
  Ok(password_matches)
}
