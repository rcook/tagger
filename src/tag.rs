#[derive(Debug)]
pub struct Tag<'a>(&'a str);

impl<'a> Tag<'a> {
    pub fn from(s: &'a str) -> Self {
        Tag(s)
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}
