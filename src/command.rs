use crate::config::CliConfig;
use std::error::Error;

pub trait Command {
    fn can_handle(&self, config: &dyn CliConfig) -> bool;
    /// Execute the command.
    ///
    /// # Errors
    /// Returns an error if command execution fails.
    fn execute(&self, config: &dyn CliConfig) -> Result<(), Box<dyn Error>>;
    fn priority(&self) -> u8 {
        100
    }
}
