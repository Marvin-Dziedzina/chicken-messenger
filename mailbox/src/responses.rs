use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct Timestamp {
    timestamp: f64,
}
impl Timestamp {
    pub fn now() -> Timestamp {
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!");

        Timestamp {
            timestamp: since_epoch.as_secs_f64(),
        }
    }
}
