use crate::DemoConfig;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use sw_cli::{CliConfig, Command};

pub struct GrepCommand;

impl Command for GrepCommand {
    fn can_handle(&self, config: &dyn CliConfig) -> bool {
        config
            .as_any()
            .downcast_ref::<DemoConfig>()
            .is_some_and(|c| c.pattern.is_some())
    }

    fn execute(&self, config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
        let demo_config = config.as_any().downcast_ref::<DemoConfig>().unwrap();
        let pattern = demo_config.pattern.as_ref().unwrap();

        if let Some(inputs) = &demo_config.input {
            for path in inputs {
                for line in BufReader::new(File::open(path)?).lines() {
                    let line = line?;
                    if line.contains(pattern) {
                        if config.verbosity() > 0 {
                            println!("{}: {}", path.display(), line);
                        } else {
                            println!("{line}");
                        }
                    }
                }
            }
        } else {
            for line in BufReader::new(io::stdin()).lines() {
                let line = line?;
                if line.contains(pattern) {
                    println!("{line}");
                }
            }
        }
        Ok(())
    }
}
