use rusqlite::{params, Connection};
use std::path::Path;

use crate::error::Result;
use crate::location::Location;
use crate::signature::Signature;

pub struct Item {
    pub location: Location,
    pub signature: Signature,
}

impl Item {
    pub fn from_file(start_dir: &Path, path: &Path) -> Result<Self> {
        Ok(Self {
            location: Location::from(&start_dir, &path)?,
            signature: Signature::from_file(path)?,
        })
    }

    #[allow(dead_code)]
    pub fn insert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO items (location, signature) VALUES (?1, ?2)",
            params![self.location, self.signature],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn upsert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO items (location, signature) VALUES (?1, ?2)
                ON CONFLICT(location) DO UPDATE SET signature = ?2",
            params![self.location, self.signature],
        )?;
        Ok(())
    }
}
