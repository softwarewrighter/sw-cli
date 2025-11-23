use sw_cli::dispatch;
use working_cli_demo::actions::{CopyCommand, CountCommand, GrepCommand, ReverseCommand};
use working_cli_demo::{build_cli, parse_config};

fn main() {
    let matches = build_cli().get_matches();
    let config = parse_config(&matches);

    let dispatcher = dispatch!(CountCommand, GrepCommand, ReverseCommand, CopyCommand);

    if let Err(e) = dispatcher.dispatch(&config) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
