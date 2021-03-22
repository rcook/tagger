use clap::{crate_authors, App, AppSettings, Arg, SubCommand};

pub mod command {
    pub const CHECK_DATABASE: &str = "checkdb";
    pub const CHECK_FILE_SYSTEM: &str = "checkfs";
    pub const DEFAULT: &str = "";
    pub const DELETE_TAG: &str = "del";
    pub const SCAN: &str = "scan";
    pub const SEARCH: &str = "search";
    pub const TAG: &str = "tag";

    // New commands
    pub const LIST_FILES: &str = "listfiles";
    pub const LIST_TAGS: &str = "listtags";
}

pub mod arg {
    pub const DIR: &str = "dir";
    pub const PATHS: &str = "paths";
    pub const TAG: &str = "tag";

    // New args
    pub const LIKE: &str = "like";
}

pub fn make_app<'a, 'b>() -> App<'a, 'b> {
    let t = Arg::with_name(arg::TAG)
        .help("Tag")
        .value_name("TAG")
        .takes_value(true)
        .long(arg::TAG)
        .multiple(true)
        .number_of_values(1)
        .required(true)
        .min_values(1);

    App::new("Richard's Tagging Tool")
        .author(crate_authors!())
        .about("Maintains database of tags for files")
        .setting(AppSettings::TrailingVarArg)
        .version(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name(arg::DIR)
                .help("Project directory")
                .value_name("PROJECT-DIR")
                .takes_value(true)
                .long(arg::DIR)
                .default_value("."),
        )
        .subcommand(
            SubCommand::with_name(command::CHECK_DATABASE)
                .about("Scan project and database for inconsistencies"),
        )
        .subcommand(
            SubCommand::with_name(command::CHECK_FILE_SYSTEM)
                .about("Scan project and directory for inconsistencies"),
        )
        .subcommand(
            SubCommand::with_name(command::DELETE_TAG)
                .about("Delete tag")
                .arg(&t),
        )
        .subcommand(
            SubCommand::with_name(command::SCAN)
                .about("Scan project directory and populate database"),
        )
        .subcommand(
            SubCommand::with_name(command::SEARCH)
                .about("Search files by tag")
                .arg(&t),
        )
        .subcommand(
            SubCommand::with_name(command::TAG)
                .about("Tag files")
                .arg(t)
                .arg(
                    Arg::with_name(arg::PATHS)
                        .help("Files")
                        .value_name("PATHS")
                        .takes_value(true)
                        .multiple(true)
                        .required(true)
                        .min_values(1),
                ),
        )
        // New commands
        .subcommand(
            SubCommand::with_name(command::LIST_FILES)
                .about("Show files in database")
                .arg(
                    Arg::with_name(arg::LIKE)
                        .help("Match file locations using SQL-style LIKE filter")
                        .value_name("LIKE")
                        .takes_value(true)
                        .long(arg::LIKE)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name(command::LIST_TAGS)
                .about("Show tags in database")
                .arg(
                    Arg::with_name(arg::LIKE)
                        .help("Match tag names using SQL-style LIKE filter")
                        .value_name("LIKE")
                        .takes_value(true)
                        .long(arg::LIKE)
                        .required(false),
                ),
        )
}
