use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

pub struct DB<D>
where
    D: Default + Serialize + for<'de> Deserialize<'de>,
{
    path: PathBuf,
    data: D,
}

impl<D> DB<D>
where
    D: Default + Serialize + for<'de> Deserialize<'de>,
{
    /// Opens the file at path.
    /// If the file does not exist create one.
    pub fn open<P>(path: &P, password_key: &[u8]) -> anyhow::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        // TODO: Add decryption
        match fs::read(&path) {
            Ok(data) => Ok(Self {
                path: path.as_ref().into(),
                data: bincode::deserialize(&data)?,
            }),
            Err(_) => Self::create(&path),
        }
    }

    /// Create a new file. If a file exists, it will replace its contents.
    pub fn create<P>(path: &P) -> anyhow::Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let data = D::default();
        fs::write(&path, bincode::serialize(&data)?)?;

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
    pub fn save(&self, password_key: &[u8]) -> anyhow::Result<()> {
        let data = bincode::serialize(&self.data)?;
        fs::write(&self.path, data)?;

        Ok(())
    }

    pub fn close(self, password_key: &[u8]) -> anyhow::Result<()> {
        self.save(password_key)
    }

    pub fn modify(&mut self) -> &mut D {
        &mut self.data
    }
}
