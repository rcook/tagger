use std::collections::HashSet;
use std::fs::{self, DirEntry};
use std::path::Path;

use crate::error::Result;

pub struct ExtensionSet {
    extensions: HashSet<String>,
}

impl ExtensionSet {
    pub fn new(values: &[&'static str]) -> Self {
        let mut h = HashSet::new();
        for x in values {
            h.insert(x.to_lowercase());
        }
        Self { extensions: h }
    }

    fn matches(&self, path: &Path) -> bool {
        match path.extension() {
            Some(x) => self.extensions.contains(
                &x.to_str()
                    .expect("could not convert extension")
                    .to_lowercase(),
            ),
            None => false,
        }
    }
}

pub struct SampleVisitor {
    extensions: ExtensionSet,
}

impl SampleVisitor {
    pub fn new(extensions: ExtensionSet) -> Self {
        Self {
            extensions: extensions,
        }
    }

    pub fn visit(&self, dir: &Path, cb: &dyn Fn(&DirEntry) -> Result<()>) -> Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.visit(&path, cb)?;
                } else {
                    if self.extensions.matches(&path) {
                        cb(&entry)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_set() {
        let e = ExtensionSet::new(&["a", "B", "c"]);
        assert!(e.matches(Path::new("p/q/r.a")));
        assert!(e.matches(Path::new("p/q/r.A")));
        assert!(e.matches(Path::new("p/q/r.b")));
        assert!(e.matches(Path::new("p/q/r.B")));
        assert!(e.matches(Path::new("p/q/r.c")));
        assert!(e.matches(Path::new("p/q/r.C")));
        assert!(!e.matches(Path::new("p/q/r.d")));
        assert!(!e.matches(Path::new("p/q/r.D")));
        assert!(!e.matches(Path::new("p/q/r")))
    }
}
