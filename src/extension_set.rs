use std::collections::HashSet;
use std::path::Path;

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

    pub fn matches(&self, path: &Path) -> bool {
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
