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

    #[allow(dead_code)]
    pub fn all_by_location(conn: &Connection, location: &Location) -> Result<Vec<Self>> {
        let mut stmt =
            conn.prepare("SELECT id, location, signature FROM items WHERE location = ?1")?;
        let record_iter = stmt.query_map(params![location], |row| {
            Ok(Self {
                id: row.get(0)?,
                location: row.get(1)?,
                signature: row.get(2)?,
            })
        })?;

        let mut items = Vec::new();
        for record in record_iter {
            items.push(record?)
        }

        Ok(items)
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
}
