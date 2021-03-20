use std::time::Instant;

use crate::db;
use crate::error::{Error, Result};
use crate::item::Item;
use crate::project::Project;

pub fn do_scan(project: &Project) -> Result<()> {
    let start = Instant::now();
    let conn = project.open_db_connection()?;
    project
        .create_sample_visitor()
        .visit(&project.dir, &|entry| {
            let item = Item::from_file(&project.dir, &entry.path())?;
            match db::Item::upsert(&conn, &item) {
                Ok(_) => {}
                Err(Error::Internal("Rusqlite", _)) => {
                    if db::DuplicateItem::upsert(&conn, &item)? != 0 {
                        println!(
                            "Duplicate file location and/or signature: {}, {}",
                            item.location.as_str(),
                            item.signature.as_str()
                        )
                    }
                }
                _ => {}
            }
            Ok(())
        })?;
    let elapsed = start.elapsed().as_secs();
    println!("Rebuild operation completed in {} seconds", elapsed);
    Ok(())
}
