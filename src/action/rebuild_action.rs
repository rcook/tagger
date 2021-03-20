use std::fs::{remove_file, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Instant;

use crate::db;
use crate::error::{Error, Result};
use crate::item::Item;
use crate::project::Project;

fn log_append(log_path: &impl AsRef<Path>, message: &str) {
    let mut file = match OpenOptions::new().create(true).append(true).open(log_path) {
        Err(e) => {
            eprintln!(
                "Failed to open log file {} ({})",
                log_path.as_ref().display(),
                e
            );
            return;
        }
        Ok(f) => f,
    };

    if let Err(e) = writeln!(file, "{}", message) {
        eprintln!(
            "Failed to write to log file {} ({})",
            log_path.as_ref().display(),
            e
        )
    }
}

pub fn do_rebuild(project: &Project, duplicates_path: &Option<impl AsRef<Path>>) -> Result<()> {
    if let Some(d) = duplicates_path {
        // Ignore failure
        let _ = remove_file(d);
    }

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
                        let message = format!(
                            "Duplicate file location and/or signature: {}, {}",
                            item.location.as_str(),
                            item.signature.as_str()
                        );
                        println!("{}", message);
                        if let Some(d) = duplicates_path {
                            log_append(&d, &message)
                        }
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
