use crate::{CliConfig, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct ReverseCommand;

impl Command for ReverseCommand {
    fn can_handle(&self, config: &CliConfig) -> bool {
        config.reverse
    }

    fn execute(&self, config: &CliConfig) -> Result<(), Box<dyn Error>> {
        if let Some(inputs) = &config.base.input {
            for path in inputs {
                reverse_file(path, config)?;
            }
        } else {
            reverse_stdin(config)?;
        }
        Ok(())
    }
}

fn reverse_file(path: &std::path::Path, config: &CliConfig) -> Result<(), Box<dyn Error>> {
    if config.is_dry_run() {
        println!("Would reverse lines in: {}", path.display());
        return Ok(());
    }

    let lines: Result<Vec<_>, _> = BufReader::new(File::open(path)?).lines().collect();
    print_reversed(&lines?, config);
    Ok(())
}

fn reverse_stdin(config: &CliConfig) -> Result<(), Box<dyn Error>> {
    let lines: Result<Vec<_>, _> = BufReader::new(io::stdin()).lines().collect();
    print_reversed(&lines?, config);
    Ok(())
}

fn print_reversed(lines: &[String], _config: &CliConfig) {
    for line in lines.iter().rev() {
        println!("{}", line);
    }
}
