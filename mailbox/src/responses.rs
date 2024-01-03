use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    timestamp: f64,
    is_error: bool,
    error_msg: String,
    content: T,
}
impl<T> Response<T> {
    pub fn new(is_error: bool, error_msg: &str, content: T) -> Response<T> {
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");

        let error_msg = match is_error {
            true => error_msg,
            false => "",
        };

        Response {
            timestamp: since_epoch.as_secs_f64(),
            is_error,
            error_msg: error_msg.to_string(),
            content,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct NoContent {}
impl NoContent {
    pub fn new() -> NoContent {
        NoContent {}
    }
}

#[derive(Deserialize, Serialize)]
pub struct NameToUserHashContent {
    user_name: String,
    user_hash: String,
}
impl NameToUserHashContent {
    pub fn new(user_name: String, user_hash: String) -> NameToUserHashContent {
        NameToUserHashContent {
            user_name: user_name.to_string(),
            user_hash: user_hash.to_string(),
        }
    }
}
