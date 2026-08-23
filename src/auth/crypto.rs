use rand::RngCore;
use sha2::{Digest, Sha256};

pub struct Crypto;

impl Crypto {
    /// Generate a secure random hexadecimal token
    pub fn random_token(length: usize) -> String {
        let mut bytes = vec![0u8; length];
        rand::thread_rng().fill_bytes(&mut bytes);
        hex::encode(bytes)
    }

    /// Hash a password using SHA-256 with random salt ($s2$salt$hash)
    pub fn hash_password(password: &str) -> String {
        let mut salt_bytes = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt_bytes);
        let salt = hex::encode(salt_bytes);

        let mut hasher = Sha256::new();
        hasher.update(salt.as_bytes());
        hasher.update(password.as_bytes());
        let hash = hex::encode(hasher.finalize());

        format!("$s2${}${}", salt, hash)
    }

    /// Verify plaintext password against stored hash in constant time to prevent timing attacks
    pub fn verify_password(password: &str, stored_hash: &str) -> bool {
        let parts: Vec<&str> = stored_hash.split('$').collect();
        if parts.len() != 4 || parts[1] != "s2" {
            return false;
        }

        let salt = parts[2];
        let expected_hash = parts[3];

        let mut hasher = Sha256::new();
        hasher.update(salt.as_bytes());
        hasher.update(password.as_bytes());
        let actual_hash = hex::encode(hasher.finalize());

        if actual_hash.len() != expected_hash.len() {
            return false;
        }

        // Constant-time byte comparison
        let mut diff = 0u8;
        for (a, b) in actual_hash.bytes().zip(expected_hash.bytes()) {
            diff |= a ^ b;
        }
        diff == 0
    }
}
