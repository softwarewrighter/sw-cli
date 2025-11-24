use sw_cli::{cli_app, cli_command, dispatch, CliConfig};

// Define our CLI with just 6 lines!
cli_app! {
    name: "mini-cli-demo",
    about: "Minimal CLI example using sw-cli macros",
    config: MiniConfig,
    fields: {
        text: Option<String>, short = 't', long = "text", help = "Text to process",
        uppercase: bool, short = 'u', long = "uppercase", help = "Convert to uppercase",
        repeat: Option<usize>, short = 'r', long = "repeat", help = "Repeat N times",
    }
}

// Define commands with ~10 lines each

cli_command! {
    name: UppercaseCommand,
    config: MiniConfig,
    can_handle: |c: &MiniConfig| c.uppercase,
    execute: |config: &MiniConfig| {
        let text = config.text.as_deref().unwrap_or("Hello, World!");

        if config.is_dry_run() {
            println!("Would uppercase: {}", text);
        } else {
            if config.verbosity() > 0 {
                println!("Converting to uppercase...");
            }
            println!("{}", text.to_uppercase());
        }
        Ok(())
    }
}

cli_command! {
    name: RepeatCommand,
    config: MiniConfig,
    can_handle: |c: &MiniConfig| c.repeat.is_some(),
    execute: |config: &MiniConfig| {
        let text = config.text.as_deref().unwrap_or("Hello, World!");
        let count = config.repeat.unwrap_or(1);

        if config.is_dry_run() {
            println!("Would repeat '{}' {} times", text, count);
        } else {
            if config.verbosity() > 0 {
                println!("Repeating {} times...", count);
            }
            for _ in 0..count {
                println!("{}", text);
            }
        }
        Ok(())
    }
}

cli_command! {
    name: EchoCommand,
    config: MiniConfig,
    can_handle: |_c: &MiniConfig| true,
    execute: |config: &MiniConfig| {
        let text = config.text.as_deref().unwrap_or("Hello, World!");

        if config.verbosity() > 0 {
            println!("Echoing text...");
        }
        println!("{}", text);
        Ok(())
    }
}

fn main() {
    let matches = build_cli().get_matches();
    let config = parse_config(&matches);

    let dispatcher = dispatch!(UppercaseCommand, RepeatCommand, EchoCommand);

    if let Err(e) = dispatcher.dispatch(&config) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
