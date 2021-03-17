#![feature(try_trait)]
#![allow(dead_code)]

mod cli;
mod db;
mod error;
mod extension_set;
mod item;
mod location;
mod project;
mod sample_visitor;
mod signature;

use absolute_path::absolute_path;
use std::env::current_dir;

use crate::cli::{arg, command, make_app};
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
    for item in db::Item::all(&conn)? {
        println!("  ({}): {:?}, {:?}", item.id, item.location, item.signature);
    }

    println!("Tags:");
    for tag in db::Tag::all(&conn)? {
        println!("  ({}): {}", tag.id, tag.name);
    }

    println!("Item tags:");
    for item_tag in db::ItemTag::all(&conn)? {
        println!(
            "  ({}): {}, {}",
            item_tag.id, item_tag.item_id, item_tag.tag_id
        );
    }

    Ok(())
}

fn do_rebuild(project: &Project) -> Result<()> {
    let conn = project.open_db_connection()?;
    project
        .create_sample_visitor()
        .visit(&project.dir, &|entry| {
            db::Item::upsert(&conn, &Item::from_file(&project.dir, &entry.path())?)?;
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
            let item_by_location = db::Item::by_location(&conn, &item.location)?;
            match item_by_location {
                Some(x) => println!(
                    "With same location: {:?} signatures_match={}",
                    x,
                    x.signature.eq(&item.signature)
                ),
                None => println!("Item not found"),
            }
            let item_by_signature = db::Item::by_signature(&conn, &item.signature)?;
            match item_by_signature {
                Some(x) => println!(
                    "With same signature: {:?} locations_match={}",
                    x,
                    x.location.eq(&item.location)
                ),
                None => println!("Item not found"),
            }
            Ok(())
        })?;
    Ok(())
}
