use crate::{CliConfig, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct GrepCommand;

impl Command for GrepCommand {
    fn can_handle(&self, config: &CliConfig) -> bool {
        config.pattern.is_some()
    }

    fn execute(&self, config: &CliConfig) -> Result<(), Box<dyn Error>> {
        let pattern = config.pattern.as_ref().unwrap();

        if let Some(inputs) = &config.base.input {
            for path in inputs {
                grep_file(path, pattern, config)?;
            }
        } else {
            grep_stdin(pattern, config)?;
        }
        Ok(())
    }
}

fn grep_file(path: &std::path::Path, pattern: &str, config: &CliConfig) -> Result<(), Box<dyn Error>> {
    if config.is_dry_run() {
        println!("Would search for '{}' in: {}", pattern, path.display());
        return Ok(());
    }

    for line in BufReader::new(File::open(path)?).lines() {
        let line = line?;
        if line.contains(pattern) {
            print_match(&line, Some(path), config);
        }
    }
    Ok(())
}

fn grep_stdin(pattern: &str, config: &CliConfig) -> Result<(), Box<dyn Error>> {
    for line in BufReader::new(io::stdin()).lines() {
        let line = line?;
        if line.contains(pattern) {
            print_match(&line, None, config);
        }
    }
    Ok(())
}

fn print_match(line: &str, path: Option<&std::path::Path>, config: &CliConfig) {
    if config.verbosity() > 0 {
        if let Some(p) = path {
            println!("{}: {}", p.display(), line);
        } else {
            println!("{}", line);
        }
    } else {
        println!("{}", line);
    }
}
