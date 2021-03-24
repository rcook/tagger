use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use std::convert::TryFrom;
use std::path::{Path, PathBuf};

use crate::result::{Error, Result};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Location(String);

impl Location {
    #[cfg(windows)]
    const LOCATION_SEPARATOR: &'static str = "/";

    #[cfg(windows)]
    const OS_SEPARATOR: &'static str = "\\";

    pub fn from_path(base_dir: impl AsRef<Path>, path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self(Self::from_os_path_string(
            path.as_ref().strip_prefix(base_dir)?.to_str()?,
        )))
    }

    pub fn to_path(&self, base_dir: impl AsRef<Path>) -> PathBuf {
        base_dir.as_ref().join(Self::to_os_path_string(&self.0))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    fn new(value: &str) -> Self {
        Self(String::from(value))
    }

    #[cfg(windows)]
    fn from_os_path_string(path: &str) -> String {
        path.replace(Self::OS_SEPARATOR, Self::LOCATION_SEPARATOR)
    }

    #[cfg(not(windows))]
    fn from_os_path_string(path: &str) -> String {
        path.to_string()
    }

    #[cfg(windows)]
    fn to_os_path_string(value: &str) -> String {
        value.replace(Self::LOCATION_SEPARATOR, Self::OS_SEPARATOR)
    }

    #[cfg(not(windows))]
    fn to_os_path_string(value: &str) -> String {
        value.to_string()
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
        Ok(Self::new(&value))
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    #[test]
    #[cfg(windows)]
    fn test_from_path() -> Result<()> {
        let location = Location::from_path(
            Path::new("X:\\foo\\bar"),
            Path::new("X:\\foo\\bar\\aaa\\bbb"),
        )?;
        assert_eq!("aaa/bbb", location.as_str());
        Ok(())
    }

    #[test]
    #[cfg(not(windows))]
    fn test_from_path() -> Result<()> {
        let location = Location::from_path(Path::new("/foo/bar"), Path::new("/foo/bar/aaa/bbb"))?;
        assert_eq!("aaa/bbb", location.as_str());
        Ok(())
    }

    #[test]
    #[cfg(windows)]
    fn test_to_path() -> Result<()> {
        let location = Location::from_path(
            Path::new("X:\\foo\\bar"),
            Path::new("X:\\foo\\bar\\aaa\\bbb"),
        )?;
        let path = location.to_path(Path::new("X:\\foo\\bar"));
        assert_eq!("X:\\foo\\bar\\aaa\\bbb", path.to_str()?);
        Ok(())
    }

    #[test]
    #[cfg(not(windows))]
    fn test_to_path() -> Result<()> {
        let location = Location::from_path(Path::new("/foo/bar"), Path::new("/foo/bar/aaa/bbb"))?;
        let path = location.to_path(Path::new("/foo/bar"));
        assert_eq!("/foo/bar/aaa/bbb", path.to_str()?);
        Ok(())
    }

    #[test]
    fn test_try_from() -> Result<()> {
        let location = Location::try_from("LOCATION")?;
        assert_eq!("LOCATION", location.as_str());
        Ok(())
    }

    #[test]
    fn test_try_into() -> Result<()> {
        let location: Location = "LOCATION".try_into()?;
        assert_eq!("LOCATION", location.as_str());
        Ok(())
    }

    #[test]
    fn test_eq() -> Result<()> {
        assert_eq!(
            Location::try_from("LOCATION")?,
            Location::try_from("LOCATION")?
        );
        assert!(Location::try_from("LOCATION")?.eq(&Location::try_from("LOCATION")?));
        assert!(!Location::try_from("LOCATION0")?.eq(&Location::try_from("LOCATION1")?));
        assert!(Location::try_from("LOCATION0")? != Location::try_from("LOCATION1")?);
        Ok(())
    }
}
