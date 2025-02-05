use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use cryptography::{ciphertext::Ciphertext, xchacha20poly1305::XChaCha20Poly1305Algorithm};

pub struct DB<D>
where
    D: Default + Serialize + for<'de> Deserialize<'de>,
{
    path: PathBuf,
    pub data: D,
}

impl<D> DB<D>
where
    D: Default + Serialize + for<'de> Deserialize<'de>,
{
    /// Opens the file at path.
    /// If the file does not exist create one.
    pub fn open<P>(path: &P, key: &[u8]) -> anyhow::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        match fs::read(&path) {
            Ok(ciphertext_bytes) => Ok({
                Self {
                    path: path.as_ref().into(),
                    data: Self::deserialize(ciphertext_bytes, key)?,
                }
            }),
            Err(_) => Self::create(&path, key),
        }
    }

    /// Create a new file. If a file exists, it will replace its contents.
    pub fn create<P>(path: &P, key: &[u8]) -> anyhow::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let data = D::default();
        fs::write(&path, Self::serialize(&data, key)?)?;

        Ok(Self {
            path: path.as_ref().into(),
            data,
        })
    }

    /// Delete the file at the path.
    pub fn delete<P>(path: &P) -> anyhow::Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        fs::remove_file(path)?;

        Ok(())
    }

    /// Encrypt and save data to file.
    pub fn save(&self, key: &[u8]) -> anyhow::Result<()> {
        fs::write(&self.path, Self::serialize(&self.data, key)?)?;

        Ok(())
    }

    pub fn close(self, password_key: &[u8]) -> anyhow::Result<()> {
        self.save(password_key)
    }

    pub fn mutate(&mut self) -> &mut D {
        &mut self.data
    }

    fn serialize(data: &D, key: &[u8]) -> anyhow::Result<Vec<u8>> {
        let data = bincode::serialize(data)?;
        let ciphertext = XChaCha20Poly1305Algorithm::encrypt(&data, key)?;

        Ok(bincode::serialize(&ciphertext)?)
    }

    fn deserialize(ciphertext_bytes: Vec<u8>, key: &[u8]) -> anyhow::Result<D> {
        let ciphertext: Ciphertext = bincode::deserialize(&ciphertext_bytes)?;
        let data = XChaCha20Poly1305Algorithm::decrypt(key, ciphertext)?;
        Ok(bincode::deserialize(&data)?)
    }
}
