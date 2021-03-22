use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

use crate::db;
use crate::error::{user_error_result, Result};
use crate::location::Location;
use crate::project::Project;
use crate::tag::Tag;

pub fn do_tag(
    project: &Project,
    tags: &Vec<Tag>,
    paths: &Vec<impl AsRef<Path> + Debug>,
) -> Result<()> {
    let conn = project.open_db_connection()?;

    let locations = paths
        .into_iter()
        .map(|x| Location::from_path(&project.dir, x))
        .collect::<Result<_>>()?;
    let files = db::File::all_by_locations(&conn, &locations)?;
    if files.len() != locations.len() {
        let h = files
            .iter()
            .map(|x| (&x.location, x))
            .collect::<HashMap<_, _>>();
        let missing_locations_str = locations
            .iter()
            .filter(|&x| !h.contains_key(&x))
            .map(|x| x.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        return user_error_result(format!(
            "No files found at locations {}",
            missing_locations_str
        ));
    }

    for tag in tags {
        let _ = db::Tag::upsert(&conn, tag)?;
    }

    let names = tags.into_iter().map(|x| x.as_str()).collect();
    let tags = db::Tag::all_by_names(&conn, &names)?;

    for file in &files {
        for tag in &tags {
            let _ = db::FileTag::upsert(&conn, file.id, tag.id)?;
        }
    }

    Ok(())
}
