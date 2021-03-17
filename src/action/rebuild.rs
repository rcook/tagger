use crate::db;
use crate::error::Result;
use crate::item::Item;
use crate::project::Project;

pub fn do_rebuild(project: &Project) -> Result<()> {
    let conn = project.open_db_connection()?;
    project
        .create_sample_visitor()
        .visit(&project.dir, &|entry| {
            db::Item::upsert(&conn, &Item::from_file(&project.dir, &entry.path())?)?;
            Ok(())
        })?;
    Ok(())
}
