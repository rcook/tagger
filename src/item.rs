use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::{params, Connection, OptionalExtension, Statement};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::copy;
use std::path::Path;

use crate::error::Result;

#[derive(Debug)]
pub struct Item2 {
    id: i32,
    location: Location,
    signature: Signature,
}

impl Item2 {
    #[allow(dead_code)]
    pub fn all_by_location(conn: &Connection, item: &Item) -> Result<Vec<Self>> {
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM items WHERE location = ?1")?;
        let record_iter = stmt.query_map(params![item.location], |row| {
            Ok(Self {
                id: row.get(0)?,
                location: row.get(1)?,
                signature: row.get(2)?,
            })
        })?;

        let mut items = Vec::new();
        for record in record_iter {
            items.push(record?)
        }

        Ok(items)
    }

    pub fn by_location(conn: &Connection, item: &Item) -> Result<Option<Self>> {
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM items WHERE location = ?1")?;
        Self::query_single(&mut stmt, params![item.location])
    }

    pub fn by_signature(conn: &Connection, item: &Item) -> Result<Option<Self>> {
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM items WHERE signature = ?1")?;
        Self::query_single(&mut stmt, params![item.signature])
    }

    pub fn signatures_eq(&self, item: &Item) -> bool {
        self.signature.eq(&item.signature)
    }

    pub fn locations_eq(&self, item: &Item) -> bool {
        self.location.eq(&item.location)
    }

    fn query_single(stmt: &mut Statement, params: &[&dyn ToSql]) -> Result<Option<Self>> {
        Ok(stmt
            .query_row(params, |row| {
                Ok(Self {
                    id: row.get(0)?,
                    location: row.get(1)?,
                    signature: row.get(2)?,
                })
            })
            .optional()?)
    }
}

pub struct Item {
    location: Location,
    signature: Signature,
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

#[derive(Debug)]
pub struct Location {
    value: String,
}

impl Location {
    pub fn from(base: &Path, path: &Path) -> Result<Self> {
        Ok(Self {
            value: path.strip_prefix(base)?.to_str()?.to_string(),
        })
    }

    fn new(value: &str) -> Self {
        Self {
            value: String::from(value),
        }
    }

    fn eq(&self, other: &Location) -> bool {
        self.value.eq(&other.value)
    }
}

impl FromSql for Location {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| Ok(Self::new(s)))
    }
}

impl ToSql for Location {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.value.as_str()))
    }
}

#[derive(Debug)]
pub struct Signature {
    value: String,
}

impl Signature {
    fn from_file(path: &Path) -> Result<Self> {
        let mut f = File::open(&path)?;
        let size = f.metadata()?.len();
        let mut hasher = Sha256::new();
        copy(&mut f, &mut hasher)?;
        let hash = hasher.finalize();
        Ok(Self {
            value: format!("{:x}:{}", hash, size),
        })
    }

    fn new(value: &str) -> Self {
        Self {
            value: String::from(value),
        }
    }

    fn eq(&self, other: &Signature) -> bool {
        self.value.eq(&other.value)
    }
}

impl FromSql for Signature {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| Ok(Self::new(s)))
    }
}

impl ToSql for Signature {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.value.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() -> Result<()> {
        let location = Location::from(Path::new("/foo/bar"), Path::new("/foo/bar/aaa/bbb"))?;
        assert_eq!("aaa/bbb", location.value);
        Ok(())
    }
}
