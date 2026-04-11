//! AEAD encryption — ChaCha20Poly1305 for at-rest secret storage.
//!
//! Key derivation: HMAC-SHA256(master_secret, org_id) -> 32-byte key.
//! Ciphertext format: base64(nonce || ciphertext || tag).

use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Nonce};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::OnceLock;

type HmacSha256 = Hmac<Sha256>;

static MASTER_KEY: OnceLock<[u8; 32]> = OnceLock::new();

/// Initialize the master encryption key. Call once at startup.
/// If `key` is None, generates a random 32-byte key.
pub fn init_master_key(key: Option<&[u8; 32]>) {
    let k = match key {
        Some(k) => *k,
        None => {
            let mut buf = [0u8; 32];
            getrandom::getrandom(&mut buf).expect("RNG");
            buf
        }
    };
    let _ = MASTER_KEY.set(k);
}

fn master_key() -> &'static [u8; 32] {
    MASTER_KEY.get().expect("call init_master_key first")
}

/// Derive a per-org 32-byte key from the master key.
fn derive_org_key(org_id: &str) -> [u8; 32] {
    let mut mac = <HmacSha256 as Mac>::new_from_slice(master_key()).expect("HMAC key");
    mac.update(org_id.as_bytes());
    let result = mac.finalize().into_bytes();
    <[u8; 32]>::try_from(result.as_slice()).expect("HMAC-SHA256 output is 32 bytes")
}

/// Encrypt a plaintext string. Returns base64(nonce || ciphertext).
pub fn encrypt(org_id: &str, plaintext: &str) -> Result<String, AeadError> {
    let key = derive_org_key(org_id);
    let cipher = ChaCha20Poly1305::new_from_slice(&key).map_err(|_| AeadError::KeyInit)?;
    let mut nonce_bytes = [0u8; 12];
    getrandom::getrandom(&mut nonce_bytes).map_err(|_| AeadError::Rng)?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| AeadError::Encrypt)?;
    let mut blob = Vec::with_capacity(12 + ciphertext.len());
    blob.extend_from_slice(&nonce_bytes);
    blob.extend_from_slice(&ciphertext);
    Ok(B64.encode(&blob))
}

/// Decrypt a base64-encoded ciphertext. Returns plaintext string.
pub fn decrypt(org_id: &str, encoded: &str) -> Result<String, AeadError> {
    let blob = B64.decode(encoded).map_err(|_| AeadError::Decode)?;
    if blob.len() < 13 {
        return Err(AeadError::TooShort);
    }
    let (nonce_bytes, ciphertext) = blob.split_at(12);
    let key = derive_org_key(org_id);
    let cipher = ChaCha20Poly1305::new_from_slice(&key).map_err(|_| AeadError::KeyInit)?;
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| AeadError::Decrypt)?;
    String::from_utf8(plaintext).map_err(|_| AeadError::Utf8)
}

#[derive(Debug, thiserror::Error)]
pub enum AeadError {
    #[error("key init failed")]
    KeyInit,
    #[error("RNG failure")]
    Rng,
    #[error("encryption failed")]
    Encrypt,
    #[error("base64 decode failed")]
    Decode,
    #[error("ciphertext too short")]
    TooShort,
    #[error("decryption failed (wrong key or tampered data)")]
    Decrypt,
    #[error("decrypted data is not valid UTF-8")]
    Utf8,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        let s = format!("test-key-{}-bytes-for-aead-0{}", 32, "1234");
        let key: [u8; 32] = s.as_bytes()[..32]
            .try_into()
            .expect("test key must be 32 bytes");
        init_master_key(Some(&key));
    }

    #[test]
    fn roundtrip() {
        setup();
        let ct = encrypt("acme", "my-secret-value").unwrap();
        let pt = decrypt("acme", &ct).unwrap();
        assert_eq!(pt, "my-secret-value");
    }

    #[test]
    fn cross_org_decrypt_fails() {
        setup();
        let ct = encrypt("alpha", "secret-a").unwrap();
        assert!(decrypt("beta", &ct).is_err());
    }

    #[test]
    fn tampered_ciphertext_fails() {
        setup();
        let ct = encrypt("acme", "data").unwrap();
        let mut blob = B64.decode(&ct).unwrap();
        blob[15] ^= 0xFF;
        let tampered = B64.encode(&blob);
        assert!(decrypt("acme", &tampered).is_err());
    }

    #[test]
    fn too_short_fails() {
        setup();
        let short = B64.encode([0u8; 5]);
        assert!(decrypt("acme", &short).is_err());
    }
}
