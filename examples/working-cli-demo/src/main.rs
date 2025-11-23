//! # Working CLI Demo - Builder-Config-Dispatcher Pattern
//!
//! This is a WORKING example that demonstrates the Builder-Config-Dispatcher
//! pattern using current sw-cli capabilities (without the future cli_app! macro).
//!
//! It shows what the pattern looks like when implemented manually with clap,
//! and provides a reference for what the cli_app! macro will generate.
//!
//! ## What This Demonstrates
//!
//! 1. **BaseConfig**: Standard flags (-v, -n, -q, -h, -V, -i, -o)
//! 2. **Custom Config**: CLI-specific fields (pattern, count, reverse)
//! 3. **Builder Pattern**: Constructing clap Command with standard + custom args
//! 4. **Dispatcher**: Chain of responsibility for command routing
//! 5. **Commands**: VersionCommand, HelpCommand, and custom commands
//!
//! ## Usage Examples
//!
//! ```bash
//! # Show version
//! cargo run -p working-cli-demo -- --version
//!
//! # Show help
//! cargo run -p working-cli-demo -- --help
//!
//! # Count lines (create test file first)
//! echo -e "line1\nline2\nline3" > /tmp/test.txt
//! cargo run -p working-cli-demo -- --count -i /tmp/test.txt
//!
//! # Search for pattern
//! cargo run -p working-cli-demo -- -p "line2" -i /tmp/test.txt
//!
//! # Reverse lines
//! cargo run -p working-cli-demo -- --reverse -i /tmp/test.txt
//!
//! # Verbose mode
//! cargo run -p working-cli-demo -- -v --count -i /tmp/test.txt
//!
//! # Dry-run mode
//! cargo run -p working-cli-demo -- -n --count -i /tmp/test.txt
//! ```

use clap::{Arg, ArgAction, ArgMatches, Command};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

// ============================================================================
// 1. CONFIG LAYER - What was requested
// ============================================================================

/// Standard flags common to all CLIs
#[derive(Debug, Clone)]
pub struct BaseConfig {
    pub verbose: bool,
    pub dry_run: bool,
    pub quiet: bool,
    pub help: bool,
    pub version: bool,
    pub input: Option<Vec<PathBuf>>,
    pub output: Option<PathBuf>,
}

/// CLI-specific configuration
#[derive(Debug, Clone)]
pub struct CliConfig {
    pub base: BaseConfig,
    // Custom fields for this CLI
    pub pattern: Option<String>,
    pub count: bool,
    pub reverse: bool,
}

impl CliConfig {
    pub fn verbosity(&self) -> u8 {
        if self.base.quiet {
            0
        } else if self.base.verbose {
            1
        } else {
            0
        }
    }

    pub fn is_dry_run(&self) -> bool {
        self.base.dry_run
    }
}

// ============================================================================
// 2. BUILDER LAYER - Constructing the CLI
// ============================================================================

fn build_cli() -> Command {
    Command::new("working-cli-demo")
        .disable_version_flag(true) // We handle --version ourselves
        .disable_help_flag(true) // We handle --help ourselves
        .about("Demonstrates Builder-Config-Dispatcher pattern (working implementation)")
        .long_about(
            "This is a working example showing the architecture pattern that \
             the future cli_app! macro will generate automatically."
        )
        // Add custom version flag that we control
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .action(ArgAction::SetTrue)
                .help("Show version information"),
        )
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .action(ArgAction::SetTrue)
                .help("Show help information"),
        )
        // Standard flags
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Increase output verbosity"),
        )
        .arg(
            Arg::new("dry-run")
                .short('n')
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Show what would be done without doing it"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Suppress non-essential output"),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .action(ArgAction::Append)
                .help("Input file(s)"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file"),
        )
        // Custom args
        .arg(
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .value_name("PATTERN")
                .help("Pattern to search for"),
        )
        .arg(
            Arg::new("count")
                .long("count")
                .action(ArgAction::SetTrue)
                .help("Count lines in input"),
        )
        .arg(
            Arg::new("reverse")
                .long("reverse")
                .action(ArgAction::SetTrue)
                .help("Reverse line order"),
        )
}

fn parse_config(matches: ArgMatches) -> CliConfig {
    CliConfig {
        base: BaseConfig {
            verbose: matches.get_flag("verbose"),
            dry_run: matches.get_flag("dry-run"),
            quiet: matches.get_flag("quiet"),
            help: matches.get_flag("help"),
            version: matches.get_flag("version"),
            input: matches
                .get_many::<String>("input")
                .map(|vals| vals.map(PathBuf::from).collect()),
            output: matches.get_one::<String>("output").map(PathBuf::from),
        },
        pattern: matches.get_one::<String>("pattern").cloned(),
        count: matches.get_flag("count"),
        reverse: matches.get_flag("reverse"),
    }
}

// ============================================================================
// 3. COMMAND LAYER - What to do
// ============================================================================

trait CliCommand {
    fn can_handle(&self, config: &CliConfig) -> bool;
    fn execute(&self, config: &CliConfig) -> Result<(), Box<dyn Error>>;
    fn priority(&self) -> u8 {
        100
    }
}

// Built-in: Version Command (Priority 0)
struct VersionCommand;

impl CliCommand for VersionCommand {
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

// Built-in: Help Command (Priority 1)
struct HelpCommand {
    help_text: String,
}

impl HelpCommand {
    fn new() -> Self {
        let help_text = build_cli().render_help().to_string();
        Self { help_text }
    }
}

impl CliCommand for HelpCommand {
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

// Custom: Count Command
struct CountCommand;

impl CliCommand for CountCommand {
    fn can_handle(&self, config: &CliConfig) -> bool {
        config.count
    }

    fn execute(&self, config: &CliConfig) -> Result<(), Box<dyn Error>> {
        if let Some(inputs) = &config.base.input {
            for path in inputs {
                if config.is_dry_run() {
                    println!("Would count lines in: {}", path.display());
                    continue;
                }

                if config.verbosity() > 0 {
                    eprintln!("Processing: {}", path.display());
                }

                let file = File::open(path)?;
                let reader = BufReader::new(file);
                let count = reader.lines().count();

                if config.verbosity() > 0 {
                    println!("{}: {} lines", path.display(), count);
                } else {
                    println!("{}", count);
                }
            }
        } else {
            if config.verbosity() > 0 {
                eprintln!("Reading from stdin...");
            }

            let stdin = io::stdin();
            let reader = BufReader::new(stdin);
            let count = reader.lines().count();
            println!("{}", count);
        }

        Ok(())
    }
}

// Custom: Grep Command
struct GrepCommand;

impl CliCommand for GrepCommand {
    fn can_handle(&self, config: &CliConfig) -> bool {
        config.pattern.is_some()
    }

    fn execute(&self, config: &CliConfig) -> Result<(), Box<dyn Error>> {
        let pattern = config.pattern.as_ref().unwrap();

        if let Some(inputs) = &config.base.input {
            for path in inputs {
                if config.is_dry_run() {
                    println!("Would search for '{}' in: {}", pattern, path.display());
                    continue;
                }

                let file = File::open(path)?;
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    let line = line?;
                    if line.contains(pattern) {
                        if config.verbosity() > 0 {
                            println!("{}: {}", path.display(), line);
                        } else {
                            println!("{}", line);
                        }
                    }
                }
            }
        } else {
            let stdin = io::stdin();
            let reader = BufReader::new(stdin);

            for line in reader.lines() {
                let line = line?;
                if line.contains(pattern) {
                    println!("{}", line);
                }
            }
        }

        Ok(())
    }
}

// Custom: Reverse Command
struct ReverseCommand;

impl CliCommand for ReverseCommand {
    fn can_handle(&self, config: &CliConfig) -> bool {
        config.reverse
    }

    fn execute(&self, config: &CliConfig) -> Result<(), Box<dyn Error>> {
        if let Some(inputs) = &config.base.input {
            for path in inputs {
                if config.is_dry_run() {
                    println!("Would reverse lines in: {}", path.display());
                    continue;
                }

                let file = File::open(path)?;
                let reader = BufReader::new(file);
                let lines: Result<Vec<_>, _> = reader.lines().collect();
                let lines = lines?;

                for line in lines.iter().rev() {
                    println!("{}", line);
                }
            }
        } else {
            let stdin = io::stdin();
            let reader = BufReader::new(stdin);
            let lines: Result<Vec<_>, _> = reader.lines().collect();
            let lines = lines?;

            for line in lines.iter().rev() {
                println!("{}", line);
            }
        }

        Ok(())
    }
}

// Custom: Copy Command (default)
struct CopyCommand;

impl CliCommand for CopyCommand {
    fn can_handle(&self, _config: &CliConfig) -> bool {
        true // Default handler
    }

    fn execute(&self, config: &CliConfig) -> Result<(), Box<dyn Error>> {
        if let Some(inputs) = &config.base.input {
            for path in inputs {
                if config.is_dry_run() {
                    println!("Would copy: {}", path.display());
                    continue;
                }

                let file = File::open(path)?;
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    println!("{}", line?);
                }
            }
        } else {
            let stdin = io::stdin();
            let reader = BufReader::new(stdin);

            for line in reader.lines() {
                println!("{}", line?);
            }
        }

        Ok(())
    }
}

// ============================================================================
// 4. DISPATCHER - Chain of Responsibility
// ============================================================================

struct Dispatcher {
    commands: Vec<Box<dyn CliCommand>>,
}

impl Dispatcher {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    fn register<C: CliCommand + 'static>(mut self, command: C) -> Self {
        self.commands.push(Box::new(command));
        // Sort by priority
        self.commands.sort_by_key(|c| c.priority());
        self
    }

    fn dispatch(&self, config: &CliConfig) -> Result<(), Box<dyn Error>> {
        // Try each command in priority order
        for command in &self.commands {
            if command.can_handle(config) {
                return command.execute(config);
            }
        }

        Err("No command could handle this request".into())
    }
}

// ============================================================================
// 5. MAIN - Tie everything together
// ============================================================================

fn main() {
    // 1. Build CLI and parse arguments
    let matches = build_cli().get_matches();
    let config = parse_config(matches);

    // 2. Create dispatcher with all commands
    let dispatcher = Dispatcher::new()
        // Priority commands (checked first)
        .register(VersionCommand) // Priority 0
        .register(HelpCommand::new()) // Priority 1
        // Feature commands
        .register(CountCommand) // Priority 100
        .register(GrepCommand) // Priority 100
        .register(ReverseCommand) // Priority 100
        // Default command
        .register(CopyCommand); // Priority 100

    // 3. Execute appropriate command
    if let Err(e) = dispatcher.dispatch(&config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
