use std::convert::TryFrom;

use crate::error::Error;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Like(String);

impl Like {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }

    fn new(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl TryFrom<&str> for Like {
    type Error = Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(Self::new(&value))
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;
    use crate::error::Result;

    #[test]
    fn test_try_from() -> Result<()> {
        let like = Like::try_from("LIKE")?;
        assert_eq!("LIKE", like.as_str());
        Ok(())
    }

    #[test]
    fn test_try_into() -> Result<()> {
        let like: Like = "LIKE".try_into()?;
        assert_eq!("LIKE", like.as_str());
        Ok(())
    }

    #[test]
    fn test_eq() -> Result<()> {
        assert_eq!(Like::try_from("LIKE")?, Like::try_from("LIKE")?);
        assert!(Like::try_from("LIKE")?.eq(&Like::try_from("LIKE")?));
        assert!(!Like::try_from("LIKE0")?.eq(&Like::try_from("LIKE1")?));
        assert!(Like::try_from("LIKE0")? != Like::try_from("LIKE1")?);
        Ok(())
    }
}
