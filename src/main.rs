#![feature(try_trait)]

mod cli;
mod data;
mod error;
mod item;
mod location;
mod project;
mod signature;
mod walk;

use absolute_path::absolute_path;
use rusqlite::params;
use std::env::current_dir;

use crate::cli::{arg, command, make_app};
use crate::data::Item2;
use crate::error::{user_error_result, Result};
use crate::item::Item;
use crate::project::Project;

fn main() -> Result<()> {
    let matches = make_app().get_matches();

    let project = match matches.value_of(arg::DIR) {
        Some(d) => Project::from_dir(absolute_path(current_dir()?, d)?),
        None => return user_error_result("No project directory specified"),
    };

    match matches.subcommand() {
        (command::DUMP, _submatches) => do_dump(&project),
        (command::REBUILD, _submatches) => do_rebuild(&project),
        (command::REPORT, _submatches) => do_report(&project),
        _ => {
            println!("Not implemented!");
            Ok(())
        }
    }
}

fn do_dump(project: &Project) -> Result<()> {
    let conn = project.open_db_connection()?;

    println!("Items:");
    let mut stmt = conn.prepare("SELECT id, location, signature FROM items")?;
    let items_iter = stmt.query_map(params![], |row| {
        Ok(data::Item {
            id: row.get(0)?,
            location: row.get(1)?,
            signature: row.get(2)?,
        })
    })?;
    for item in items_iter {
        println!("  {:?}", item.unwrap());
    }

    println!("Tags:");
    let mut stmt = conn.prepare("SELECT id, name FROM tags")?;
    let tags_iter = stmt.query_map(params![], |row| {
        Ok(data::Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    for tag in tags_iter {
        println!("  {:?}", tag.unwrap());
    }

    println!("Item tags:");
    let mut stmt = conn.prepare("SELECT id, item_id, tag_id FROM item_tags")?;
    let item_tags_iter = stmt.query_map(params![], |row| {
        Ok(data::ItemTag {
            id: row.get(0)?,
            item_id: row.get(1)?,
            tag_id: row.get(2)?,
        })
    })?;
    for item_tag in item_tags_iter {
        println!("  {:?}", item_tag.unwrap());
    }

    Ok(())
}

fn do_rebuild(project: &Project) -> Result<()> {
    let conn = project.open_db_connection()?;
    project
        .create_sample_visitor()
        .visit(&project.dir, &|entry| {
            Item::from_file(&project.dir, &entry.path())?.upsert(&conn)?;
            Ok(())
        })?;
    Ok(())
}

fn do_report(project: &Project) -> Result<()> {
    println!("Scanning {}", project.dir.to_str()?);

    let conn = project.open_db_connection()?;
    project
        .create_sample_visitor()
        .visit(&project.dir, &|entry| {
            let p = entry.path();
            println!("Found {}", p.to_str()?);
            let item = Item::from_file(&project.dir, &p)?;
            let item_by_location = Item2::by_location(&conn, &item.location)?;
            match item_by_location {
                Some(x) => println!(
                    "With same location: {:?} signatures_match={}",
                    x,
                    x.signatures_eq(&item.signature)
                ),
                None => println!("Item not found"),
            }
            let item_by_signature = Item2::by_signature(&conn, &item.signature)?;
            match item_by_signature {
                Some(x) => println!(
                    "With same signature: {:?} locations_match={}",
                    x,
                    x.locations_eq(&item.location)
                ),
                None => println!("Item not found"),
            }
            Ok(())
        })?;
    Ok(())
}
