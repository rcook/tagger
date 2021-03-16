#![feature(try_trait)]

mod error;
mod item;
mod walk;

use absolute_path::absolute_path;
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use std::env::{args, current_dir};
use std::fs::File;
use std::io;
use std::path::Path;

use crate::error::Result;
use crate::item::{Hash, Item, ItemPath, ItemRecord};
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
        let mut f = File::open(&p)?;
        let size = i64::try_from(f.metadata()?.len())?;
        let mut hasher = Sha256::new();
        io::copy(&mut f, &mut hasher)?;
        let hash: Hash = hasher.finalize();
        let item_path = ItemPath::from(&start_dir, &p)?;
        let item = Item::new(item_path, hash, size);
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
