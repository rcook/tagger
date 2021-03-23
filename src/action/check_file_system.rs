use crate::db;
use crate::error::Result;
use crate::file_info::FileInfo;
use crate::project::Project;
use crate::sample_visitor;

pub fn do_check_file_system(project: &Project) -> Result<()> {
    println!("Checking {}", project.dir.display());

    let conn = project.open_db_connection()?;
    sample_visitor::visit(&project.dir, project.path_checker(), &mut |entry| {
        let p = entry.path();
        let file_info = FileInfo::from_file(&project.dir, &p)?;
        let rel_path = p.strip_prefix(&project.dir)?;
        let mut has_error = false;
        let mut message_shown = false;

        match db::File::by_location(&conn, &file_info.location)? {
            Some(x) => {
                if !x.signature.eq(&file_info.signature) {
                    println!(
                        "File {} is tracked but its signature has changed",
                        rel_path.display()
                    );
                    has_error = true;
                    message_shown = true;
                }
            }
            None => has_error = true,
        };

        match db::File::by_signature(&conn, &file_info.signature)? {
            Some(x) => {
                if !x.location.eq(&file_info.location) {
                    println!("File {} is not tracked and has a signature matching an existing item in the database", rel_path.display());
                    has_error = true;
                    message_shown = true;
                }
            }
            None => has_error = true,
        };

        if has_error && !message_shown {
            println!("File not tracked in database: {}", rel_path.display())
        }

        Ok(())
    })?;
    Ok(())
}
