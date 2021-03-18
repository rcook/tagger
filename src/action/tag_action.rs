use std::fmt::Debug;
use std::path::Path;

use crate::error::Result;
use crate::project::Project;
use crate::tag::Tag;

pub fn do_tag(
    _project: &Project,
    tags: &Vec<Tag>,
    paths: &Vec<impl AsRef<Path> + Debug>,
) -> Result<()> {
    println!("do_tag");
    for tag in tags {
        println!("tag={:?}", tag)
    }
    for path in paths {
        println!("path={:?}", path)
    }
    Ok(())
}
