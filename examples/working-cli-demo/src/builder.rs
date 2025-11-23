use crate::config::{BaseConfig, CliConfig};
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;

pub fn build_cli() -> Command {
    Command::new("working-cli-demo")
        .disable_version_flag(true)
        .disable_help_flag(true)
        .about("Builder-Config-Dispatcher pattern demo")
        .args(standard_args())
        .args(custom_args())
}

fn standard_args() -> Vec<Arg> {
    vec![
        arg("version", 'V', "Show version information"),
        arg("help", 'h', "Show help information"),
        arg("verbose", 'v', "Increase output verbosity"),
        arg("dry-run", 'n', "Show what would be done"),
        arg("quiet", 'q', "Suppress non-essential output"),
        Arg::new("input").short('i').long("input").value_name("FILE")
            .action(ArgAction::Append).help("Input file(s)"),
        Arg::new("output").short('o').long("output").value_name("FILE")
            .help("Output file"),
    ]
}

fn custom_args() -> Vec<Arg> {
    vec![
        Arg::new("pattern").short('p').long("pattern").value_name("PATTERN")
            .help("Pattern to search for"),
        arg("count", '\0', "Count lines in input"),
        arg("reverse", '\0', "Reverse line order"),
    ]
}

fn arg(name: &'static str, short: char, help: &'static str) -> Arg {
    let mut arg = Arg::new(name).long(name).action(ArgAction::SetTrue).help(help);
    if short != '\0' { arg = arg.short(short); }
    arg
}

pub fn parse_config(matches: ArgMatches) -> CliConfig {
    CliConfig {
        base: parse_base_config(&matches),
        pattern: matches.get_one::<String>("pattern").cloned(),
        count: matches.get_flag("count"),
        reverse: matches.get_flag("reverse"),
    }
}

fn parse_base_config(matches: &ArgMatches) -> BaseConfig {
    BaseConfig {
        verbose: matches.get_flag("verbose"),
        dry_run: matches.get_flag("dry-run"),
        quiet: matches.get_flag("quiet"),
        help: matches.get_flag("help"),
        version: matches.get_flag("version"),
        input: matches.get_many::<String>("input")
            .map(|vals| vals.map(PathBuf::from).collect()),
        output: matches.get_one::<String>("output").map(PathBuf::from),
    }
}
