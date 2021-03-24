use crate::db;
use crate::project::Project;
use crate::result::Result;

pub fn do_check_database(project: &Project) -> Result<()> {
    println!("Checking {}", project.db_path.display());

    let conn = project.open_db_connection()?;
    for file in db::File::all(&conn, None)? {
        let path = file.location.to_path(&project.dir);
        if !path.exists() {
            println!(
                "File in database does not exist in file system: {}",
                path.display()
            );
        }
    }

    Ok(())
}
