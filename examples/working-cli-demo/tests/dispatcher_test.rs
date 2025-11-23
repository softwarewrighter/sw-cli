use sw_cli::{BaseConfig, Command, HelpType, VersionCommand};
use working_cli_demo::DemoConfig;
use working_cli_demo::actions::*;

#[test]
fn test_version_command_priority() {
    let config = DemoConfig {
        base: BaseConfig {
            verbose: false,
            dry_run: false,
            help: HelpType::None,
            version: true,
        },
        input: None,
        output: None,
        pattern: None,
        count: false,
        reverse: false,
    };

    let cmd = VersionCommand;
    assert!(cmd.can_handle(&config));
    assert_eq!(cmd.priority(), 0);
}

#[test]
fn test_count_command_handles_count_flag() {
    let config = DemoConfig {
        base: BaseConfig {
            verbose: false,
            dry_run: false,
            help: HelpType::None,
            version: false,
        },
        input: None,
        output: None,
        pattern: None,
        count: true,
        reverse: false,
    };

    let cmd = CountCommand;
    assert!(cmd.can_handle(&config));
}

#[test]
fn test_copy_command_is_default() {
    let config = DemoConfig {
        base: BaseConfig {
            verbose: false,
            dry_run: false,
            help: HelpType::None,
            version: false,
        },
        input: None,
        output: None,
        pattern: None,
        count: false,
        reverse: false,
    };

    let cmd = CopyCommand;
    assert!(cmd.can_handle(&config));
}
