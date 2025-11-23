use crate::command::Command;
use crate::config::CliConfig;
use std::error::Error;

pub struct HelpCommand {
    short_help: String,
    long_help: String,
}

impl HelpCommand {
    #[must_use]
    pub fn new(short_help: String, long_help: String) -> Self {
        Self {
            short_help,
            long_help,
        }
    }
}

impl Command for HelpCommand {
    fn can_handle(&self, config: &dyn CliConfig) -> bool {
        config.wants_help()
    }

    fn execute(&self, config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
        if config.wants_long_help() {
            println!("{}", self.long_help);
        } else {
            // Default to short help for -h
            println!("{}", self.short_help);
        }
        Ok(())
    }

    fn priority(&self) -> u8 {
        1
    }
}
