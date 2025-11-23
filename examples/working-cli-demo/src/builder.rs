use crate::args;
use crate::config::DemoConfig;
use clap::{ArgMatches, Command};
use std::path::PathBuf;

#[must_use]
pub fn build_cli() -> Command {
    Command::new("working-cli-demo")
        .disable_version_flag(true)
        .disable_help_flag(true)
        .about("Builder-Config-Dispatcher pattern demo")
        .args(sw_cli::builder::standard_args())
        .args(args::custom_args())
}

pub fn parse_config(matches: &ArgMatches) -> DemoConfig {
    DemoConfig {
        base: sw_cli::builder::parse_base_config(matches),
        input: matches
            .get_many::<String>("input")
            .map(|vals| vals.map(PathBuf::from).collect()),
        output: matches.get_one::<String>("output").map(PathBuf::from),
        pattern: matches.get_one::<String>("pattern").cloned(),
        count: matches.get_flag("count"),
        reverse: matches.get_flag("reverse"),
    }
}
