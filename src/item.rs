use generic_array::GenericArray;
use rusqlite::types::{ToSql, ToSqlOutput};
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::copy;
use std::path::Path;

use crate::error::Result;

pub type Hash = GenericArray<u8, <Sha256 as Digest>::OutputSize>;

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

    pub fn insert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO items (location, signature) VALUES (?1, ?2)",
            params![self.location.to_str(), self.signature],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn upsert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO items (location, signature) VALUES (?1, ?2, ?3)
                ON CONFLICT(location) DO UPDATE SET hash = ?2, size = ?3",
            params![self.location, self.signature],
        )?;
        Ok(())
    }
}

pub struct Location {
    value: String,
}

impl Location {
    pub fn from(base: &Path, path: &Path) -> Result<Self> {
        Ok(Self {
            value: path.strip_prefix(base)?.to_str()?.to_string(),
        })
    }

    pub fn to_str(&self) -> &str {
        &self.value
    }
}

impl ToSql for Location {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.value.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() -> Result<()> {
        let item_path = Location::from(Path::new("/foo/bar"), Path::new("/foo/bar/aaa/bbb"))?;
        assert_eq!("aaa/bbb", item_path.to_str());
        Ok(())
    }
}

pub struct Signature {
    hash: Hash,
    size: u64,
}

impl Signature {
    fn from_file(path: &Path) -> Result<Self> {
        let mut f = File::open(&path)?;
        let size = f.metadata()?.len();
        let mut hasher = Sha256::new();
        copy(&mut f, &mut hasher)?;
        let hash = hasher.finalize();
        Ok(Self {
            hash: hash,
            size: size,
        })
    }
}

impl ToSql for Signature {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let signature_str = format!("{:x}:{}", self.hash, self.size);
        Ok(ToSqlOutput::from(signature_str))
    }
}
