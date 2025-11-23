use crate::{CliConfig, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct CopyCommand;

impl Command for CopyCommand {
    fn can_handle(&self, _config: &CliConfig) -> bool {
        true
    }

    fn execute(&self, config: &CliConfig) -> Result<(), Box<dyn Error>> {
        if let Some(inputs) = &config.base.input {
            for path in inputs {
                copy_file(path, config)?;
            }
        } else {
            copy_stdin(config)?;
        }
        Ok(())
    }
}

fn copy_file(path: &std::path::Path, config: &CliConfig) -> Result<(), Box<dyn Error>> {
    if config.is_dry_run() {
        println!("Would copy: {}", path.display());
        return Ok(());
    }

    for line in BufReader::new(File::open(path)?).lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn copy_stdin(_config: &CliConfig) -> Result<(), Box<dyn Error>> {
    for line in BufReader::new(io::stdin()).lines() {
        println!("{}", line?);
    }
    Ok(())
}
