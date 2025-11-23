use crate::DemoConfig;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use sw_cli::{CliConfig, Command};

pub struct CopyCommand;

impl Command for CopyCommand {
    fn can_handle(&self, _config: &dyn CliConfig) -> bool {
        true
    }

    fn execute(&self, config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
        let demo_config = config.as_any().downcast_ref::<DemoConfig>().unwrap();

        if let Some(inputs) = &demo_config.input {
            for path in inputs {
                for line in BufReader::new(File::open(path)?).lines() {
                    println!("{}", line?);
                }
            }
        } else {
            for line in BufReader::new(io::stdin()).lines() {
                println!("{}", line?);
            }
        }
        Ok(())
    }
}
