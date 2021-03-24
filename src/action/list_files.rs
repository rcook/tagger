use itertools::Itertools;

use crate::db;
use crate::like::Like;
use crate::project::Project;
use crate::result::Result;

pub fn do_list_files(project: &Project, like: Option<Like>) -> Result<()> {
    let conn = project.open_db_connection()?;

    println!("Project directory: {}", project.dir.display());
    println!("Database path: {}", project.db_path.display());

    println!("Files:");
    for file in db::File::all(&conn, like)?
        .iter()
        .sorted_by_key(|&x| x.location.as_str())
    {
        println!("  {}", file.location.as_str());
    }

    Ok(())
}
