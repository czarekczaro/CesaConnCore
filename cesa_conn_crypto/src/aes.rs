use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::rngs::SysRng;

fn encrypt(key: &[u8; 32], data: &[u8]) {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    
}