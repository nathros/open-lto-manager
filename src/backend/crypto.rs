use balloon_hash::{Balloon, PasswordHasher};
use base64::{Engine, prelude::BASE64_STANDARD};
use sha2::Sha256;
use tracing::error;

use crate::backend::env::get_pepper;

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
pub fn generate_hash(password: &str, salt: &str, pepper: Option<String>) -> String {
    match BASE64_STANDARD.decode(salt) {
        Ok(salt_vec) => {
            let hash_result = if let Some(pepper) = pepper {
                Balloon::<Sha256>::default().hash_password_with_salt(
                    format!("{}{}", password, pepper).as_bytes(),
                    &salt_vec,
                )
            } else {
                Balloon::<Sha256>::default().hash_password_with_salt(password.as_bytes(), &salt_vec)
            };

            match hash_result {
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
    generate_hash(password, salt, get_pepper()) == hash
}

#[cfg(test)]
mod tests {
    use crate::backend::crypto::{generate_hash, validate_password};

    use super::generate_salt;

    #[test]
    fn test_hash_base() {
        let password = "hello";
        let salt = generate_salt();
        let hash = generate_hash(password, &salt, None);
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

    #[test]
    fn hashing_set_fixed() {
        // Check Hashing with fixed inputs

        // Balloon hash Sha256
        let balloon_sha256_data = [
            (
                "password",                                    // Password
                "OzAaKBHuxKcZNdVPbw8tWw==",                    // Salt
                "oQwXM+phlIwntj5Ux0TBZkp+z70i6GyEvzpzrihtHWo", // Hash without pepper
                "88805fac0b4bb5611ce5598a2efdbc16",            // Pepper
                "WEAa13sN99Kwbf0HThfQobZrVav2fKrKvhmr1+TbUak", // Hash with pepper
            ),
            (
                "w0ykHB:$:p}m=z-E",                            // Password
                "2HTA4x+sqd02TBzc2h1zIQ==",                    // Salt
                "hF4mQeobnbA7JNAIHEZRJKyN2s8cj0oPJVCukd7E3Yc", // Hash without pepper
                "90d8cdfad26e025bd977f53a8aa5fe51",            // Pepper
                "evH6SWBSvP7X8H3jkJ89vSDTgPiopvB6gL0sBRvgfSI", // Hash with pepper
            ),
            (
                "]z7#zW6!M2KpLXm=0W?m",                        // Password
                "Tjlo9pZ01KbFebhXwG/rtA==",                    // Salt
                "tEW47LtUZ11+Rp9gGMgcNSuZH2RUdwFHo1tvDwESRVk", // Hash without pepper
                "e3663f9008f88fba15884654f6ef8703",            // Pepper
                "eeb9MrO1Q96U0u8jVboSw+XTZLDobDItFLC060O3kFY", // Hash with pepper
            ),
        ];
        balloon_sha256_data.iter().for_each(
            |(password, salt, hash_no_pepper, pepper, hash_with_pepper)| {
                assert_eq!(generate_hash(password, salt, None), *hash_no_pepper);
                assert_eq!(
                    generate_hash(password, salt, Some(pepper.to_string())),
                    *hash_with_pepper
                );
            },
        );
    }
}
