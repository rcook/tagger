use rusqlite::types::{ToSql, Value};
use rusqlite::{params, Connection, OptionalExtension, Statement};
use std::rc::Rc;

use crate::error::Result;
use crate::item;
use crate::location::Location;
use crate::signature::Signature;
use crate::tag;

type Id = i64;

#[derive(Debug)]
pub struct Item {
    pub id: Id,
    pub location: Location,
    pub signature: Signature,
}

#[derive(Debug)]
pub struct DuplicateItem {
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
pub struct ItemTag {
    pub id: Id,
    pub item_id: Id,
    pub tag_id: Id,
}

pub fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
            id          INTEGER PRIMARY KEY,
            location    TEXT NOT NULL UNIQUE,
            signature   TEXT NOT NULL UNIQUE
        )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS duplicate_items (
            id          INTEGER PRIMARY KEY,
            location    TEXT NOT NULL UNIQUE,
            signature   TEXT NOT NULL
        )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL UNIQUE
        )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_tags (
            id          INTEGER PRIMARY KEY,
            item_id     INTEGER NOT NULL,
            tag_id      INTEGER NOT NULL,
            FOREIGN KEY(item_id) REFERENCES items(id),
            FOREIGN KEY(tag_id) REFERENCES tags(id),
            UNIQUE(item_id, tag_id)
        )",
        params![],
    )?;
    Ok(())
}

impl Item {
    pub fn all(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, location, signature FROM items")?;
        Self::query_multi(&mut stmt, params![])
    }

    pub fn all_by_location(conn: &Connection, location: &Location) -> Result<Vec<Self>> {
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM items WHERE location = ?1")?;
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
            conn.prepare("SELECT id, location, signature FROM items WHERE location IN RARRAY(?1)")?;
        Self::query_multi(&mut stmt, params![location_values])
    }

    pub fn by_location(conn: &Connection, location: &Location) -> Result<Option<Self>> {
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM items WHERE location = ?1")?;
        Self::query_single(&mut stmt, params![location])
    }

    pub fn by_signature(conn: &Connection, signature: &Signature) -> Result<Option<Self>> {
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM items WHERE signature = ?1")?;
        Self::query_single(&mut stmt, params![signature])
    }

    pub fn insert(conn: &Connection, item: &item::Item) -> Result<Id> {
        conn.execute(
            "INSERT INTO items (location, signature) VALUES (?1, ?2)",
            params![item.location, item.signature],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn upsert(conn: &Connection, item: &item::Item) -> Result<Id> {
        conn.execute(
            "INSERT INTO items (location, signature) VALUES (?1, ?2)
                ON CONFLICT(location) DO UPDATE SET signature = ?2",
            params![item.location, item.signature],
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

impl DuplicateItem {
    pub fn all(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, location, signature FROM duplicate_items")?;
        Self::query_multi(&mut stmt, params![])
    }

    pub fn upsert(conn: &Connection, item: &item::Item) -> Result<Id> {
        conn.execute(
            "INSERT INTO duplicate_items (location, signature) VALUES (?1, ?2)
                ON CONFLICT(location) DO UPDATE SET signature = ?2",
            params![item.location, item.signature],
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
    pub fn all(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, name FROM tags")?;
        Self::query_multi(&mut stmt, params![])
    }

    pub fn all_by_names(conn: &Connection, names: &Vec<&str>) -> Result<Vec<Self>> {
        let name_values = Rc::new(
            names
                .iter()
                .copied()
                .map(|x| Value::from(x.to_string()))
                .collect::<Vec<Value>>(),
        );
        let mut stmt = conn.prepare("SELECT id, name FROM tags WHERE name IN RARRAY(?1)")?;
        Self::query_multi(&mut stmt, params![name_values])
    }

    pub fn upsert(conn: &Connection, tag: &tag::Tag) -> Result<Id> {
        conn.execute(
            "INSERT INTO tags (name) VALUES (?1)
                ON CONFLICT(name) DO NOTHING",
            params![tag.as_str()],
        )?;
        Ok(conn.last_insert_rowid())
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

impl ItemTag {
    pub fn all(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, item_id, tag_id FROM item_tags")?;
        Self::query_multi(&mut stmt, params![])
    }

    pub fn upsert(conn: &Connection, item_id: Id, tag_id: Id) -> Result<Id> {
        let mut stmt = conn.prepare(
            "INSERT INTO item_tags (item_id, tag_id) VALUES (?1, ?2)
                ON CONFLICT(item_id, tag_id) DO NOTHING",
        )?;
        stmt.execute(params![item_id, tag_id])?;
        Ok(conn.last_insert_rowid())
    }

    fn query_multi(stmt: &mut Statement, params: &[&dyn ToSql]) -> Result<Vec<Self>> {
        Ok(stmt
            .query_map(params, |row| {
                Ok(Self {
                    id: row.get(0)?,
                    item_id: row.get(1)?,
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

    #[test]
    fn basics() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        rusqlite::vtab::array::load_module(&conn)?;

        create_schema(&conn)?;

        assert!(Item::all(&conn)?.is_empty());
        assert!(DuplicateItem::all(&conn)?.is_empty());
        assert!(Tag::all(&conn)?.is_empty());
        assert!(ItemTag::all(&conn)?.is_empty());

        Item::insert(
            &conn,
            &item::Item::new(
                Location::try_from("LOCATION")?,
                Signature::try_from("SIGNATURE")?,
            ),
        )?;

        assert_eq!(1, Item::all(&conn)?.len());
        assert!(DuplicateItem::all(&conn)?.is_empty());

        Tag::upsert(&conn, &tag::Tag::from("tag0"))?;
        Tag::upsert(&conn, &tag::Tag::from("tag1"))?;
        Tag::upsert(&conn, &tag::Tag::from("tag2"))?;

        assert_eq!(3, Tag::all(&conn)?.len());

        let tags = Tag::all_by_names(&conn, &vec!["tag0", "tag1"])?;
        assert_eq!(2, tags.len());
        assert_eq!(1, tags[0].id);
        assert_eq!("tag0", tags[0].name);
        assert_eq!(2, tags[1].id);
        assert_eq!("tag1", tags[1].name);

        DuplicateItem::upsert(
            &conn,
            &item::Item::new(
                Location::try_from("LOCATION")?,
                Signature::try_from("SIGNATURE")?,
            ),
        )?;

        assert_eq!(1, Item::all(&conn)?.len());
        assert_eq!(1, DuplicateItem::all(&conn)?.len());

        Ok(())
    }
}
