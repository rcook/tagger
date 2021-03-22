use crate::db;
use crate::error::Result;
use crate::project::Project;
use crate::tag::Tag;

pub fn do_delete_tag(project: &Project, tags: &Vec<Tag>) -> Result<()> {
    let conn = project.open_db_connection()?;
    let names = tags.into_iter().map(|x| x.as_str()).collect();
    db::Tag::delete_by_names(&conn, &names)?;
    Ok(())
}
