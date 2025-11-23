pub mod builder;
pub mod command;
pub mod commands;
pub mod config;
pub mod dispatcher;
pub mod version;

// Re-export macros from sw-cli-macros for convenient usage
pub use sw_cli_macros::{
    create_version, define_build_info, define_help_info, dispatch, long_help, short_help, version,
};

// Re-export commonly used types
pub use command::Command;
pub use commands::{HelpCommand, VersionCommand};
pub use config::{BaseConfig, CliConfig, HelpType};
pub use dispatcher::Dispatcher;
