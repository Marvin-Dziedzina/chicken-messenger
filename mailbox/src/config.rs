use serde_json::{self, Value};
use std::{fs::File, io::Read};

pub struct ConfigData {
    password: String,
    admin_password: String,
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

    ConfigData {
        password: config_data["password"].to_string(),
        admin_password: config_data["admin_password"].to_string(),
    }
}
