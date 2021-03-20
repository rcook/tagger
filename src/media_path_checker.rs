use regex::Regex;
use std::path::{Path, MAIN_SEPARATOR};

use crate::error::Result;
use crate::sample_visitor::PathChecker;

pub struct MediaPathChecker {}

impl MediaPathChecker {
    pub fn new() -> Self {
        Self {}
    }
}

const MEDIA_FILE_EXTENSIONS: [&'static str; 8] =
    ["aiff", "au", "mid", "m4a", "mp3", "snd", "wav", "wma"];
const EXCLUDED_DIRECTORIES: [&'static str; 1] = ["__MACOSX"];

fn create_include_regexes(extensions: &[&str]) -> Result<Vec<Regex>> {
    let pattern = format!(
        r"(?i)^.+\.({})$",
        extensions
            .iter()
            .map(|x| regex::escape(x))
            .collect::<Vec<_>>()
            .join("|")
    );
    Ok(vec![Regex::new(&pattern)?])
}

fn create_exclude_regexes() -> Result<Vec<Regex>> {
    let escaped_separator = regex::escape(&MAIN_SEPARATOR.to_string());
    Ok(EXCLUDED_DIRECTORIES
        .iter()
        .map(|x| {
            Regex::new(&format!(
                r"^.+{}{}{}.+$",
                escaped_separator,
                regex::escape(x),
                escaped_separator
            ))
        })
        .collect::<std::result::Result<Vec<Regex>, regex::Error>>()?)
}

lazy_static! {
    static ref INCLUDE_REGEXES: Vec<Regex> =
        create_include_regexes(&MEDIA_FILE_EXTENSIONS).expect("Static initialization failure");
    static ref EXCLUDE_REGEXES: Vec<Regex> =
        create_exclude_regexes().expect("Static initialization failure");
}

fn matches_any_of(regexes: &Vec<Regex>, s: &str) -> bool {
    regexes.iter().any(|x| x.is_match(s))
}

impl PathChecker for MediaPathChecker {
    fn matches(&self, path: &impl AsRef<Path>) -> Result<bool> {
        let path_str = path.as_ref().to_str()?;
        Ok(matches_any_of(&INCLUDE_REGEXES, path_str)
            && !matches_any_of(&EXCLUDE_REGEXES, path_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_any_of() -> Result<()> {
        let e = create_include_regexes(&["a", "B", "c"])?;
        assert!(matches_any_of(&e, "p/q/r.a"));
        assert!(matches_any_of(&e, "p/q/r.A"));
        assert!(matches_any_of(&e, "p/q/r.b"));
        assert!(matches_any_of(&e, "p/q/r.B"));
        assert!(matches_any_of(&e, "p/q/r.c"));
        assert!(matches_any_of(&e, "p/q/r.C"));
        assert!(!matches_any_of(&e, "p/q/r.d"));
        assert!(!matches_any_of(&e, "p/q/r.D"));
        assert!(!matches_any_of(&e, "p/q/r"));
        Ok(())
    }
}
