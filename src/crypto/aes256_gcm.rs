use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};

pub fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    let encrypting_key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(encrypting_key);
    let nonce = Nonce::from_slice(iv);
    let encrypted_data = cipher
        .encrypt(nonce, data.as_ref())
        .map_err(|e| format!("Encrypt data failed: {:?}", e).to_string());

    encrypted_data
}

pub fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    let decrypting_key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(decrypting_key);
    let nonce = Nonce::from_slice(iv);
    let plain_text = cipher
        .decrypt(nonce, encrypted_data.as_ref())
        .map_err(|e| format!("Decrypt data failed: {:?}", e).to_string());

    plain_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_and_decrypt() {
        let key = b"01234567890123456789012345678901";
        let iv = b"012345678901";
        let data = b"test_data";

        let encrypted_data = encrypt(data, key, iv);
        assert!(encrypted_data.is_ok(), "encrypt() failed");

        let decrypted_data = decrypt(&encrypted_data.unwrap(), key, iv);
        assert!(decrypted_data.is_ok(), "decrypt() failed");
    }
}
