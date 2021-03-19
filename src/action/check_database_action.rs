use crate::db;
use crate::error::Result;
use crate::project::Project;

pub fn do_check_database(project: &Project) -> Result<()> {
    println!("Checking {}", project.db_path.display());

    let conn = project.open_db_connection()?;
    for item in db::Item::all(&conn)? {
        let path = item.location.to_path(&project.dir);
        if !path.exists() {
            println!(
                "File in database does not exist in file system: {}",
                path.display()
            );
        }
    }

    Ok(())
}
