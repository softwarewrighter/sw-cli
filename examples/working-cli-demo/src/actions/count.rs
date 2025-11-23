use crate::DemoConfig;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use sw_cli::{CliConfig, Command};

pub struct CountCommand;

impl Command for CountCommand {
    fn can_handle(&self, config: &dyn CliConfig) -> bool {
        config
            .as_any()
            .downcast_ref::<DemoConfig>()
            .is_some_and(|c| c.count)
    }

    fn execute(&self, config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
        let demo_config = config.as_any().downcast_ref::<DemoConfig>().unwrap();

        if let Some(inputs) = &demo_config.input {
            for path in inputs {
                count_file(path, config)?;
            }
        } else {
            count_stdin(config)?;
        }
        Ok(())
    }
}

fn count_file(path: &std::path::Path, config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
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

#[allow(clippy::unnecessary_wraps)]
fn count_stdin(config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
    if config.verbosity() > 0 {
        eprintln!("Reading from stdin...");
    }
    let count = BufReader::new(io::stdin()).lines().count();
    println!("{count}");
    Ok(())
}
