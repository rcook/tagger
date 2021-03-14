#![feature(osstring_ascii)]

use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::{self, DirEntry, File};
use std::io;
use std::path::Path;

use dirs;
use sha2::{Digest, Sha256};

struct ExtensionSet {
    extensions: HashSet<OsString>,
}

impl ExtensionSet {
    fn new(values: &[&'static str]) -> Self {
        let mut h = HashSet::new();
        for x in values {
            h.insert(OsString::from(x.to_lowercase()));
        }
        Self { extensions: h }
    }

    fn matches(&self, path: &Path) -> bool {
        match path.extension() {
            Some(x) => self.extensions.contains(&x.to_ascii_lowercase()),
            None => false,
        }
    }
}

struct SampleVisitor {
    extensions: ExtensionSet,
}

impl SampleVisitor {
    pub fn new(extensions: ExtensionSet) -> Self {
        Self {
            extensions: extensions,
        }
    }

    pub fn visit(&self, dir: &Path, cb: &dyn Fn(&DirEntry) -> io::Result<()>) -> io::Result<()> {
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

fn main() -> io::Result<()> {
    let visitor = SampleVisitor::new(ExtensionSet::new(&["aiff", "wav"]));

    let home_dir = dirs::home_dir().expect("could not determine home directory");

    visitor.visit(&home_dir, &|entry| {
        let p = entry.path();
        let mut file = File::open(&p)?;
        let mut hasher = Sha256::new();
        io::copy(&mut file, &mut hasher)?;
        let hash = hasher.finalize();
        println!(
            "{}: {:x}",
            p.to_str().expect("could not convert path"),
            hash
        );
        Ok(())
    })?;

    Ok(())
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
