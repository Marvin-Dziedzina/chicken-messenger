use anyhow::anyhow;
use chacha20poly1305::{
    aead::{Aead, AeadMutInPlace, Buffer, Nonce as XChaCha20Poly1305Nonce, OsRng},
    AeadCore, KeyInit, XChaCha20Poly1305,
};

use crate::ciphertext::{Ciphertext, CiphertextBuf};

pub struct XChaCha20Poly1305Algorithm;

impl XChaCha20Poly1305Algorithm {
    /// Encrypt data with key.
    ///
    /// The [`Nonce`] is needed to decrypt the data.
    /// It needs to be stored everytime data is encrypted to ensure safety and that data can be decrypted with [`XChaCha20Poly1305Algorithm::decrypt()`].
    pub fn encrypt(data: &[u8], key: &[u8]) -> anyhow::Result<Ciphertext> {
        let cipher = XChaCha20Poly1305::new(key.into());
        let nonce = Self::generate_nonce();
        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|e| anyhow!("Failed to encrypt data: {}", e))?;

        Ok(Ciphertext::new(ciphertext, nonce.to_vec()))
    }

    /// Decrypt a ciphertext.
    ///
    /// Always use the [`Nonce`] provided while encrypting with [`XChaCha20Poly1305Algorithm::encrypt()`].
    /// If the current [`Nonce`] is lost, all data lost.
    ///
    /// # Errors
    ///
    /// This function will fail if the wrong key or [`Nonce`] are supplied.
    pub fn decrypt(key: &[u8], ciphertext: Ciphertext) -> anyhow::Result<Vec<u8>> {
        let cipher = XChaCha20Poly1305::new(key.into());
        cipher
            .decrypt(ciphertext.get_nonce().into(), ciphertext.get_ciphertext())
            .map_err(|e| anyhow!("Failed to decrypt ciphertext: {}", e))
    }

    /// Encrypt a [`Vec<u8>`] in place.
    ///
    /// Associated_data is some additional data.
    /// Associated_data will stay in plain bytes but [`XChaCha20Poly1305Algorithm::decrypt_buf()`] will fail if it was tampered with.
    pub fn encrypt_buf(
        buf: &mut impl Buffer,
        key: &[u8],
        associated_data: Option<&[u8]>,
    ) -> anyhow::Result<CiphertextBuf> {
        let associated_data = match associated_data {
            Some(associated_data) => associated_data,
            None => b"",
        };

        let mut cipher = XChaCha20Poly1305::new(key.into());
        let nonce = Self::generate_nonce();

        cipher
            .encrypt_in_place(&nonce, &associated_data, buf)
            .map_err(|e| anyhow!("Failed to encrypt buffer: {}", e))?;

        Ok(CiphertextBuf::new(nonce.to_vec(), associated_data.to_vec()))
    }

    /// Decrypt a [`Vec<u8>`] in place.
    ///
    /// Associated_data is some additional data.
    /// It is the same data that was used to encrypt the buf with [`XChaCha20Poly1305Algorithm::encrypt_buf()`].
    ///
    /// # Errors
    ///
    /// This function will fail if the wrong key, [`Nonce`] or associated_data are supplied.
    pub fn decrypt_buf(
        buf: &mut Vec<u8>,
        key: &[u8],
        ciphertext_buf: CiphertextBuf,
    ) -> anyhow::Result<()> {
        let mut cipher = XChaCha20Poly1305::new(key.into());

        cipher
            .decrypt_in_place(
                ciphertext_buf.get_nonce().into(),
                ciphertext_buf.get_associated_data(),
                buf,
            )
            .map_err(|e| anyhow!("Failed to decrypt buffer: {}", e))?;

        Ok(())
    }

    fn generate_nonce() -> XChaCha20Poly1305Nonce<XChaCha20Poly1305> {
        XChaCha20Poly1305::generate_nonce(&mut OsRng)
    }
}

mod xchacha20poly1305_test {
    use chacha20poly1305::{aead::OsRng, KeyInit, XChaCha20Poly1305};

    #[allow(unused)]
    use super::XChaCha20Poly1305Algorithm;

    #[allow(unused)]
    const DATA: &[u8] = b"Test Data 123";
    #[allow(unused)]
    fn gen_key() -> Vec<u8> {
        XChaCha20Poly1305::generate_key(&mut OsRng).to_vec()
    }

    #[test]
    fn encrypt_decrypt_test() {
        let key = gen_key();

        let ciphertext = XChaCha20Poly1305Algorithm::encrypt(DATA, &key).unwrap();

        let decrypted_data = XChaCha20Poly1305Algorithm::decrypt(&key, ciphertext).unwrap();

        assert_eq!(DATA, decrypted_data);
    }

    #[test]
    fn encrypt_decrypt_fail_test() {
        let ciphertext = XChaCha20Poly1305Algorithm::encrypt(DATA, &gen_key()).unwrap();

        assert!(XChaCha20Poly1305Algorithm::decrypt(&gen_key(), ciphertext).is_err());
    }

    #[test]
    fn encrypt_decrypt_buffer_test() {
        let mut buf = DATA.to_vec();
        let key = gen_key();

        let ciphertext_buf = XChaCha20Poly1305Algorithm::encrypt_buf(&mut buf, &key, None).unwrap();

        XChaCha20Poly1305Algorithm::decrypt_buf(&mut buf, &key, ciphertext_buf).unwrap();

        assert_eq!(DATA, buf);
    }

    #[test]
    fn encrypt_decrypt_buffer_fail_key_test() {
        let mut buf = DATA.to_vec();

        let ciphertext_buf =
            XChaCha20Poly1305Algorithm::encrypt_buf(&mut buf, &gen_key(), None).unwrap();

        assert!(
            XChaCha20Poly1305Algorithm::decrypt_buf(&mut buf, &gen_key(), ciphertext_buf).is_err()
        );
    }

    #[test]
    fn encrypt_decrypt_buffer_with_associated_data_test() {
        let mut buf = DATA.to_vec();
        let key = gen_key();
        let associated_data = b"Test associated data!!! WARNING: THIS TEXT WILL BE UNENCRYPTED AND IS NEEDED FOR DECRYPTION!!!";

        let ciphertext_buf =
            XChaCha20Poly1305Algorithm::encrypt_buf(&mut buf, &key, Some(associated_data)).unwrap();

        XChaCha20Poly1305Algorithm::decrypt_buf(&mut buf, &key, ciphertext_buf).unwrap();

        assert_eq!(DATA, buf);
    }

    #[test]
    fn encrypt_decrypt_buffer_with_associated_data_fail_key_test() {
        let mut buf = DATA.to_vec();
        let associated_data = b"Test associated data!!! WARNING: THIS TEXT WILL BE UNENCRYPTED AND IS NEEDED FOR DECRYPTION!!!";

        let ciphertext_buf =
            XChaCha20Poly1305Algorithm::encrypt_buf(&mut buf, &gen_key(), Some(associated_data))
                .unwrap();

        assert!(
            XChaCha20Poly1305Algorithm::decrypt_buf(&mut buf, &gen_key(), ciphertext_buf,).is_err()
        );
    }

    #[test]
    fn encrypt_decrypt_buffer_with_associated_data_fail_associated_data_test() {
        let mut buf = DATA.to_vec();
        let associated_data_1 = b"Test associated data!!! WARNING: THIS TEXT WILL BE UNENCRYPTED AND IS NEEDED FOR DECRYPTION!!!1";
        let associated_data_2 = b"Test associated data!!! WARNING: THIS TEXT WILL BE UNENCRYPTED AND IS NEEDED FOR DECRYPTION!!!2";

        let ciphertext_buf =
            XChaCha20Poly1305Algorithm::encrypt_buf(&mut buf, &gen_key(), Some(associated_data_1))
                .unwrap();

        let mut wrong_ciphertext_buf = ciphertext_buf;
        wrong_ciphertext_buf.associated_data = associated_data_2.to_vec();

        assert!(XChaCha20Poly1305Algorithm::decrypt_buf(
            &mut buf,
            &gen_key(),
            wrong_ciphertext_buf,
        )
        .is_err());
    }
}
