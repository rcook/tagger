use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use std::path::Path;

use crate::error::Result;

#[derive(Debug)]
pub struct Location {
    value: String,
}

impl Location {
    pub fn new(value: &str) -> Self {
        Self {
            value: String::from(value),
        }
    }

    pub fn from_path(base: &Path, path: &Path) -> Result<Self> {
        Ok(Self {
            value: path.strip_prefix(base)?.to_str()?.to_string(),
        })
    }

    pub fn eq(&self, other: &Location) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() -> Result<()> {
        let location = Location::from_path(Path::new("/foo/bar"), Path::new("/foo/bar/aaa/bbb"))?;
        assert_eq!("aaa/bbb", location.value);
        Ok(())
    }
}
