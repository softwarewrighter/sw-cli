use working_cli_demo::CliConfig;
use working_cli_demo::config::BaseConfig;

#[test]
fn test_verbosity_quiet() {
    let config = CliConfig {
        base: BaseConfig {
            verbose: false,
            dry_run: false,
            quiet: true,
            help: false,
            version: false,
            input: None,
            output: None,
        },
        pattern: None,
        count: false,
        reverse: false,
    };

    assert_eq!(config.verbosity(), 0);
}

#[test]
fn test_verbosity_verbose() {
    let config = CliConfig {
        base: BaseConfig {
            verbose: true,
            dry_run: false,
            quiet: false,
            help: false,
            version: false,
            input: None,
            output: None,
        },
        pattern: None,
        count: false,
        reverse: false,
    };

    assert_eq!(config.verbosity(), 1);
}

#[test]
fn test_dry_run() {
    let config = CliConfig {
        base: BaseConfig {
            verbose: false,
            dry_run: true,
            quiet: false,
            help: false,
            version: false,
            input: None,
            output: None,
        },
        pattern: None,
        count: false,
        reverse: false,
    };

    assert!(config.is_dry_run());
}
