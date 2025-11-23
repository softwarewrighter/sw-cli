use working_cli_demo::*;
use working_cli_demo::actions::*;

fn main() {
    let matches = build_cli().get_matches();
    let config = builder::parse_config(matches);

    let help_text = build_cli().render_help().to_string();

    let dispatcher = Dispatcher::new()
        .register(VersionCommand)
        .register(HelpCommand::new(help_text))
        .register(CountCommand)
        .register(GrepCommand)
        .register(ReverseCommand)
        .register(CopyCommand);

    if let Err(e) = dispatcher.dispatch(&config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
