use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

pub struct XChaCha20Poly1305Algorithm;

impl XChaCha20Poly1305Algorithm {
    pub fn encrypt(key: &[u8]) {
        let cipher = XChaCha20Poly1305::new(key);
    }

    pub fn decrypt() {}
}
