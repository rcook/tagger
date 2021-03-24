use std::path::Path;

use crate::location::Location;
use crate::result::Result;
use crate::signature::Signature;

pub struct FileInfo {
    pub location: Location,
    pub signature: Signature,
}

impl FileInfo {
    pub fn new(location: Location, signature: Signature) -> Self {
        Self {
            location: location,
            signature: signature,
        }
    }

    pub fn from_file(start_dir: &Path, path: &Path) -> Result<Self> {
        Ok(Self {
            location: Location::from_path(&start_dir, &path)?,
            signature: Signature::from_file(path)?,
        })
    }
}
