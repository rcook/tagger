#![feature(try_trait)]

mod error;
mod item;
mod walk;

use absolute_path::absolute_path;
use rusqlite::{params, Connection};
use std::env::{args, current_dir};
use std::path::Path;

use crate::error::Result;
use crate::item::{Item, Item2};
use crate::walk::{ExtensionSet, SampleVisitor};

#[derive(Debug)]
pub struct Record {
    pub id: i32,
    pub location: String,
    pub signature: String,
}

#[allow(dead_code)]
fn do_report(conn: &Connection, start_dir: &Path) -> Result<()> {
    let visitor = SampleVisitor::new(ExtensionSet::new(&["aiff", "wav"]));

    println!("Scanning {}", start_dir.to_str()?);

    visitor.visit(&start_dir, &|entry| {
        let p = entry.path();
        println!("Found {}", p.to_str()?);
        let item = Item::from_file(start_dir, &p)?;
        let item_by_location = Item2::by_location(&conn, &item)?;
        match item_by_location {
            Some(x) => println!(
                "With same location: {:?} signatures_match={}",
                x,
                x.signatures_eq(&item)
            ),
            None => println!("Item not found"),
        }
        let item_by_signature = Item2::by_signature(&conn, &item)?;
        match item_by_signature {
            Some(x) => println!(
                "With same signature: {:?} locations_match={}",
                x,
                x.locations_eq(&item)
            ),
            None => println!("Item not found"),
        }
        Ok(())
    })?;

    Ok(())
}

#[allow(dead_code)]
fn do_force_update(conn: &Connection, start_dir: &Path) -> Result<()> {
    let visitor = SampleVisitor::new(ExtensionSet::new(&["aiff", "wav"]));

    println!("Scanning {}", start_dir.to_str()?);

    visitor.visit(&start_dir, &|entry| {
        let p = entry.path();
        println!("Found {}", p.to_str()?);
        let item = Item::from_file(start_dir, &p)?;
        item.upsert(&conn)?;
        Ok(())
    })?;

    let mut stmt = conn.prepare("SELECT id, location, signature FROM items")?;
    let record_iter = stmt.query_map(params![], |row| {
        Ok(Record {
            id: row.get(0)?,
            location: row.get(1)?,
            signature: row.get(2)?,
        })
    })?;

    for record in record_iter {
        println!("Found record {:?}", record.unwrap());
    }

    Ok(())
}

fn main() -> Result<()> {
    let base_dir = current_dir()?;
    for arg in args().skip(1) {
        let project_dir = absolute_path(&base_dir, Path::new(&arg))?;
        let db_path = project_dir.join("tagger.db");

        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id          INTEGER PRIMARY KEY,
                location    TEXT NOT NULL UNIQUE,
                signature   TEXT NOT NULL UNIQUE
            )",
            params![],
        )?;

        do_report(&conn, &project_dir)?;
        //do_force_update(&conn, &project_dir)?;
    }
    Ok(())
}
