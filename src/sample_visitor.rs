use std::fs::{self, DirEntry};
use std::path::Path;

use crate::error::Result;

pub trait PathChecker {
    fn matches(&self, path: &impl AsRef<Path>) -> Result<bool>;
}

pub fn visit(
    dir: &Path,
    path_checker: &impl PathChecker,
    cb: &dyn Fn(&DirEntry) -> Result<()>,
) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit(&path, path_checker, cb)?;
            } else {
                if path_checker.matches(&path)? {
                    cb(&entry)?;
                }
            }
        }
    }
    Ok(())
}
