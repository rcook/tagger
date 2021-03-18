use rusqlite::params;
use rusqlite::types::Value;
use std::rc::Rc;

use crate::db;
use crate::error::Result;
use crate::project::Project;
use crate::tag::Tag;

pub fn do_search(project: &Project, tags: &Vec<Tag>) -> Result<()> {
    #[derive(Debug)]
    struct Record {
        id: i32,
        item_id: i32,
        tag_id: i32,
    }

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
        conn.prepare("SELECT id, item_id, tag_id FROM item_tags WHERE tag_id IN RARRAY(?1)")?;
    let records: Vec<_> = stmt
        .query_map(params![tag_id_values], |row| {
            Ok(Record {
                id: row.get(0)?,
                item_id: row.get(1)?,
                tag_id: row.get(2)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;

    for record in records {
        println!("record={:?}", record)
    }

    Ok(())
}
