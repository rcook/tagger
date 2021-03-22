use std::time::Instant;

use crate::db;
use crate::error::{Error, Result};
use crate::file_info::FileInfo;
use crate::project::Project;
use crate::sample_visitor;

pub fn do_scan(project: &Project) -> Result<()> {
    let start = Instant::now();
    let conn = project.open_db_connection()?;
    sample_visitor::visit(&project.dir, project.path_checker(), &|entry| {
        let file_info = FileInfo::from_file(&project.dir, &entry.path())?;
        match db::File::upsert(&conn, &file_info) {
            Ok(_) => {}
            Err(Error::Internal("Rusqlite", _)) => {
                if db::DuplicateFile::upsert(&conn, &file_info)? != 0 {
                    println!(
                        "Duplicate file location and/or signature: {}, {}",
                        file_info.location.as_str(),
                        file_info.signature.as_str()
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
