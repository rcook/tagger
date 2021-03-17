#![feature(try_trait)]
#![allow(dead_code)]

mod action;
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

use crate::action::{do_dump, do_rebuild, do_report};
use crate::cli::{arg, command, make_app};
use crate::error::{user_error_result, Result};
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
