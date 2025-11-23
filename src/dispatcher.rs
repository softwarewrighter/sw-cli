use crate::command::Command;
use crate::commands::{HelpCommand, VersionCommand};
use crate::config::CliConfig;
use std::error::Error;

pub struct Dispatcher {
    commands: Vec<Box<dyn Command>>,
}

impl Dispatcher {
    /// Create a new Dispatcher with `VersionCommand` and `HelpCommand` automatically registered.
    ///
    /// # Arguments
    /// * `short_help` - The short help text to display when -h is used
    /// * `long_help` - The long help text to display when --help is used
    #[must_use]
    pub fn new(short_help: String, long_help: String) -> Self {
        let mut dispatcher = Self {
            commands: Vec::new(),
        };

        // Auto-register VersionCommand (priority 0) and HelpCommand (priority 1)
        dispatcher.commands.push(Box::new(VersionCommand));
        dispatcher
            .commands
            .push(Box::new(HelpCommand::new(short_help, long_help)));
        dispatcher.commands.sort_by_key(|c| c.priority());

        dispatcher
    }

    /// Register a command with the dispatcher.
    ///
    /// # Arguments
    /// * `command` - The command to register
    #[must_use]
    pub fn register<C: Command + 'static>(mut self, command: C) -> Self {
        self.commands.push(Box::new(command));
        self.commands.sort_by_key(|c| c.priority());
        self
    }

    /// Dispatch the request to the appropriate command.
    ///
    /// # Errors
    /// Returns an error if no command can handle the request or if command execution fails.
    pub fn dispatch(&self, config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
        for command in &self.commands {
            if command.can_handle(config) {
                return command.execute(config);
            }
        }
        Err("No command could handle this request".into())
    }
}
