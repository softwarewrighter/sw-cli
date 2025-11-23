use clap::{Arg, ArgAction};

#[must_use]
pub fn custom_args() -> Vec<Arg> {
    vec![
        Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .action(ArgAction::Append)
            .help("Input file(s)"),
        Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Output file"),
        Arg::new("pattern")
            .short('p')
            .long("pattern")
            .value_name("PATTERN")
            .help("Pattern to search for"),
        Arg::new("count")
            .long("count")
            .action(ArgAction::SetTrue)
            .help("Count lines in input"),
        Arg::new("reverse")
            .long("reverse")
            .action(ArgAction::SetTrue)
            .help("Reverse line order"),
    ]
}
