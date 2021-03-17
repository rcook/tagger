use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::walk::{ExtensionSet, SampleVisitor};

pub struct Project {
    pub dir: PathBuf,
    db_path: PathBuf,
}

impl Project {
    pub fn from_dir<P: AsRef<Path>>(dir: P) -> Self {
        let db_path = dir.as_ref().join("tagger.db");
        Self {
            dir: dir.as_ref().to_owned(),
            db_path: db_path,
        }
    }

    pub fn open_db_connection(&self) -> Result<Connection> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id          INTEGER PRIMARY KEY,
                location    TEXT NOT NULL UNIQUE,
                signature   TEXT NOT NULL UNIQUE
            )",
            params![],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id          INTEGER PRIMARY KEY,
                name        TEXT NOT NULL UNIQUE
            )",
            params![],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS item_tags (
                id          INTEGER PRIMARY KEY,
                item_id     INTEGER NOT NULL,
                tag_id      INTEGER NOT NULL,
                FOREIGN KEY(item_id) REFERENCES items(id),
                FOREIGN KEY(tag_id) REFERENCES tags(id)
            )",
            params![],
        )?;
        Ok(conn)
    }

    pub fn create_sample_visitor(&self) -> SampleVisitor {
        SampleVisitor::new(ExtensionSet::new(&["aiff", "wav"]))
    }
}
