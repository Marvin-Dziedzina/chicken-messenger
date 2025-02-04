use std::fmt;

use anyhow::anyhow;
use argon2::{
    password_hash::rand_core::OsRng as Argon2OsRng, Argon2, PasswordHasher, PasswordVerifier,
};

pub use argon2::password_hash::{PasswordHashString, SaltString};

pub struct Argon2Hasher;

impl Argon2Hasher {
    /// Hash a password.
    ///
    /// Do not use for cryptographic key derivation use [`Self::key_derivation()`] instead!
    pub fn hash_password(password: &[u8]) -> anyhow::Result<PasswordHashString> {
        let argon2 = Argon2::default();

        let salt = Self::generate_salt();
        let password_hash = argon2
            .hash_password(&password, &salt)
            .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

        Ok(password_hash.serialize())
    }

    /// Verify a password with a hashed password.
    pub fn verify_password(
        password: &[u8],
        password_hash: &PasswordHashString,
    ) -> anyhow::Result<()> {
        Argon2::default()
            .verify_password(&password, &password_hash.password_hash())
            .map_err(|e| anyhow!("Failed to verify passwords: {}", e))?;

        Ok(())
    }
}

impl Argon2Hasher {
    /// Securely generate salt.
    pub fn generate_salt() -> SaltString {
        SaltString::generate(&mut Argon2OsRng)
    }

    /// Get a 32 byte long cryptographic key from a password and salt.
    /// The same salt needs to
    pub fn key_derivation(password: &[u8], salt_string: &SaltString) -> anyhow::Result<[u8; 32]> {
        let salt = salt_string.as_str().as_bytes();

        let mut output_key = [0u8; 32];
        Argon2::default()
            .hash_password_into(&password, salt, &mut output_key)
            .map_err(|e| anyhow!("Failed to hash password: {}", e))?;

        Ok(output_key)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// Associated data is too long.
    AdTooLong,

    /// Algorithm identifier invalid.
    AlgorithmInvalid,

    /// "B64" encoding is invalid.
    B64Encoding(base64ct::Error),

    /// Key ID is too long.
    KeyIdTooLong,

    /// Memory cost is too small.
    MemoryTooLittle,

    /// Memory cost is too large.
    MemoryTooMuch,

    /// Output is too short.
    OutputTooShort,

    /// Output is too long.
    OutputTooLong,

    /// Password is too long.
    PwdTooLong,

    /// Salt is too short.
    SaltTooShort,

    /// Salt is too long.
    SaltTooLong,

    /// Secret is too long.
    SecretTooLong,

    /// Not enough threads.
    ThreadsTooFew,

    /// Too many threads.
    ThreadsTooMany,

    /// Time cost is too small.
    TimeTooSmall,

    /// Invalid version
    VersionInvalid,
}
impl From<argon2::Error> for Error {
    fn from(error: argon2::Error) -> Self {
        match error {
            argon2::Error::AdTooLong => Self::AdTooLong,
            argon2::Error::AlgorithmInvalid => Self::AlgorithmInvalid,
            argon2::Error::B64Encoding(error) => Self::B64Encoding(error),
            argon2::Error::KeyIdTooLong => Self::KeyIdTooLong,
            argon2::Error::MemoryTooLittle => Self::MemoryTooLittle,
            argon2::Error::MemoryTooMuch => Self::MemoryTooMuch,
            argon2::Error::OutputTooShort => Self::OutputTooShort,
            argon2::Error::OutputTooLong => Self::OutputTooLong,
            argon2::Error::PwdTooLong => Self::PwdTooLong,
            argon2::Error::SaltTooShort => Self::SaltTooShort,
            argon2::Error::SaltTooLong => Self::SaltTooLong,
            argon2::Error::SecretTooLong => Self::SecretTooLong,
            argon2::Error::ThreadsTooFew => Self::ThreadsTooFew,
            argon2::Error::ThreadsTooMany => Self::ThreadsTooMany,
            argon2::Error::TimeTooSmall => Self::TimeTooSmall,
            argon2::Error::VersionInvalid => Self::VersionInvalid,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::AdTooLong => "associated data is too long",
            Self::AlgorithmInvalid => "algorithm identifier invalid",
            Self::B64Encoding(inner) => return write!(f, "B64 encoding invalid: {inner}"),
            Self::KeyIdTooLong => "key ID is too long",
            Self::MemoryTooLittle => "memory cost is too small",
            Self::MemoryTooMuch => "memory cost is too large",
            Self::OutputTooShort => "output is too short",
            Self::OutputTooLong => "output is too long",
            Self::PwdTooLong => "password is too long",
            Self::SaltTooShort => "salt is too short",
            Self::SaltTooLong => "salt is too long",
            Self::SecretTooLong => "secret is too long",
            Self::ThreadsTooFew => "not enough threads",
            Self::ThreadsTooMany => "too many threads",
            Self::TimeTooSmall => "time cost is too small",
            Self::VersionInvalid => "invalid version",
        })
    }
}

impl std::error::Error for Error {}

mod argon2_test {
    #[allow(unused_imports)]
    use super::Argon2Hasher;

    #[allow(dead_code)]
    const PASSWORD: &[u8; 32] = b"TestPasswordForVerification12345";

    #[test]
    fn same_password_hash() {
        let password_hash = Argon2Hasher::hash_password(PASSWORD).unwrap();

        assert!(Argon2Hasher::verify_password(PASSWORD, &password_hash).is_ok());
    }

    #[test]
    fn different_password_hash() {
        let other_password =
            Argon2Hasher::hash_password("TestOtherPassword67890".as_bytes()).unwrap();

        assert!(Argon2Hasher::verify_password(PASSWORD, &other_password).is_err());
    }

    #[test]
    fn same_password_key_derivation() {
        let salt = Argon2Hasher::generate_salt();

        let key_1 = Argon2Hasher::key_derivation(PASSWORD, &salt).unwrap();
        let key_2 = Argon2Hasher::key_derivation(PASSWORD, &salt).unwrap();

        assert_eq!(key_1, key_2);
    }

    #[test]
    fn different_password_key_derivation() {
        let key_1 = Argon2Hasher::key_derivation(PASSWORD, &Argon2Hasher::generate_salt()).unwrap();
        let key_2 = Argon2Hasher::key_derivation(PASSWORD, &Argon2Hasher::generate_salt()).unwrap();

        assert_ne!(key_1, key_2);
    }
}
