use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ciphertext {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl Ciphertext {
    pub fn new(ciphertext: Vec<u8>, nonce: Vec<u8>) -> Self {
        Self { ciphertext, nonce }
    }

    pub fn get_ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }

    pub fn get_nonce(&self) -> &[u8] {
        &self.nonce
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CiphertextBuf {
    pub nonce: Vec<u8>,
    pub associated_data: Vec<u8>,
}

impl CiphertextBuf {
    pub fn new(nonce: Vec<u8>, associated_data: Vec<u8>) -> Self {
        Self {
            nonce,
            associated_data,
        }
    }

    pub fn get_nonce(&self) -> &[u8] {
        &self.nonce
    }

    pub fn get_associated_data(&self) -> &[u8] {
        &self.associated_data
    }
}
