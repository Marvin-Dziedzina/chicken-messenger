use std::{
    fs::File,
    io::{self, Read, Write},
};

use serde_json;

pub fn get_save_data(path: &str) -> serde_json::Value {
    let result = File::open(path);
    let mut file: File = match result {
        Ok(file) => file,
        Err(_) => {
            let result = create_database(path);
            match result {
                Ok(_) => (),
                Err(e) => panic!("Could not create file! Error: {}", e),
            }
            File::open(path).expect("Could not open database! How could this happen?")
        }
    };
    let mut data_buf = String::new();
    file.read_to_string(&mut data_buf)
        .expect("Could not read save file!");

    // check if empty
    if data_buf.trim().len() < 2 {
        let result = create_database(path);
        match result {
            Ok(_) => (),
            Err(e) => panic!("Could not create database! Error: {}", e),
        };
    }

    let result = serde_json::from_str(data_buf.as_str());
    match result {
        Ok(result) => result,
        Err(_) => {
            let result = create_database(path);
            match result {
                Ok(_) => (),
                Err(e) => panic!("Could not create database! Error: {}", e),
            };
            serde_json::from_str("{}").expect("Call me now! There is something badly wrong.")
        }
    }
}

pub fn write_save_data(path: &str, data: serde_json::Value) {
    let mut file = File::create(path).expect("Could not create file!");
    file.write_all(data.to_string().as_bytes())
        .expect("Could not write data!");
}

fn create_database(path: &str) -> Result<(), io::Error> {
    let result = File::create(path);
    let mut file = match result {
        Ok(file) => file,
        Err(e) => return Result::Err(e),
    };

    file.write_all(b"{}")
}
