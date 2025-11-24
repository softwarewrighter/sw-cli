use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use sw_cli::{cli_app, cli_command, dispatch, CliConfig};

// Generate config struct, builder, and parser in ~5 lines!
cli_app! {
    name: "macro-cli-demo",
    about: "Macro-based CLI demo",
    config: DemoConfig,
    fields: {
        input: Option<Vec<PathBuf>>, short = 'i', long = "input", help = "Input file(s)", action = Append,
        output: Option<PathBuf>, short = 'o', long = "output", help = "Output file",
        pattern: Option<String>, short = 'p', long = "pattern", help = "Pattern to search for",
        count: bool, long = "count", help = "Count lines in input",
        reverse: bool, long = "reverse", help = "Reverse line order",
    }
}

// Define count command in ~15 lines instead of ~59!
cli_command! {
    name: CountCommand,
    config: DemoConfig,
    can_handle: |c: &DemoConfig| c.count,
    execute: |config: &DemoConfig| {
        if let Some(inputs) = &config.input {
            for path in inputs {
                count_file(path, config)?;
            }
        } else {
            count_stdin(config)?;
        }
        Ok(())
    }
}

// Define grep command
cli_command! {
    name: GrepCommand,
    config: DemoConfig,
    can_handle: |c: &DemoConfig| c.pattern.is_some(),
    execute: |config: &DemoConfig| {
        let pattern = config.pattern.as_ref().unwrap();
        if let Some(inputs) = &config.input {
            for path in inputs {
                grep_file(path, pattern, config)?;
            }
        } else {
            grep_stdin(pattern, config)?;
        }
        Ok(())
    }
}

// Define reverse command
cli_command! {
    name: ReverseCommand,
    config: DemoConfig,
    can_handle: |c: &DemoConfig| c.reverse,
    execute: |config: &DemoConfig| {
        if let Some(inputs) = &config.input {
            for path in inputs {
                reverse_file(path, config)?;
            }
        } else {
            reverse_stdin(config)?;
        }
        Ok(())
    }
}

// Define default copy command
cli_command! {
    name: CopyCommand,
    config: DemoConfig,
    can_handle: |_c: &DemoConfig| true,
    execute: |config: &DemoConfig| {
        if let Some(inputs) = &config.input {
            for path in inputs {
                copy_file(path)?;
            }
        } else {
            copy_stdin()?;
        }
        Ok(())
    }
}

fn main() {
    let matches = build_cli().get_matches();
    let config = parse_config(&matches);

    let dispatcher = dispatch!(CountCommand, GrepCommand, ReverseCommand, CopyCommand);

    if let Err(e) = dispatcher.dispatch(&config) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

// Helper functions (same as working-cli-demo)

fn count_file(path: &std::path::Path, config: &DemoConfig) -> Result<(), Box<dyn std::error::Error>> {
    if config.is_dry_run() {
        println!("Would count lines in: {}", path.display());
        return Ok(());
    }

    if config.verbosity() > 0 {
        eprintln!("Processing: {}", path.display());
    }

    let count = BufReader::new(File::open(path)?).lines().count();
    if config.verbosity() > 0 {
        println!("{}: {} lines", path.display(), count);
    } else {
        println!("{count}");
    }
    Ok(())
}

fn count_stdin(config: &DemoConfig) -> Result<(), Box<dyn std::error::Error>> {
    if config.verbosity() > 0 {
        eprintln!("Reading from stdin...");
    }
    let count = BufReader::new(io::stdin()).lines().count();
    println!("{count}");
    Ok(())
}

fn grep_file(path: &std::path::Path, pattern: &str, config: &DemoConfig) -> Result<(), Box<dyn std::error::Error>> {
    if config.is_dry_run() {
        println!("Would search for '{}' in: {}", pattern, path.display());
        return Ok(());
    }

    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.contains(pattern) {
            if config.verbosity() > 0 {
                println!("{}: {}", path.display(), line);
            } else {
                println!("{line}");
            }
        }
    }
    Ok(())
}

fn grep_stdin(pattern: &str, config: &DemoConfig) -> Result<(), Box<dyn std::error::Error>> {
    for line in BufReader::new(io::stdin()).lines() {
        let line = line?;
        if line.contains(pattern) {
            if config.verbosity() > 0 {
                println!("stdin: {line}");
            } else {
                println!("{line}");
            }
        }
    }
    Ok(())
}

fn reverse_file(path: &std::path::Path, config: &DemoConfig) -> Result<(), Box<dyn std::error::Error>> {
    if config.is_dry_run() {
        println!("Would reverse lines in: {}", path.display());
        return Ok(());
    }

    let file = File::open(path)?;
    let lines: Result<Vec<_>, _> = BufReader::new(file).lines().collect();
    for line in lines?.into_iter().rev() {
        println!("{line}");
    }
    Ok(())
}

fn reverse_stdin(_config: &DemoConfig) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Result<Vec<_>, _> = BufReader::new(io::stdin()).lines().collect();
    for line in lines?.into_iter().rev() {
        println!("{line}");
    }
    Ok(())
}

fn copy_file(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn copy_stdin() -> Result<(), Box<dyn std::error::Error>> {
    for line in BufReader::new(io::stdin()).lines() {
        println!("{}", line?);
    }
    Ok(())
}
