use crate::{CliConfig, Command};
use std::error::Error;

pub struct VersionCommand;

impl Command for VersionCommand {
    fn can_handle(&self, config: &CliConfig) -> bool {
        config.base.version
    }

    fn execute(&self, _config: &CliConfig) -> Result<(), Box<dyn Error>> {
        println!("{}", sw_cli::version!());
        Ok(())
    }

    fn priority(&self) -> u8 {
        0
    }
}
