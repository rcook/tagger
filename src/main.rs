#![feature(try_trait)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod action;
mod cli;
mod db;
mod db_migrations;
mod error;
mod item;
mod location;
mod media_path_checker;
mod project;
mod sample_visitor;
mod signature;
mod tag;

use absolute_path::absolute_path;
use clap::ArgMatches;
use colored::Colorize;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::action::{
    do_check_database, do_check_file_system, do_default, do_delete_tag, do_dump, do_scan,
    do_search, do_tag,
};
use crate::cli::{arg, command, make_app};
use crate::error::{user_error_result, Error, Result};
use crate::project::Project;
use crate::tag::Tag;

#[cfg(windows)]
use colored::control::set_virtual_terminal;

fn main() {
    exit(match main_inner() {
        Ok(()) => 0,
        Err(Error::User(message)) => {
            println!("{}", format!("Error: {}", message).bright_red());
            1
        }
        Err(Error::Internal(facility, message)) => {
            println!(
                "{}",
                format!("Internal ({}): {}", facility, message).red().bold()
            );
            2
        }
    })
}

fn reset_terminal() {
    #[cfg(windows)]
    set_virtual_terminal(true).expect("set_virtual_terminal failed");
}

fn main_inner() -> Result<()> {
    reset_terminal();

    let matches = make_app().get_matches();

    let working_dir = current_dir()?;

    let project = match matches.value_of(arg::DIR) {
        Some(d) => Project::from_dir(absolute_path(&working_dir, d)?),
        None => return user_error_result("No project directory specified"),
    };

    match matches.subcommand() {
        (command::CHECK_DATABASE, _submatches) => do_check_database(&project),
        (command::CHECK_FILE_SYSTEM, _submatches) => do_check_file_system(&project),
        (command::DEFAULT, _submatches) => do_default(&project),
        (command::DELETE_TAG, Some(submatches)) => do_delete_tag(&project, &get_tags(submatches)?),
        (command::DUMP, _submatches) => do_dump(&project),
        (command::SCAN, _submatches) => do_scan(&project),
        (command::SEARCH, Some(submatches)) => do_search(&project, &get_tags(submatches)?),
        (command::TAG, Some(submatches)) => do_tag(
            &project,
            &get_tags(submatches)?,
            &get_paths(&working_dir, submatches)?,
        ),
        (c, _submatches) => panic!("Subcommand \"{}\" not implemented", c),
    }
}

fn get_tags<'a>(submatches: &'a ArgMatches) -> Result<Vec<Tag<'a>>> {
    Ok(submatches
        .values_of(arg::TAG)?
        .map(|x| Tag::from(x))
        .collect())
}

fn get_paths(working_dir: &impl AsRef<Path>, submatches: &ArgMatches) -> Result<Vec<PathBuf>> {
    Ok(submatches
        .values_of(arg::PATHS)?
        .map(|x| absolute_path(&working_dir, x))
        .collect::<std::io::Result<_>>()?)
}
