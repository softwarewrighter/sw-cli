use crate::DemoConfig;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use sw_cli::{CliConfig, Command};

pub struct ReverseCommand;

impl Command for ReverseCommand {
    fn can_handle(&self, config: &dyn CliConfig) -> bool {
        config
            .as_any()
            .downcast_ref::<DemoConfig>()
            .is_some_and(|c| c.reverse)
    }

    fn execute(&self, config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
        let demo_config = config.as_any().downcast_ref::<DemoConfig>().unwrap();

        if let Some(inputs) = &demo_config.input {
            for path in inputs {
                let lines: Result<Vec<_>, _> = BufReader::new(File::open(path)?).lines().collect();
                for line in lines?.iter().rev() {
                    println!("{line}");
                }
            }
        } else {
            let lines: Result<Vec<_>, _> = BufReader::new(io::stdin()).lines().collect();
            for line in lines?.iter().rev() {
                println!("{line}");
            }
        }
        Ok(())
    }
}
