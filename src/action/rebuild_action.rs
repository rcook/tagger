use crate::db;
use crate::error::{Error, Result};
use crate::item::Item;
use crate::project::Project;

pub fn do_rebuild(project: &Project) -> Result<()> {
    let conn = project.open_db_connection()?;
    project
        .create_sample_visitor()
        .visit(&project.dir, &|entry| {
            let item = Item::from_file(&project.dir, &entry.path())?;
            match db::Item::upsert(&conn, &item) {
                Ok(_) => {}
                Err(Error::Internal("Rusqlite", _)) => println!(
                    "Duplicate file location and/or signature: {}, {}",
                    item.location.as_str(),
                    item.signature.as_str()
                ),
                _ => {}
            }
            Ok(())
        })?;
    Ok(())
}
