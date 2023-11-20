use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde_json::{self, Value};
use sha256::digest;
use std::{fs::File, io::Read};

pub struct ConfigData {
    pub auth_password: String,
    pub auth_pw_hash: String,
}

pub async fn read_config() -> ConfigData {
    let file = File::open("mailbox.json");
    let mut file = match file {
        Ok(file) => file,
        Err(e) => panic!("Error while getting file! Error: {}", e),
    };

    let mut data = String::new();
    let result = file.read_to_string(&mut data);
    match result {
        Ok(_) => (),
        Err(e) => panic!("Error while reading config! Error: {}", e),
    }

    let config_data = serde_json::from_str(&data);
    let config_data: Value = match config_data {
        Ok(config_data) => config_data,
        Err(e) => panic!("Error while converting string to json! Error: {}", e),
    };

    let pw: String;
    if !config_data["password"].to_string().is_empty() {
        pw = config_data["password"].to_string();
    } else {
        pw = String::from("");
    }

    ConfigData {
        auth_password: pw.clone(),
        auth_pw_hash: digest(pw),
    }
}
