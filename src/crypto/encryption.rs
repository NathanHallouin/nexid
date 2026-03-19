use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;

use crate::error::AppError;

/// AES-256-GCM encryptor for sensitive data at rest
#[derive(Clone)]
pub struct Encryptor {
    cipher: Aes256Gcm,
}

const NONCE_SIZE: usize = 12;

impl Encryptor {
    /// Create a new encryptor from a base64-encoded 32-byte key
    pub fn new(key_base64: &str) -> Result<Self, AppError> {
        let key_bytes = BASE64
            .decode(key_base64)
            .map_err(|e| AppError::Encryption(format!("Invalid base64 key: {}", e)))?;

        if key_bytes.len() != 32 {
            return Err(AppError::Encryption(format!(
                "Key must be 32 bytes, got {}",
                key_bytes.len()
            )));
        }

        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(|e| AppError::Encryption(format!("Failed to create cipher: {}", e)))?;

        Ok(Self { cipher })
    }

    /// Encrypt plaintext, returning base64-encoded ciphertext with prepended nonce
    pub fn encrypt(&self, plaintext: &str) -> Result<String, AppError> {
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| AppError::Encryption(format!("Encryption failed: {}", e)))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);

        Ok(BASE64.encode(result))
    }

    /// Decrypt base64-encoded ciphertext (with prepended nonce)
    pub fn decrypt(&self, ciphertext_base64: &str) -> Result<String, AppError> {
        let data = BASE64
            .decode(ciphertext_base64)
            .map_err(|e| AppError::Encryption(format!("Invalid base64 ciphertext: {}", e)))?;

        if data.len() < NONCE_SIZE {
            return Err(AppError::Encryption("Ciphertext too short".to_string()));
        }

        let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| AppError::Encryption(format!("Decryption failed: {}", e)))?;

        String::from_utf8(plaintext)
            .map_err(|e| AppError::Encryption(format!("Invalid UTF-8: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        // 32 bytes = 256 bits, base64 encoded
        let key = BASE64.encode([0u8; 32]);
        let encryptor = Encryptor::new(&key).unwrap();

        let plaintext = "Hello, World! 🌍";
        let ciphertext = encryptor.encrypt(plaintext).unwrap();
        let decrypted = encryptor.decrypt(&ciphertext).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_different_encryptions_produce_different_ciphertexts() {
        let key = BASE64.encode([0u8; 32]);
        let encryptor = Encryptor::new(&key).unwrap();

        let plaintext = "Same text";
        let ciphertext1 = encryptor.encrypt(plaintext).unwrap();
        let ciphertext2 = encryptor.encrypt(plaintext).unwrap();

        // Due to random nonce, ciphertexts should be different
        assert_ne!(ciphertext1, ciphertext2);
    }
}
