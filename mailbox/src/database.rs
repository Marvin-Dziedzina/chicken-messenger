use std::{
    fs::File,
    io::{Read, Write},
};

use serde_json;

pub fn get_save_data(path: &str) -> serde_json::Value {
    let mut file = File::open(path).expect("Could not open save file!");
    let mut data_buf = String::new();
    file.read_to_string(&mut data_buf)
        .expect("Could not read save file!");

    let result = serde_json::from_str(data_buf.as_str());
    match result {
        Ok(result) => result,
        Err(_) => serde_json::from_str("{}").expect("Call me now! There is something badly wrong."),
    }
}

pub fn write_save_data(path: &str, data: serde_json::Value) {
    let mut file = File::create(path).expect("Could not create file!");
    file.write_all(data.to_string().as_bytes())
        .expect("Could not write data!");
}
