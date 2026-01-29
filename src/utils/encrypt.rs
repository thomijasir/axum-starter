use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
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

// Simple version of hash_password
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

// The hash string they provided is:
// $argon2id$v=19$m=65536,t=3,p=4$qVVmcPK2fEduLBp5Ko7AxQ$NOhkuLMfj0vb3/1RKdsQhv6RV4AnhcR+r7KXd6IYZa4

// This is a PHC-formatted string that contains:

// The algorithm (argon2id)
// The version (v=19)
// Parameters (m=65536,t=3,p=4 - memory, iterations, parallelism)
// Salt (qVVmcPK2fEduLBp5Ko7AxQ)
// Hash (NOhkuLMfj0vb3/1RKdsQhv6RV4AnhcR+r7KXd6IYZa4)

// const ALGORITHM_INFO: &str = "$argon2id$v=19$m=65536,t=3,p=4$";

// Hash a password securely using Argon2id.
// Returns a PHC-formatted string containing the algorithm, version,
// parameters, salt, and hash - ready for storage.
// pub fn hash_password(password: &str) -> Result<String, PasswordError> {
//     // Generate a cryptographically secure random salt
//     let salt = SaltString::generate(&mut OsRng);
//
//     // Configure Argon2 with recommended settings for high-security applications
//     // These parameters may need adjustment based on your server capabilities
//     let params = Params::new(
//         64 * 1024, // 64MB memory cost
//         3,         // 3 iterations
//         4,         // 4 parallelism
//         Some(32),  // 32-byte output
//     )
//     .map_err(|e| PasswordError::HashingError(format!("Invalid parameters: {}", e)))?;
//
//     // Create an Argon2id instance with the specified parameters
//     let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
//
//     // Hash the password and return the PHC-formatted string
//     let created_hash = argon2
//         .hash_password(password.as_bytes(), &salt)
//         .map_err(|e| PasswordError::HashingError(format!("Hashing failed: {}", e)))?;
//
//     // parse the password hash
//     let (algorithm_info, password_hash) = password_extraction(created_hash.to_string().as_str())
//         .map_err(|e| PasswordError::HashingError(format!("Invalid hash format: {}", e)))?;
//
//     // reconstruct the password hash
//     if algorithm_info != ALGORITHM_INFO {
//         return Err(PasswordError::HashingError(format!(
//             "Invalid algorithm info: {}",
//             algorithm_info
//         )));
//     }
//
//     Ok(password_hash.to_string())
// }
//
// Verify a password against a stored hash.
// The hash parameter should be a PHC-formatted string as returned by hash_password.
// pub fn verify_password(
//     password: &str,
//     hash: &str,
// ) -> Result<bool, PasswordError> {
//     // reconstruct the password hash
//     let build_hash = password_reconstruction(ALGORITHM_INFO, hash).map_err(|e| {
//         PasswordError::VerificationError(format!("Invalid hash format build hash: {}", e))
//     })?;
//
//     // Parse the PHC-formatted hash string
//     let parsed_hash = PasswordHash::new(build_hash.as_str()).map_err(|e| {
//         PasswordError::VerificationError(format!("Invalid hash format parsed hash: {}", e))
//     })?;
//
//     // Create an Argon2id instance - parameters are extracted from the hash string
//     let argon2 = Argon2::default();
//
//     // Verify the password
//     match argon2.verify_password(password.as_bytes(), &parsed_hash) {
//         Ok(_) => Ok(true),
//         Err(err) => {
//             // Log the error type for monitoring but don't expose it to the caller
//             tracing::debug!("Password verification error: {}", err);
//             Ok(false)
//         }
//     }
// }

// pub fn password_extraction(hash_string: &str) -> Result<(String, String), &'static str> {
//     // Split the hash by the '$' character
//     let parts: Vec<&str> = hash_string.split('$').collect();
//
//     // Check if the format is valid
//     // We expect: ["", "argon2id", "v=19", "m=65536,t=3,p=4", "salt", "hash"]
//     if parts.len() < 6 {
//         return Err("Invalid Argon2id hash format");
//     }
//
//     // Reconstruct the algorithm info part (ending with a '$')
//     let algorithm_info = format!("${}${}${}$", parts[1], parts[2], parts[3]);
//
//     // The salt and hash are the remaining parts
//     // The salt is at index 4 and the hash at index 5
//     let salt_and_hash = format!("{}${}", parts[4], parts[5]);
//
//     Ok((algorithm_info, salt_and_hash))
// }

// pub fn password_reconstruction(
//     algorithm_info: &str,
//     salt_and_hash: &str,
// ) -> Result<String, &'static str> {
//     // Validate the algorithm info
//     if !algorithm_info.starts_with("$argon2") || !algorithm_info.ends_with("$") {
//         return Err("Invalid algorithm info format");
//     }
//
//     // Validate the salt and hash (basic check)
//     if !salt_and_hash.contains('$') {
//         return Err("Invalid salt and hash format");
//     }
//
//     // Combine the parts
//     let combined_hash = format!("{}{}", algorithm_info, salt_and_hash);
//
//     Ok(combined_hash)
// }

// New function to extract components
// pub fn extract_hash_components(phc_string: &str) -> Result<(String, String), PasswordError> {
//     let parsed_hash = PasswordHash::new(phc_string)
//         .map_err(|e| PasswordError::HashingError(format!("Invalid hash format: {}", e)))?;

//     // Extract salt and hash
//     let salt_b64 = parsed_hash
//         .salt
//         .ok_or_else(|| PasswordError::HashingError("Missing salt".to_string()))?
//         .as_str();

//     let hash_b64 = parsed_hash
//         .hash
//         .ok_or_else(|| PasswordError::HashingError("Missing hash".to_string()))?
//         .as_str();

//     Ok((salt_b64.to_string(), hash_b64.to_string()))
// }

// // New function to reconstruct PHC string
// pub fn reconstruct_phc_string(
//     salt: &str,
//     hash: &str,
// ) -> String {
//     format!("$argon2id$v=19$m=65536,t=3,p=4${}${}", salt, hash)
// }

// Add a unit test module to verify the implementation
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_password_hash_and_verify() {
//         let password = "secure_password_123";
//
//         // Hash the password
//         let hash = hash_password(password).expect("Failed to hash password");
//
//         // Verify the hash format (PHC string format)
//         assert!(hash.starts_with("$argon2id$"));
//
//         // Verify correct password
//         let result = verify_password(password, &hash).expect("Verification failed");
//         assert!(result);
//
//         // Verify incorrect password
//         let result = verify_password("wrong_password", &hash).expect("Verification failed");
//         assert!(!result);
//     }
//
//     #[test]
//     fn test_empty_password() {
//         // Empty passwords should still hash without errors
//         let password = "";
//         let hash = hash_password(password).expect("Failed to hash empty password");
//
//         // Verification should work
//         let result = verify_password(password, &hash).expect("Verification failed");
//         assert!(result);
//
//         // Non-empty password should not match empty password hash
//         let result = verify_password("not_empty", &hash).expect("Verification failed");
//         assert!(!result);
//     }
//
//     #[test]
//     fn test_long_password() {
//         // Test with a very long password (100 characters)
//         let password = "a".repeat(100);
//         let hash = hash_password(&password).expect("Failed to hash long password");
//
//         // Verification should work
//         let result = verify_password(&password, &hash).expect("Verification failed");
//         assert!(result);
//     }
//
//     #[test]
//     fn test_special_characters() {
//         // Test with special characters
//         let password = "p@$$w0rd!@#$%^&*()_+{}|:<>?~`-=[]\\;',./";
//         let hash = hash_password(password).expect("Failed to hash password with special chars");
//
//         // Verification should work
//         let result = verify_password(password, &hash).expect("Verification failed");
//         assert!(result);
//     }
//
//     #[test]
//     fn test_invalid_hash_format() {
//         // Test with an invalid hash format
//         let result = verify_password("password", "invalid_hash_format");
//         assert!(result.is_err());
//
//         // The error should be a VerificationError
//         match result {
//             Err(PasswordError::VerificationError(_)) => (), // Expected
//             _ => panic!("Expected VerificationError"),
//         }
//     }
// }
