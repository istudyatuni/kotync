use anyhow::Result;
use diesel::{Connection, SqliteConnection};

pub struct DB {
    conn: SqliteConnection,
}

impl DB {
    pub fn new(db_url: &str) -> Result<Self> {
        Ok(Self {
            conn: SqliteConnection::establish(db_url)?,
        })
    }
}
