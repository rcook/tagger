use clap::{crate_authors, App, AppSettings, Arg, SubCommand};

pub mod command {
    pub const DEFAULT: &str = "";
    pub const DUMP: &str = "dump";
    pub const REBUILD: &str = "rebuild";
    pub const REPORT: &str = "report";
    pub const SEARCH: &str = "search";
    pub const TAG: &str = "tag";
}

pub mod arg {
    pub const DIR: &str = "dir";
    pub const PATHS: &str = "paths";
    pub const TAG: &str = "tag";
}

pub fn make_app<'a, 'b>() -> App<'a, 'b> {
    use arg::*;
    use command::*;

    App::new("Richard's Tagging Tool")
        .author(crate_authors!())
        .about("Maintains database of tags for files")
        .setting(AppSettings::TrailingVarArg)
        .version(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name(DIR)
                .help("Project directory")
                .value_name("PROJECT-DIR")
                .takes_value(true)
                .long(DIR)
                .default_value("."),
        )
        .subcommand(SubCommand::with_name(DUMP).about("Dump database"))
        .subcommand(SubCommand::with_name(REBUILD).about("Scan project and rebuild database"))
        .subcommand(SubCommand::with_name(REPORT).about("Scan project and show report"))
        .subcommand(
            SubCommand::with_name(SEARCH)
                .about("Search files by tag")
                .arg(
                    Arg::with_name(arg::TAG)
                        .help("Tag")
                        .value_name("TAG")
                        .takes_value(true)
                        .long(arg::TAG)
                        .multiple(true)
                        .number_of_values(1)
                        .required(true)
                        .min_values(1),
                ),
        )
        .subcommand(
            SubCommand::with_name(command::TAG)
                .about("Tag files")
                .arg(
                    Arg::with_name(arg::TAG)
                        .help("Tag")
                        .value_name("TAG")
                        .takes_value(true)
                        .long(arg::TAG)
                        .multiple(true)
                        .number_of_values(1)
                        .required(true)
                        .min_values(1),
                )
                .arg(
                    Arg::with_name(PATHS)
                        .help("Files")
                        .value_name("PATHS")
                        .takes_value(true)
                        .multiple(true)
                        .required(true)
                        .min_values(1),
                ),
        )
}
