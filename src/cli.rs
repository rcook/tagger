use clap::{crate_authors, App, AppSettings, Arg, SubCommand};

pub mod command {
    pub const DUMP: &str = "dump";
    pub const REBUILD: &str = "rebuild";
    pub const REPORT: &str = "report";
}

pub mod arg {
    pub const DIR: &str = "dir";
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
                .help("Path to project directory")
                .long(DIR)
                .value_name("PROJECT-DIR")
                .takes_value(true)
                .default_value("."),
        )
        .subcommand(SubCommand::with_name(DUMP).about("Dump database"))
        .subcommand(SubCommand::with_name(REBUILD).about("Scan project and rebuild database"))
        .subcommand(SubCommand::with_name(REPORT).about("Scan project and show report"))
}
