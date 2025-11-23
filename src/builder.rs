use crate::config::{BaseConfig, HelpType};
use clap::{Arg, ArgAction, ArgMatches};

/// Creates standard flags for all Software Wrighter CLIs
#[must_use]
pub fn standard_args() -> Vec<Arg> {
    vec![
        Arg::new("version")
            .short('V')
            .long("version")
            .action(ArgAction::SetTrue)
            .help("Show version information"),
        Arg::new("help-short")
            .short('h')
            .action(ArgAction::SetTrue)
            .help("Show short help (quick reference)"),
        Arg::new("help-long")
            .long("help")
            .action(ArgAction::SetTrue)
            .help("Show detailed help with examples"),
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(ArgAction::SetTrue)
            .help("Increase output verbosity"),
        Arg::new("dry-run")
            .short('n')
            .long("dry-run")
            .action(ArgAction::SetTrue)
            .help("Show what would be done without doing it"),
    ]
}

/// Parse standard flags from `ArgMatches`
#[must_use]
pub fn parse_base_config(matches: &ArgMatches) -> BaseConfig {
    let help = if matches.get_flag("help-long") {
        HelpType::Long
    } else if matches.get_flag("help-short") {
        HelpType::Short
    } else {
        HelpType::None
    };

    BaseConfig {
        verbose: matches.get_flag("verbose"),
        dry_run: matches.get_flag("dry-run"),
        help,
        version: matches.get_flag("version"),
    }
}
