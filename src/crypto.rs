use ring::aead::{Aad, Nonce, UnboundKey, LessSafeKey, AES_256_GCM, NONCE_LEN};
use ring::rand::{SecureRandom, SystemRandom};
use ring::digest::{SHA256, Context};

pub fn derive_key(api_key: &str) -> [u8; 32] {
    let mut hasher = Context::new(&SHA256);
    hasher.update(api_key.as_bytes());
    let digest = hasher.finish();
    let mut key = [0u8; 32];
    key.copy_from_slice(digest.as_ref());
    key
}

pub fn encrypt_data(data: &str, api_key: &str) -> Result<Vec<u8>, String> {
    let key_bytes = derive_key(api_key);
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|e| format!("Failed to create key: {}", e))?;
    let less_safe_key = LessSafeKey::new(unbound_key);

    let rand = SystemRandom::new();
    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand.fill(&mut nonce_bytes)
        .map_err(|e| format!("Failed to generate nonce: {}", e))?;
    let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes)
        .map_err(|e| format!("Invalid nonce: {}", e))?;

    let mut in_out = data.as_bytes().to_vec();
    let aad = Aad::from(b"DonutDB odb");
    less_safe_key.seal_in_place_append_tag(nonce, aad, &mut in_out)
        .map_err(|e| format!("Encryption failed: {}", e))?;

    let mut output = nonce_bytes.to_vec();
    output.extend_from_slice(&in_out);
    Ok(output)
}

pub fn decrypt_data(encrypted_data: &[u8], api_key: &str) -> Result<String, String> {
    if encrypted_data.len() < NONCE_LEN {
        return Err("Invalid file format".to_string());
    }

    let key_bytes = derive_key(api_key);
    let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
        .map_err(|e| format!("Failed to create key: {}", e))?;
    let less_safe_key = LessSafeKey::new(unbound_key);

    let (nonce_bytes, ciphertext) = encrypted_data.split_at(NONCE_LEN);
    let nonce = Nonce::try_assume_unique_for_key(nonce_bytes)
        .map_err(|e| format!("Invalid nonce: {}", e))?;

    let mut in_out = ciphertext.to_vec();
    let aad = Aad::from(b"DonutDB odb");
    let decrypted = less_safe_key
        .open_in_place(nonce, aad, &mut in_out)
        .map_err(|e| format!("Decryption failed: {}", e))?;
    String::from_utf8(decrypted.to_vec())
        .map_err(|e| format!("Invalid UTF-8: {}", e))
}