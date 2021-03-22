use rusqlite::types::{ToSql, Value};
use rusqlite::{params, Connection, OptionalExtension, Statement, NO_PARAMS};
use std::rc::Rc;

use super::util::make_like_expression;
use crate::error::Result;
use crate::file_info;
use crate::like::Like;
use crate::location::Location;
use crate::signature::Signature;
use crate::tag;

type Id = i64;

#[derive(Debug)]
pub struct File {
    pub id: Id,
    pub location: Location,
    pub signature: Signature,
}

#[derive(Debug)]
pub struct DuplicateFile {
    pub id: Id,
    pub location: Location,
    pub signature: Signature,
}

#[derive(Debug)]
pub struct Tag {
    pub id: Id,
    pub name: String,
}

#[derive(Debug)]
pub struct FileTag {
    pub id: Id,
    pub file_id: Id,
    pub tag_id: Id,
}

fn to_sql_values(values: &Vec<&str>) -> Rc<Vec<Value>> {
    Rc::new(
        values
            .iter()
            .copied()
            .map(|x| Value::from(x.to_string()))
            .collect::<Vec<Value>>(),
    )
}

impl File {
    pub fn all(conn: &Connection, like: Option<Like>) -> Result<Vec<Self>> {
        let sql = match like {
            Some(l) => format!(
                "SELECT id, location, signature FROM files WHERE location {}",
                make_like_expression(&l)
            ),
            None => String::from("SELECT id, location, signature FROM files"),
        };
        let mut stmt = conn.prepare(&sql)?;
        Self::query_multi(&mut stmt, NO_PARAMS)
    }

    pub fn all_by_location(conn: &Connection, location: &Location) -> Result<Vec<Self>> {
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM files WHERE location = ?1")?;
        Self::query_multi(&mut stmt, params![location])
    }

    pub fn all_by_locations(conn: &Connection, locations: &Vec<Location>) -> Result<Vec<Self>> {
        let location_values = Rc::new(
            locations
                .iter()
                .map(|x| Value::from(x.as_str().to_string()))
                .collect::<Vec<Value>>(),
        );
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM files WHERE location IN RARRAY(?1)")?;
        Self::query_multi(&mut stmt, params![location_values])
    }

    pub fn by_location(conn: &Connection, location: &Location) -> Result<Option<Self>> {
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM files WHERE location = ?1")?;
        Self::query_single(&mut stmt, params![location])
    }

    pub fn by_signature(conn: &Connection, signature: &Signature) -> Result<Option<Self>> {
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM files WHERE signature = ?1")?;
        Self::query_single(&mut stmt, params![signature])
    }

    pub fn insert(conn: &Connection, file_info: &file_info::FileInfo) -> Result<Id> {
        conn.execute(
            "INSERT INTO files (location, signature) VALUES (?1, ?2)",
            params![file_info.location, file_info.signature],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn upsert(conn: &Connection, file_info: &file_info::FileInfo) -> Result<Id> {
        conn.execute(
            "INSERT INTO files (location, signature) VALUES (?1, ?2)
                ON CONFLICT(location) DO UPDATE SET signature = ?2",
            params![file_info.location, file_info.signature],
        )?;
        Ok(conn.last_insert_rowid())
    }

    fn query_single(stmt: &mut Statement, params: &[&dyn ToSql]) -> Result<Option<Self>> {
        Ok(stmt
            .query_row(params, |row| {
                Ok(Self {
                    id: row.get(0)?,
                    location: row.get(1)?,
                    signature: row.get(2)?,
                })
            })
            .optional()?)
    }

    fn query_multi(stmt: &mut Statement, params: &[&dyn ToSql]) -> Result<Vec<Self>> {
        Ok(stmt
            .query_map(params, |row| {
                Ok(Self {
                    id: row.get(0)?,
                    location: row.get(1)?,
                    signature: row.get(2)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?)
    }
}

impl DuplicateFile {
    pub fn all(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, location, signature FROM duplicate_files")?;
        Self::query_multi(&mut stmt, NO_PARAMS)
    }

    pub fn upsert(conn: &Connection, file_info: &file_info::FileInfo) -> Result<Id> {
        conn.execute(
            "INSERT INTO duplicate_files (location, signature) VALUES (?1, ?2)
                ON CONFLICT(location) DO UPDATE SET signature = ?2",
            params![file_info.location, file_info.signature],
        )?;
        Ok(conn.last_insert_rowid())
    }

    fn query_multi(stmt: &mut Statement, params: &[&dyn ToSql]) -> Result<Vec<Self>> {
        Ok(stmt
            .query_map(params, |row| {
                Ok(Self {
                    id: row.get(0)?,
                    location: row.get(1)?,
                    signature: row.get(2)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?)
    }
}

impl Tag {
    pub fn all(conn: &Connection, like: Option<Like>) -> Result<Vec<Self>> {
        let sql = match like {
            Some(l) => format!(
                "SELECT id, name FROM tags WHERE name {}",
                make_like_expression(&l)
            ),
            None => String::from("SELECT id, name FROM tags"),
        };
        let mut stmt = conn.prepare(&sql)?;
        Self::query_multi(&mut stmt, NO_PARAMS)
    }

    pub fn all_by_names(conn: &Connection, names: &Vec<&str>) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, name FROM tags WHERE name IN RARRAY(?1)")?;
        Self::query_multi(&mut stmt, params![to_sql_values(names)])
    }

    pub fn upsert(conn: &Connection, tag: &tag::Tag) -> Result<Id> {
        conn.execute(
            "INSERT INTO tags (name) VALUES (?1)
                ON CONFLICT(name) DO NOTHING",
            params![tag.as_str()],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn delete_by_names(conn: &Connection, names: &Vec<&str>) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("DELETE FROM tags WHERE name IN RARRAY(?1)")?;
        Self::query_multi(&mut stmt, params![to_sql_values(names)])
    }

    fn query_multi(stmt: &mut Statement, params: &[&dyn ToSql]) -> Result<Vec<Self>> {
        Ok(stmt
            .query_map(params, |row| {
                Ok(Self {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?)
    }
}

impl FileTag {
    pub fn all(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, file_id, tag_id FROM file_tags")?;
        Self::query_multi(&mut stmt, NO_PARAMS)
    }

    pub fn upsert(conn: &Connection, file_id: Id, tag_id: Id) -> Result<Id> {
        let mut stmt = conn.prepare(
            "INSERT INTO file_tags (file_id, tag_id) VALUES (?1, ?2)
                ON CONFLICT(file_id, tag_id) DO NOTHING",
        )?;
        stmt.execute(params![file_id, tag_id])?;
        Ok(conn.last_insert_rowid())
    }

    fn query_multi(stmt: &mut Statement, params: &[&dyn ToSql]) -> Result<Vec<Self>> {
        Ok(stmt
            .query_map(params, |row| {
                Ok(Self {
                    id: row.get(0)?,
                    file_id: row.get(1)?,
                    tag_id: row.get(2)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::*;
    use crate::db::run_migrations;

    #[test]
    fn basics() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        rusqlite::vtab::array::load_module(&conn)?;
        run_migrations(&conn)?;

        assert!(File::all(&conn, None)?.is_empty());
        assert!(DuplicateFile::all(&conn)?.is_empty());
        assert!(Tag::all(&conn, None)?.is_empty());
        assert!(FileTag::all(&conn)?.is_empty());

        File::insert(
            &conn,
            &file_info::FileInfo::new(
                Location::try_from("LOCATION0")?,
                Signature::try_from("SIGNATURE0")?,
            ),
        )?;
        File::insert(
            &conn,
            &file_info::FileInfo::new(
                Location::try_from("LOCATION1")?,
                Signature::try_from("SIGNATURE1")?,
            ),
        )?;

        assert_eq!(2, File::all(&conn, None)?.len());
        assert!(DuplicateFile::all(&conn)?.is_empty());

        assert_eq!(
            1,
            File::all_by_location(&conn, &Location::try_from("LOCATION0")?)?.len()
        );
        assert!(File::all_by_location(&conn, &Location::try_from("UNKNOWN-LOCATION")?)?.is_empty());

        assert_eq!(
            2,
            File::all_by_locations(
                &conn,
                &vec![
                    Location::try_from("LOCATION0")?,
                    Location::try_from("UNKNOWN-LOCATION")?,
                    Location::try_from("LOCATION1")?
                ]
            )?
            .len()
        );

        Tag::upsert(&conn, &tag::Tag::from("tag0"))?;
        Tag::upsert(&conn, &tag::Tag::from("tag1"))?;
        Tag::upsert(&conn, &tag::Tag::from("tag2"))?;

        assert_eq!(3, Tag::all(&conn, None)?.len());

        let tags = Tag::all_by_names(&conn, &vec!["tag0", "tag1"])?;
        assert_eq!(2, tags.len());
        assert_eq!(1, tags[0].id);
        assert_eq!("tag0", tags[0].name);
        assert_eq!(2, tags[1].id);
        assert_eq!("tag1", tags[1].name);

        DuplicateFile::upsert(
            &conn,
            &file_info::FileInfo::new(
                Location::try_from("LOCATION0")?,
                Signature::try_from("SIGNATURE0")?,
            ),
        )?;

        assert_eq!(2, File::all(&conn, None)?.len());
        assert_eq!(1, DuplicateFile::all(&conn)?.len());

        Ok(())
    }
}
