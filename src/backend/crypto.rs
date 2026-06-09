use balloon_hash::{Balloon, password_hash::SaltString};
use base64::{Engine, prelude::BASE64_STANDARD};
use rand_core::OsRng;
use sha2::Sha256;

pub fn generate_salt() -> String {
    SaltString::generate(&mut OsRng).to_string()
}

pub fn generate_hash(password: &str, salt: &str) -> String {
    let balloon = Balloon::<Sha256>::default();

    if let Ok(result) = balloon.hash(password.as_bytes(), salt.as_bytes()) {
        return BASE64_STANDARD.encode(result);
    }
    unreachable!("Failed to generate hash");
}

pub fn validate_password(password: &str, salt: &str, hash: &str) -> bool {
    generate_hash(password, salt) == hash
}

#[cfg(test)]
mod tests {
    use crate::backend::crypto::{generate_hash, validate_password};

    use super::generate_salt;

    #[test]
    fn test_hash() {
        let password = "hello";
        let salt = generate_salt();
        let hash = generate_hash(password, &salt);
        assert!(validate_password(password, &salt, &hash)); // Good password and hash
        assert!(!validate_password("bad_password", &salt, &hash)); // Bad password
        assert!(!validate_password(password, "bad_salt", &hash)); // Bad salt
        assert!(!validate_password(password, &salt, "bad_hash")); // Bad hash
    }
}
