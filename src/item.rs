use generic_array::GenericArray;
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use std::fs::File;
use std::io::copy;
use std::path::Path;

use crate::error::Result;

pub type Hash = GenericArray<u8, <Sha256 as Digest>::OutputSize>;

pub struct Item {
    path: ItemPath,
    hash: Hash,
    size: i64,
}

#[derive(Debug)]
pub struct ItemRecord {
    pub id: i32,
    pub path: String,
    pub hash: String,
    pub size: i64,
}

impl Item {
    pub fn from_file(start_dir: &Path, path: &Path) -> Result<Self> {
        let mut f = File::open(&path)?;
        let size = i64::try_from(f.metadata()?.len())?;
        let mut hasher = Sha256::new();
        copy(&mut f, &mut hasher)?;
        let hash: Hash = hasher.finalize();
        Ok(Self {
            path: ItemPath::from(&start_dir, &path)?,
            hash: hash,
            size: size,
        })
    }

    pub fn insert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO items (path, hash, size) VALUES (?1, ?2, ?3)",
            params![self.path.to_str(), format!("{:x}", self.hash), self.size],
        )?;
        Ok(())
    }

    pub fn upsert(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO items (path, hash, size) VALUES (?1, ?2, ?3)
                ON CONFLICT(path) DO UPDATE SET hash = ?2, size = ?3",
            params![self.path.to_str(), format!("{:x}", self.hash), self.size],
        )?;
        Ok(())
    }
}

pub struct ItemPath {
    value: String,
}

impl ItemPath {
    pub fn from(base: &Path, path: &Path) -> Result<Self> {
        Ok(Self {
            value: path.strip_prefix(base)?.to_str()?.to_string(),
        })
    }

    pub fn to_str(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() -> Result<()> {
        let item_path = ItemPath::from(Path::new("/foo/bar"), Path::new("/foo/bar/aaa/bbb"))?;
        assert_eq!("aaa/bbb", item_path.to_str());
        Ok(())
    }
}
