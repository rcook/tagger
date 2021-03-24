use itertools::Itertools;

use crate::db;
use crate::like::Like;
use crate::project::Project;
use crate::result::Result;

pub fn do_list_tags(project: &Project, like: Option<Like>) -> Result<()> {
    let conn = project.open_db_connection()?;

    println!("Project directory: {}", project.dir.display());
    println!("Database path: {}", project.db_path.display());

    println!("Tags:");
    for tag in db::Tag::all(&conn, like)?
        .iter()
        .sorted_by_key(|&x| &x.name)
    {
        println!("  {}", tag.name);
    }

    Ok(())
}
