use rusqlite::{Connection, Result};

pub struct MonoDb {
    pub conn: Connection,
}

impl MonoDb {
    pub fn open(path:&str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS elements (
                id INTEGER PRIMARY KEY,
                name TEXT,
                kind TEXT NOT NULL,
                type_id INTEGER,
                level_id INTEGER,
                kind_variant_id INTEGER,
            );
            ",
        )?;
        Ok(Self { conn })
    }
}