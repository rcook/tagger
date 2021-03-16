#![feature(try_trait)]

mod cli;
mod error;
mod item;
mod walk;

use absolute_path::absolute_path;
use clap::ArgMatches;
use rusqlite::{params, Connection};
use std::env::current_dir;
use std::path::Path;

use crate::cli::{arg, command, make_app};
use crate::error::{user_error_result, Result};
use crate::item::{Item, Item2};
use crate::walk::{ExtensionSet, SampleVisitor};

#[derive(Debug)]
pub struct Record {
    pub id: i32,
    pub location: String,
    pub signature: String,
}

fn main() -> Result<()> {
    let matches = make_app().get_matches();

    let project_dir = match matches.value_of(arg::DIR) {
        Some(d) => absolute_path(current_dir()?, d)?,
        None => return user_error_result("No project directory specified"),
    };

    match matches.subcommand() {
        (command::REBUILD, submatches) => do_rebuild(&project_dir, submatches),
        (command::REPORT, submatches) => do_report(&project_dir, submatches),
        _ => {
            println!("Not implemented!");
            Ok(())
        }
    }
}

fn do_rebuild(project_dir: &Path, _submatches: Option<&ArgMatches>) -> Result<()> {
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

    do_rebuild_helper(&conn, &project_dir)
}

fn do_rebuild_helper(conn: &Connection, start_dir: &Path) -> Result<()> {
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

fn do_report(project_dir: &Path, _submatches: Option<&ArgMatches>) -> Result<()> {
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

    do_report_helper(&conn, &project_dir)
}

fn do_report_helper(conn: &Connection, start_dir: &Path) -> Result<()> {
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
