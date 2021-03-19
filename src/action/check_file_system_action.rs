use crate::db;
use crate::error::Result;
use crate::item::Item;
use crate::project::Project;

pub fn do_check_file_system(project: &Project) -> Result<()> {
    println!("Checking {}", project.dir.display());

    let conn = project.open_db_connection()?;
    project
        .create_sample_visitor()
        .visit(&project.dir, &|entry| {
            let p = entry.path();
            let item = Item::from_file(&project.dir, &p)?;
            let rel_path = p.strip_prefix(&project.dir)?;
            let mut is_tracked = true;

            match db::Item::by_location(&conn, &item.location)? {
                Some(x) => if !x.signature.eq(&item.signature) {
                    println!("File {} is tracked but its signature has changed", rel_path.display())
                },
                None => is_tracked = false,
            };

            match db::Item::by_signature(&conn, &item.signature)? {
                Some(x) => if !x.location.eq(&item.location) {
                    println!("File {} is not tracked and has a signature matching an existing item in the database", rel_path.display())
                },
                None => is_tracked=false,
            };

            if !is_tracked {
                println!("File not tracked in database: {}", rel_path.display())
            }

            Ok(())
        })?;
    Ok(())
}
