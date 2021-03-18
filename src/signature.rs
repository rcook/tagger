use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use std::fs::File;
use std::io::copy;
use std::path::Path;

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct Signature(String);

impl Signature {
    pub fn from_file(path: &Path) -> Result<Self> {
        let mut f = File::open(&path)?;
        let size = f.metadata()?.len();
        let mut hasher = Sha256::new();
        copy(&mut f, &mut hasher)?;
        let hash = hasher.finalize();
        Ok(Self(format!("{:x}:{}", hash, size)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn eq(&self, other: &Signature) -> bool {
        self.0.eq(&other.0)
    }

    fn new(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl FromSql for Signature {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| Ok(Self::new(s)))
    }
}

impl ToSql for Signature {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.as_str()))
    }
}

impl TryFrom<&str> for Signature {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(Signature::new(&value))
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn test_try_from() -> Result<()> {
        let signature = Signature::try_from("SIGNATURE")?;
        assert_eq!("SIGNATURE", signature.as_str());
        Ok(())
    }

    #[test]
    fn test_try_info() -> Result<()> {
        let signature: Signature = "SIGNATURE".try_into()?;
        assert_eq!("SIGNATURE", signature.as_str());
        Ok(())
    }
}
