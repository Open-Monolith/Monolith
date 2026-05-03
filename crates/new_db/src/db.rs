use rusqlite::{Connection, Result};

pub struct MonoDb {
    pub conn: Connection,
}

impl MonoDb {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS elements (
                id INTEGER PRIMARY KEY,
                name TEXT,
                kind TEXT NOT NULL,
                kind_type TEXT,
                spec_id INTEGER,
                level_id INTEGER
            );

            CREATE TABLE IF NOT EXISTS placement_pose (
                element_id INTEGER PRIMARY KEY,
                px REAL NOT NULL,
                py REAL NOT NULL,
                pz REAL NOT NULL,
                qx REAL NOT NULL,
                qy REAL NOT NULL,
                qz REAL NOT NULL,
                qw REAL NOT NULL,
                FOREIGN KEY (element_id) REFERENCES elements(id)
            );
            ",
        )?;

        Ok(Self { conn })
    }
}