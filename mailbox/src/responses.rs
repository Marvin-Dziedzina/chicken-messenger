use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Serialize, Deserialize)]
pub struct DefaultResponse<T> {
    error: bool,
    error_msg: String,
    success: bool,
    data: T,
}

impl<T> DefaultResponse<T> {
    pub fn new(error: bool, error_msg: &str, success: bool, data: T) -> DefaultResponse<T> {
        let mut error_msg = error_msg;
        if !error {
            error_msg = "";
        }

        DefaultResponse {
            error: error,
            error_msg: error_msg.to_string(),
            success: success,
            data: data,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    user_name: String,
    user_hash: String,
}

impl AuthResponse {
    pub fn new(user_name: &str, user_hash: &str) -> AuthResponse {
        AuthResponse {
            user_name: user_name.to_string(),
            user_hash: user_hash.to_string(),
        }
    }
}

pub fn handle_json_error(result: Result<String, Error>) -> String {
    match result {
        Ok(result) => result,
        Err(e) => {
            println!("Error: {}", e);
            "{\"error\":true,\"error_msg\":\"Could not serialize json! Please try again later.\",\"success\":false}".to_string()
        }
    }
}
