use crate::{CliConfig, Command};
use std::error::Error;

pub struct HelpCommand {
    help_text: String,
}

impl HelpCommand {
    pub fn new(help_text: String) -> Self {
        Self { help_text }
    }
}

impl Command for HelpCommand {
    fn can_handle(&self, config: &CliConfig) -> bool {
        config.base.help
    }

    fn execute(&self, _config: &CliConfig) -> Result<(), Box<dyn Error>> {
        println!("{}", self.help_text);
        Ok(())
    }

    fn priority(&self) -> u8 {
        1
    }
}
