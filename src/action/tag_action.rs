use std::fmt::Debug;
use std::path::Path;

use crate::db;
use crate::error::Result;
use crate::location::Location;
use crate::project::Project;
use crate::tag::Tag;

pub fn do_tag(
    project: &Project,
    tags: &Vec<Tag>,
    paths: &Vec<impl AsRef<Path> + Debug>,
) -> Result<()> {
    let conn = project.open_db_connection()?;
    for tag in tags {
        let _ = db::Tag::upsert(&conn, tag)?;
    }

    let names = tags.into_iter().map(|x| x.as_str()).collect();
    let tags = db::Tag::all_by_names(&conn, &names)?;
    let locations = paths
        .into_iter()
        .map(|x| Location::from_path(&project.dir, x))
        .collect::<Result<_>>()?;
    let items = db::Item::all_by_locations(&conn, &locations)?;

    for item in &items {
        for tag in &tags {
            let _ = db::ItemTag::upsert(&conn, item.id, tag.id)?;
        }
    }

    Ok(())
}
