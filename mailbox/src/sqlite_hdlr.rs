use sqlite;
use std::{fs::File, io::Write};

pub struct SqLiteHDLR {
    path: String,
    conn: sqlite::Connection,
    log_file_path: String,
}

impl SqLiteHDLR {
    pub fn new(path: &str) -> SqLiteHDLR {
        let conn = sqlite::open(path).expect("Could not open sqlite connection!");

        let mut users_table_exist = false;
        conn.iterate(
            "SELECT name FROM sqlite_master WHERE type=\"table\" AND name=\"users\"",
            |_| {
                users_table_exist = true;
                false
            },
        )
        .unwrap();

        if !users_table_exist {
            conn.execute(
                "CREATE TABLE users (user_name TEXT, user_address TEXT, password_hash TEXT, salt TEXT)",
            )
            .expect("Could not create users table!");
        }

        SqLiteHDLR {
            path: path.to_string(),
            conn: conn,
            log_file_path: "out.log".to_string(),
        }
    }

    pub fn execute(&self, query: &str) {
        self.conn
            .execute(query)
            .expect("Invalid sqlite query provided!");
    }

    pub fn iter_execute<F>(
        &self,
        query: &str,
        callback: fn(pairs: &[(&str, Option<&str>)]) -> bool,
    ) -> Result<(), sqlite::Error> {
        return self.conn.iterate(query, callback);
    }

    fn write_log(&self, msg: &str) {
        let mut log_file = File::create(&self.log_file_path).expect("Cant open log file!");
        let mut log_msg = msg.to_string();
        log_msg.push_str("\n");

        log_file
            .write_all(log_msg.as_bytes())
            .expect(format!("Could not write to {}", &self.log_file_path).as_str());
    }
}
