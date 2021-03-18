use std::fmt::Debug;
use std::path::Path;

use crate::db;
use crate::error::Result;
use crate::project::Project;
use crate::tag::Tag;

pub fn do_tag(
    project: &Project,
    tags: &Vec<Tag>,
    paths: &Vec<impl AsRef<Path> + Debug>,
) -> Result<()> {
    let conn = project.open_db_connection()?;
    for tag in tags {
        db::Tag::upsert(&conn, tag)?
    }

    for x in db::Tag::all_by_names(&conn, &tags.into_iter().map(|x| x.as_str()).collect()) {
        println!("tag: {:?}", x)
    }

    for path in paths {
        println!("path={:?}", path)
    }
    Ok(())
}
