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
}
