use rusqlite::params;
use rusqlite::types::Value;
use std::rc::Rc;

use crate::db;
use crate::error::Result;
use crate::location::Location;
use crate::project::Project;
use crate::tag::Tag;

pub fn do_search(project: &Project, tags: &Vec<Tag>) -> Result<()> {
    let conn = project.open_db_connection()?;
    let names = tags.into_iter().map(|x| x.as_str()).collect();
    let tags = db::Tag::all_by_names(&conn, &names)?;

    let tag_ids = tags.iter().map(|x| x.id).collect::<Vec<_>>();
    let tag_id_values = Rc::new(
        tag_ids
            .iter()
            .map(|&x| Value::from(x))
            .collect::<Vec<Value>>(),
    );
    let mut stmt =
        conn.prepare("SELECT files.location FROM file_tags INNER JOIN files ON files.id = file_id WHERE tag_id IN RARRAY(?1)")?;
    let locations = stmt
        .query_map(params![tag_id_values], |row| row.get::<_, Location>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    for location in locations {
        println!("{}", location.as_str())
    }

    Ok(())
}
