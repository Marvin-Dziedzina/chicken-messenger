use sqlite;

pub struct SqLiteHDLR {
    path: String,
    conn: sqlite::Connection,
}

impl SqLiteHDLR {
    pub fn new(self, path: &str) -> SqLiteHDLR {
        let conn = sqlite::open(path).expect("Could not open sqlite connection!");

        SqLiteHDLR {
            path: path.to_string(),
            conn: conn,
        }
    }

    pub fn execute(self, query: String) {
        self.conn
            .execute(query)
            .expect("Invalid sqlite query provided!");
    }
}
