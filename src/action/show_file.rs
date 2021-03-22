use rusqlite::params;
use std::path::Path;

use crate::db::File;
use crate::error::Result;
use crate::location::Location;
use crate::project::Project;

pub fn do_show_file(project: &Project, path: &impl AsRef<Path>) -> Result<()> {
    let conn = project.open_db_connection()?;

    let location = Location::from_path(&project.dir, path)?;
    let file = File::by_location(&conn, &location)??;

    println!("Path: {}", path.as_ref().display());
    println!("Location: {}", file.location.as_str());
    println!("Signature: {}", file.signature.as_str());

    println!("Tags:");
    let mut stmt =
    conn.prepare("SELECT DISTINCT tags.name FROM file_tags INNER JOIN tags ON tags.id = file_tags.tag_id WHERE file_id = ?1 ORDER BY tags.name")?;
    let tag_names = stmt
        .query_map(params![file.id], |row| row.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    for tag_name in tag_names {
        println!("  {}", tag_name);
    }

    Ok(())
}
