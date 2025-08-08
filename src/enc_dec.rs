use std::fs;

use argon2::{self, password_hash::{self, rand_core::RngCore}, Argon2};
use chacha20poly1305::{aead::{self, Aead}, ChaCha20Poly1305, KeyInit, Nonce};

pub struct FileEncDecrpytor {
    password: String
}


impl FileEncDecrpytor {
    pub fn new(password: String) -> Self {
        Self {password}
    }
    
    pub fn encrpt_file(&self, file_path: &str, save_as: &str) {
        let file_data = fs::read(file_path).unwrap();
        let ciphertext = encrypt(file_data, &self.password).expect("failed to encrpt file");
        fs::write(format!("{save_as}"), ciphertext).expect("failed to create encryption file");
    }

    pub fn decrpt_file(&self, file_path: &str, save_as: &str) {
        let file_data = fs::read(file_path).unwrap();
        let plaintext = decrypt(file_data, &self.password).expect("failed to decrypt file");
        fs::write(save_as, plaintext).expect("failed to create decryption file");
    }
}

/// Derives a 32-byte key from a password and salt using Argon2
fn derive_key(password: &str, salt: &password_hash::SaltString) -> [u8; 32] {
    let mut key = [0u8; 32];
    Argon2::default().hash_password_into(
        password.as_bytes(), 
        salt.as_str().as_bytes(), 
        &mut key
    ).expect("failed to hash password");
    key
}

fn encrypt(file_data: Vec<u8>, password: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Generate random salt and nonce
    let salt = password_hash::SaltString::generate(&mut aead::OsRng);
    let nonce_bytes = {
        let mut nonce = [0u8; 12];
        aead::OsRng.fill_bytes(&mut nonce);
        nonce
    };

    let key = derive_key(password, &salt);
    let cipher = ChaCha20Poly1305::new(&key.into());
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, file_data.as_ref()).unwrap();

    // File format: [salt (16)] + [nonce (12)] + [ciphertext]
    let salt_b64 = salt.as_str();
    let salt_len = salt_b64.len() as u8;
    let mut output = vec![salt_len];       
    output.extend_from_slice(salt_b64.as_bytes()); 
    output.extend_from_slice(&nonce_bytes); 
    output.extend_from_slice(&ciphertext);

    Ok(output)
}

fn decrypt(encrypted: Vec<u8>, password: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (salt_bytes, rest) = encrypted.split_at(16);
    let (nonce_bytes, ciphertext) = rest.split_at(12);

    let salt_str = std::str::from_utf8(salt_bytes).unwrap();
    let salt = password_hash::SaltString::from_b64(salt_str).expect("failed to get salt string");

    let key = derive_key(password, &salt);
    let cipher = ChaCha20Poly1305::new(&key.into());
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext).expect("failed to decrpty ciphertext");
    Ok(plaintext)
}
