use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::copy;
use std::path::Path;

use crate::error::Result;

#[derive(Debug)]
pub struct Signature {
    value: String,
}

impl Signature {
    pub fn from_file(path: &Path) -> Result<Self> {
        let mut f = File::open(&path)?;
        let size = f.metadata()?.len();
        let mut hasher = Sha256::new();
        copy(&mut f, &mut hasher)?;
        let hash = hasher.finalize();
        Ok(Self {
            value: format!("{:x}:{}", hash, size),
        })
    }

    pub fn eq(&self, other: &Signature) -> bool {
        self.value.eq(&other.value)
    }

    fn new(value: &str) -> Self {
        Self {
            value: String::from(value),
        }
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
