use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use std::convert::TryFrom;
use std::fmt;
use std::path::Path;

use crate::error::{Error, Result};

#[derive(Debug)]
pub struct Location(String);

impl Location {
    pub fn from_path(base: impl AsRef<Path>, path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self(
            path.as_ref().strip_prefix(base)?.to_str()?.to_string(),
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn eq(&self, other: &Location) -> bool {
        self.0.eq(&other.0)
    }

    fn new(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl FromSql for Location {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        value.as_str().and_then(|s| Ok(Self::new(s)))
    }
}

impl ToSql for Location {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.as_str()))
    }
}

impl TryFrom<&str> for Location {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(Location::new(&value))
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    fn test_from_path() -> Result<()> {
        let location = Location::from_path(Path::new("/foo/bar"), Path::new("/foo/bar/aaa/bbb"))?;
        assert_eq!("aaa/bbb", location.to_string());
        Ok(())
    }

    #[test]
    fn test_try_from() -> Result<()> {
        let location = Location::try_from("LOCATION")?;
        assert_eq!("LOCATION", location.to_string());
        Ok(())
    }

    #[test]
    fn test_try_info() -> Result<()> {
        let location: Location = "LOCATION".try_into()?;
        assert_eq!("LOCATION", location.to_string());
        Ok(())
    }
}
