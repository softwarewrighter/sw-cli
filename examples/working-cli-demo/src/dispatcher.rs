use crate::CliConfig;
use std::error::Error;

pub trait Command {
    fn can_handle(&self, config: &CliConfig) -> bool;
    fn execute(&self, config: &CliConfig) -> Result<(), Box<dyn Error>>;
    fn priority(&self) -> u8 { 100 }
}

pub struct Dispatcher {
    commands: Vec<Box<dyn Command>>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn register<C: Command + 'static>(mut self, command: C) -> Self {
        self.commands.push(Box::new(command));
        self.commands.sort_by_key(|c| c.priority());
        self
    }

    pub fn dispatch(&self, config: &CliConfig) -> Result<(), Box<dyn Error>> {
        for command in &self.commands {
            if command.can_handle(config) {
                return command.execute(config);
            }
        }
        Err("No command could handle this request".into())
    }
}
