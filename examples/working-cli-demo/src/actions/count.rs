use crate::{CliConfig, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct CountCommand;

impl Command for CountCommand {
    fn can_handle(&self, config: &CliConfig) -> bool {
        config.count
    }

    fn execute(&self, config: &CliConfig) -> Result<(), Box<dyn Error>> {
        if let Some(inputs) = &config.base.input {
            for path in inputs {
                count_file(path, config)?;
            }
        } else {
            count_stdin(config)?;
        }
        Ok(())
    }
}

fn count_file(path: &std::path::Path, config: &CliConfig) -> Result<(), Box<dyn Error>> {
    if config.is_dry_run() {
        println!("Would count lines in: {}", path.display());
        return Ok(());
    }

    if config.verbosity() > 0 {
        eprintln!("Processing: {}", path.display());
    }

    let count = BufReader::new(File::open(path)?).lines().count();
    print_count(count, Some(path), config);
    Ok(())
}

fn count_stdin(config: &CliConfig) -> Result<(), Box<dyn Error>> {
    if config.verbosity() > 0 {
        eprintln!("Reading from stdin...");
    }
    let count = BufReader::new(io::stdin()).lines().count();
    print_count(count, None, config);
    Ok(())
}

fn print_count(count: usize, path: Option<&std::path::Path>, config: &CliConfig) {
    if config.verbosity() > 0 {
        if let Some(p) = path {
            println!("{}: {} lines", p.display(), count);
        } else {
            println!("Total lines: {}", count);
        }
    } else {
        println!("{}", count);
    }
}
