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

    /// Delete file at path.
    pub fn remove<P>(path: &P) -> anyhow::Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        fs::remove_file(path)?;

        Ok(())
    }

    /// Delete database save file.
    pub fn delete(self) -> anyhow::Result<()> {
        Self::remove(&self.path)?;

        Ok(())
    }

    /// Encrypt and save data to file.
    pub fn save(&self, key: &[u8]) -> anyhow::Result<()> {
        fs::write(&self.path, Self::serialize(&self.data, key)?)?;

        Ok(())
    }

    /// Encrypt and save file before closing.
    pub fn close(self, key: &[u8]) -> anyhow::Result<()> {
        self.save(key)
    }

    /// Mutable reference to inner data
    pub fn mutate(&mut self) -> &mut D {
        &mut self.data
    }

    /// Serialize data to bytes, encrypt to [`Ciphertext`] and serialize to bytes.
    fn serialize(data: &D, key: &[u8]) -> anyhow::Result<Vec<u8>> {
        let data = bincode::serialize(data)?;
        let ciphertext = XChaCha20Poly1305Algorithm::encrypt(&data, key)?;

        Ok(bincode::serialize(&ciphertext)?)
    }

    /// Deserialize [`Ciphertext`] bytes, decrypt to D bytes and deserialize to D.
    fn deserialize(ciphertext_bytes: Vec<u8>, key: &[u8]) -> anyhow::Result<D> {
        let ciphertext: Ciphertext = bincode::deserialize(&ciphertext_bytes)?;
        let data = XChaCha20Poly1305Algorithm::decrypt(key, ciphertext)?;
        Ok(bincode::deserialize(&data)?)
    }
}

mod database_test {
    #[allow(unused)]
    use std::path::PathBuf;

    use cryptography::argon2::{Argon2Hasher, SaltString};
    use serde::{Deserialize, Serialize};

    #[allow(unused)]
    use crate::DB;

    #[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
    struct TestData {
        one: String,
        two: u8,
    }

    #[allow(unused)]
    fn generate_key(salt: &SaltString) -> [u8; 32] {
        Argon2Hasher::key_derivation(b"bad password123", salt).unwrap()
    }

    #[allow(unused)]
    fn generate_salt() -> SaltString {
        Argon2Hasher::generate_salt()
    }

    #[test]
    fn create_test() {
        let salt = generate_salt();
        let key = generate_key(&salt);
        let path = PathBuf::from("./mydb.db");

        let test_data = TestData {
            one: String::from("Test_string"), // Mutate field one of TestData to new String
            two: 69, // Mutate field two of TestData to random number I picked.
        };

        let mut db = DB::<TestData>::create(&path, &key).unwrap();
        db.data = test_data.clone();
        db.close(&key).unwrap();

        let db = DB::<TestData>::open(&path, &key).unwrap();
        assert_eq!(&test_data, &db.data);
        db.save(&key).unwrap();

        db.delete().unwrap();
    }
}
