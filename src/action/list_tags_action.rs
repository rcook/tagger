use itertools::Itertools;

use crate::db;
use crate::error::Result;
use crate::project::Project;

pub fn do_list_tags(project: &Project) -> Result<()> {
    let conn = project.open_db_connection()?;

    println!("Project directory: {}", project.dir.display());
    println!("Database path: {}", project.db_path.display());

    println!("Tags:");
    for tag in db::Tag::all(&conn)?.iter().sorted_by_key(|&x| &x.name) {
        println!("  {}", tag.name);
    }

    Ok(())
}
