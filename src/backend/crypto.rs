use balloon_hash::{Balloon, PasswordHasher};
use base64::{Engine, prelude::BASE64_STANDARD};
use sha2::Sha256;
use tracing::error;

pub fn generate_salt() -> String {
    match password_hash::try_generate_salt() {
        Ok(salt) => BASE64_STANDARD.encode(salt),
        Err(e) => {
            error!("Generate salt {}", e);
            unreachable!("Generate salt {}", e);
        }
    }
}

// In future may need to perform check on older hash algorithm
// 1: Before User login check algo version
// 2: If old, perform hash against old hash function
// 3: If pass upgrade to latest hash function, hash and salt
// Keep old crate: https://users.rust-lang.org/t/how-can-i-use-two-versions-of-the-same-crate/100831/8
pub fn generate_hash(password: &str, salt: &str) -> String {
    match BASE64_STANDARD.decode(salt) {
        Ok(salt_vec) => {
            match Balloon::<Sha256>::default()
                .hash_password_with_salt(password.as_bytes(), &salt_vec)
            {
                //PasswordHasher::
                Ok(ph) => {
                    return ph.hash.expect("PasswordHash should exist").to_string(); // Success
                }
                Err(e) => error!("Failed to hash password with salt: {}", e),
            }
        }
        Err(e) => {
            error!("Failed to decode salt: {}", e);
        }
    }
    Default::default() // Failure
}

pub fn validate_password(password: &str, salt: &str, hash: &str) -> bool {
    generate_hash(password, salt) == hash
}

#[cfg(test)]
mod tests {
    use crate::backend::crypto::{generate_hash, validate_password};

    use super::generate_salt;

    #[test]
    fn test_hash_base() {
        let password = "hello";
        let salt = generate_salt();
        let hash = generate_hash(password, &salt);
        assert!(validate_password(password, &salt, &hash)); // Good password and hash
        assert!(!validate_password("bad_password", &salt, &hash)); // Bad password
        assert!(!validate_password(password, "bad_salt", &hash)); // Bad salt
        assert!(!validate_password(password, &salt, "bad_hash")); // Bad hash
    }

    #[test]
    fn test_hash_ballon_sha256() {
        let hash = "5Bs7CIYFk5MJcx+0FVeTMytXlpZfHMOxl/Y86G0JVf4".to_string();
        let salt = "E7lOMmq1AZoGiobh3HHrLw==".to_string();
        let password = "admin";
        assert!(validate_password(password, &salt, &hash)); // Good password and hash
        assert!(!validate_password("bad_password", &salt, &hash)); // Bad password
        assert!(!validate_password(password, "bad_salt", &hash)); // Bad salt
        assert!(!validate_password(password, &salt, "bad_hash")); // Bad hash
    }
}
