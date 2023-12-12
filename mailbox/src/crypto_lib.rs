use argon2::{self, Config};
use sha256;

struct Salted {
    salted: String,
}
impl Salted {
    pub fn new(val: String, salt: String) -> Salted {
        let prep_salt = Salted::prepare_salt(&salt);
        let mut val = val;
        val.push_str(&prep_salt);

        Salted { salted: val }
    }

    fn prepare_salt(salt: &String) -> String {
        let mut salt = salt.clone();

        salt.insert(0, ':');

        salt
    }
}

struct Hasher {}
impl Hasher {
    pub fn argon2(&self, string: String, salt: Option<&str>) -> String {
        "".to_string()
    }

    pub fn sha256(&self, val: &String, salt: Option<&String>) -> String {
        let mut salt: String = match salt {
            Some(salt) => salt.clone(),
            None => "".to_string(),
        };

        let mut val = val.clone();

        sha256::digest(val)
    }

    fn salt_password(val: &String, salt: String) -> Salted {
        let mut val = val.clone();

        val.push_str(salt.as_str());

        Salted { salted: val }
    }
}

struct Encrypt {}

struct Decrypt {}

struct PwHasher {}
impl PwHasher {
    pub fn digest(string: String) -> String {
        "".to_string()
    }
}
