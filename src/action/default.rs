use crate::error::Result;
use crate::project::Project;

pub fn do_default(project: &Project) -> Result<()> {
    println!("Project directory: {}", project.dir.display());
    println!("Database path: {}", project.db_path.display());

    match project.db_path.exists() {
        true => println!("Database file exists."),
        false => println!("Database file has not been created yet."),
    };

    Ok(())
}
