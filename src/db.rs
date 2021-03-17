use rusqlite::types::ToSql;
use rusqlite::{params, Connection, OptionalExtension, Statement};

use crate::error::Result;
use crate::item;
use crate::location::Location;
use crate::signature::Signature;

#[derive(Debug)]
pub struct Item {
    pub id: i32,
    pub location: Location,
    pub signature: Signature,
}

#[derive(Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
pub struct ItemTag {
    pub id: i32,
    pub item_id: i32,
    pub tag_id: i32,
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
            FOREIGN KEY(tag_id) REFERENCES tags(id)
        )",
        params![],
    )?;
    Ok(())
}

impl Item {
    #[allow(dead_code)]
    pub fn insert(conn: &Connection, item: &item::Item) -> Result<()> {
        conn.execute(
            "INSERT INTO items (location, signature) VALUES (?1, ?2)",
            params![item.location, item.signature],
        )?;
        Ok(())
    }

    pub fn upsert(conn: &Connection, item: &item::Item) -> Result<()> {
        conn.execute(
            "INSERT INTO items (location, signature) VALUES (?1, ?2)
                ON CONFLICT(location) DO UPDATE SET signature = ?2",
            params![item.location, item.signature],
        )?;
        Ok(())
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

    pub fn all(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, location, signature FROM items")?;
        Self::query_multi(&mut stmt, params![])
    }

    #[allow(dead_code)]
    pub fn all_by_location(conn: &Connection, location: &Location) -> Result<Vec<Self>> {
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM items WHERE location = ?1")?;
        Self::query_multi(&mut stmt, params![location])
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
        let items_iter = stmt.query_map(params, |row| {
            Ok(Self {
                id: row.get(0)?,
                location: row.get(1)?,
                signature: row.get(2)?,
            })
        })?;

        let mut items = Vec::new();
        for item in items_iter {
            items.push(item?)
        }

        Ok(items)
    }
}

impl Tag {
    pub fn all(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, name FROM tags")?;
        Self::query_multi(&mut stmt, params![])
    }

    fn query_multi(stmt: &mut Statement, params: &[&dyn ToSql]) -> Result<Vec<Self>> {
        let tags_iter = stmt.query_map(params, |row| {
            Ok(Self {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut tags = Vec::new();
        for tag in tags_iter {
            tags.push(tag?)
        }

        Ok(tags)
    }
}

impl ItemTag {
    pub fn all(conn: &Connection) -> Result<Vec<Self>> {
        let mut stmt = conn.prepare("SELECT id, item_id, tag_id FROM item_tags")?;
        Self::query_multi(&mut stmt, params![])
    }

    fn query_multi(stmt: &mut Statement, params: &[&dyn ToSql]) -> Result<Vec<Self>> {
        let item_tags_iter = stmt.query_map(params, |row| {
            Ok(Self {
                id: row.get(0)?,
                item_id: row.get(1)?,
                tag_id: row.get(2)?,
            })
        })?;

        let mut item_tags = Vec::new();
        for item_tag in item_tags_iter {
            item_tags.push(item_tag?)
        }

        Ok(item_tags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_schema(&conn)?;
        Item::all(&conn)?;
        Tag::all(&conn)?;
        ItemTag::all(&conn)?;
        Ok(())
    }
}
