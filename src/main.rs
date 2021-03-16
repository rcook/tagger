#![feature(try_trait)]

mod error;
mod item;
mod walk;

use absolute_path::absolute_path;
use rusqlite::{params, Connection};
use std::env::{args, current_dir};
use std::path::Path;

use crate::error::Result;
use crate::item::{Item, ItemRecord};
use crate::walk::{ExtensionSet, SampleVisitor};

fn do_walk(start_dir: &Path) -> Result<()> {
    let visitor = SampleVisitor::new(ExtensionSet::new(&["aiff", "wav"]));

    println!("Scanning {}", start_dir.to_str()?);

    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE items (
            id      INTEGER PRIMARY KEY,
            path    TEXT NOT NULL,
            hash    TEXT NOT NULL,
            size    INTEGER NOT NULL
        )",
        params![],
    )?;

    visitor.visit(&start_dir, &|entry| {
        let p = entry.path();
        println!("Found {}", p.to_str()?);
        let item = Item::from_file(start_dir, &p)?;
        item.save(&conn)?;
        Ok(())
    })?;

    let mut stmt = conn.prepare("SELECT id, path, hash, size FROM items")?;
    let item_iter = stmt.query_map(params![], |row| {
        Ok(ItemRecord {
            id: row.get(0)?,
            path: row.get(1)?,
            hash: row.get(2)?,
            size: row.get(3)?,
        })
    })?;

    for item in item_iter {
        println!("Found item {:?}", item.unwrap());
    }

    Ok(())
}

fn main() -> Result<()> {
    let dir = current_dir()?;
    for arg in args().skip(1) {
        let p = absolute_path(&dir, Path::new(&arg))?;
        do_walk(&p)?;
    }
    Ok(())
}
