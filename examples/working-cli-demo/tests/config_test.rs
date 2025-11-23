use sw_cli::{BaseConfig, CliConfig, HelpType};
use working_cli_demo::DemoConfig;

#[test]
fn test_verbosity_normal() {
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

    assert_eq!(config.verbosity(), 0);
}

#[test]
fn test_verbosity_verbose() {
    let config = DemoConfig {
        base: BaseConfig {
            verbose: true,
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

    assert_eq!(config.verbosity(), 1);
}

#[test]
fn test_dry_run() {
    let config = DemoConfig {
        base: BaseConfig {
            verbose: false,
            dry_run: true,
            help: HelpType::None,
            version: false,
        },
        input: None,
        output: None,
        pattern: None,
        count: false,
        reverse: false,
    };

    assert!(config.is_dry_run());
}
