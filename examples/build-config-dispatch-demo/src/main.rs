//! # Builder-Config-Dispatch Demo
//!
//! This example demonstrates the complete CLI framework pattern using the
//! Builder, Config, and Dispatcher modules from sw-cli.
//!
//! ## Architecture Overview
//!
//! 1. **Builder**: Constructs the CLI using clap with standard flags
//! 2. **Config**: Manages CLI state (flags, I/O settings, custom options)
//! 3. **Dispatcher**: Routes to appropriate command based on configuration
//!
//! ## Example Commands
//!
//! ```bash
//! # Show version information
//! build-config-dispatch-demo --version
//!
//! # Show help
//! build-config-dispatch-demo --help
//!
//! # Process files (normal mode)
//! build-config-dispatch-demo -i input.txt -o output.txt
//!
//! # Process with dry-run (show what would be done)
//! build-config-dispatch-demo -n -i input.txt -o output.txt
//!
//! # Process with verbose output
//! build-config-dispatch-demo -v -i input.txt
//!
//! # Process in quiet mode
//! build-config-dispatch-demo -q -i input.txt
//!
//! # Count lines in file
//! build-config-dispatch-demo --count -i input.txt
//! ```
//!
//! ## NOTE: This is a Design Demonstration
//!
//! The Config, Builder, and Dispatcher modules shown here represent the FUTURE
//! state of sw-cli after the refactoring described in docs/plan.md is complete.
//!
//! Currently, this example won't compile because these modules don't exist yet.
//! This code serves as a specification and design document for how the final
//! implementation should work.

// NOTE: These imports will work after the refactoring in Phase 3 (v0.3.0)
// They are shown here as a design specification
/*
use sw_cli::version;
use cli_utilities::{
    builder::{CliBuilder, StandardCliBuilder},
    config::{CliConfig, CommonFlags, IoConfig},
    dispatcher::{Command, CommandResult, Dispatcher, HelpCommand, VersionCommand},
};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
*/

// TEMPORARY: For now, provide a minimal implementation to demonstrate structure
fn main() {
    println!("=== Build-Config-Dispatch Demo ===\n");
    println!("NOTE: This is a design demonstration showing how the Builder,");
    println!("Config, and Dispatcher modules will work together in sw-cli v0.3.0.\n");
    println!("See the source code and inline documentation for the complete");
    println!("architecture and usage patterns.\n");
    println!("Status: Awaiting implementation of modules defined in docs/plan.md");

    // Show what the final implementation will look like
    show_example_usage();
}

/// Demonstrates the complete integration pattern
fn show_example_usage() {
    println!("\n=== Future Implementation Pattern ===\n");

    println!("1. BUILDER: Define CLI structure");
    println!("   - Custom builder implements CliBuilder trait");
    println!("   - Adds standard flags (-n, -v, -h, -V, -q, -f)");
    println!("   - Adds I/O arguments (-i, -o, -a)");
    println!("   - Adds custom arguments (--count, --reverse)");

    println!("\n2. CONFIG: Parse arguments into state");
    println!("   - CommonFlags: dry_run, verbose, help, version, quiet, force");
    println!("   - IoConfig: input files, output file, stdin/stdout detection");
    println!("   - Custom: Application-specific settings");

    println!("\n3. DISPATCHER: Route to appropriate command");
    println!("   - VersionCommand: Handles --version flag");
    println!("   - HelpCommand: Handles --help flag");
    println!("   - ProcessCommand: Main file processing logic");
    println!("   - CountCommand: Custom command for counting lines");

    println!("\n4. EXECUTION: Command runs with config");
    println!("   - Respects dry-run mode");
    println!("   - Adapts output based on verbosity");
    println!("   - Handles I/O according to config");

    println!("\n=== Code Structure ===\n");
    println!("{}", CODE_EXAMPLE);
}

const CODE_EXAMPLE: &str = r#"
// ============================================================================
// CUSTOM BUILDER: Define CLI structure
// ============================================================================

struct DemoCliBuilder;

impl CliBuilder for DemoCliBuilder {
    fn build_cli() -> ClapCommand {
        let cmd = ClapCommand::new("build-config-dispatch-demo")
            .version("0.1.0")
            .about("Demonstrates Builder-Config-Dispatcher pattern")
            .long_about("Complete example showing how to build CLI applications \
                        using sw-cli's modular framework")
            // Add custom arguments
            .arg(Arg::new("count")
                .long("count")
                .help("Count lines in input file(s)")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("reverse")
                .long("reverse")
                .help("Reverse line order")
                .action(clap::ArgAction::SetTrue));

        // Add standard flags and I/O arguments
        Self::with_standard_flags(Self::with_io_args(cmd))
    }

    fn parse_args(matches: ArgMatches) -> CliConfig {
        let mut config = CliConfig {
            flags: Self::parse_standard_flags(&matches),
            io: Self::parse_io_config(&matches),
            custom: HashMap::new(),
        };

        // Parse custom flags
        if matches.get_flag("count") {
            config.custom.insert("count".to_string(), "true".to_string());
        }
        if matches.get_flag("reverse") {
            config.custom.insert("reverse".to_string(), "true".to_string());
        }

        config
    }
}

// ============================================================================
// CUSTOM COMMANDS: Implement Command trait
// ============================================================================

/// Main file processing command
struct ProcessCommand;

impl Command for ProcessCommand {
    fn execute(&self, config: &CliConfig) -> CommandResult {
        // Determine operation mode
        let is_count = config.custom.get("count")
            .map(|v| v == "true")
            .unwrap_or(false);

        let is_reverse = config.custom.get("reverse")
            .map(|v| v == "true")
            .unwrap_or(false);

        if config.verbosity() > 0 {
            eprintln!("Processing mode: {}",
                if is_count { "count" }
                else if is_reverse { "reverse" }
                else { "copy" });
        }

        // Handle input sources
        let input_files = config.io.input.as_ref()
            .cloned()
            .unwrap_or_default();

        if input_files.is_empty() {
            if config.verbosity() > 0 {
                eprintln!("Reading from stdin...");
            }
            self.process_input(io::stdin().lock(), config)?;
        } else {
            for input_path in &input_files {
                if config.verbosity() > 0 {
                    eprintln!("Processing: {}", input_path.display());
                }

                if config.is_dry_run() {
                    println!("Would process: {}", input_path.display());
                    continue;
                }

                let file = fs::File::open(input_path)?;
                let reader = BufReader::new(file);
                self.process_input(reader, config)?;
            }
        }

        Ok(())
    }

    fn can_handle(&self, config: &CliConfig) -> bool {
        // Handle when not version/help
        !config.wants_version() && !config.wants_help()
    }

    fn name(&self) -> &str {
        "process"
    }
}

impl ProcessCommand {
    fn process_input<R: BufRead>(
        &self,
        reader: R,
        config: &CliConfig
    ) -> CommandResult {
        let is_count = config.custom.get("count")
            .map(|v| v == "true")
            .unwrap_or(false);

        let is_reverse = config.custom.get("reverse")
            .map(|v| v == "true")
            .unwrap_or(false);

        if is_count {
            self.count_lines(reader, config)
        } else if is_reverse {
            self.reverse_lines(reader, config)
        } else {
            self.copy_lines(reader, config)
        }
    }

    fn count_lines<R: BufRead>(
        &self,
        reader: R,
        config: &CliConfig
    ) -> CommandResult {
        let count = reader.lines().count();

        if config.verbosity() > 0 {
            println!("Total lines: {}", count);
        } else {
            println!("{}", count);
        }

        Ok(())
    }

    fn reverse_lines<R: BufRead>(
        &self,
        reader: R,
        config: &CliConfig
    ) -> CommandResult {
        let lines: Result<Vec<_>, _> = reader.lines().collect();
        let lines = lines?;

        let mut output: Box<dyn Write> = if let Some(output_path) = &config.io.output {
            if config.is_dry_run() {
                println!("Would write to: {}", output_path.display());
                return Ok(());
            }

            let file = if config.io.append {
                fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(output_path)?
            } else {
                fs::File::create(output_path)?
            };
            Box::new(file)
        } else {
            Box::new(io::stdout())
        };

        for line in lines.iter().rev() {
            writeln!(output, "{}", line)?;
        }

        Ok(())
    }

    fn copy_lines<R: BufRead>(
        &self,
        reader: R,
        config: &CliConfig
    ) -> CommandResult {
        let mut output: Box<dyn Write> = if let Some(output_path) = &config.io.output {
            if config.is_dry_run() {
                println!("Would write to: {}", output_path.display());
                return Ok(());
            }

            let file = if config.io.append {
                fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(output_path)?
            } else {
                fs::File::create(output_path)?
            };
            Box::new(file)
        } else {
            Box::new(io::stdout())
        };

        for line in reader.lines() {
            writeln!(output, "{}", line?)?;
        }

        Ok(())
    }
}

// ============================================================================
// MAIN: Tie everything together
// ============================================================================

fn main() {
    // 1. Build CLI and parse arguments
    let matches = DemoCliBuilder::build_cli().get_matches();
    let config = DemoCliBuilder::parse_args(matches);

    // 2. Create dispatcher with all commands
    let dispatcher = Dispatcher::new()
        // Priority commands (checked first)
        .register(VersionCommand::new(version!()))
        .register(HelpCommand::new(
            DemoCliBuilder::build_cli()
                .render_help()
                .to_string()
        ))
        // Default command (runs if no other command matches)
        .with_default(ProcessCommand);

    // 3. Execute appropriate command
    if let Err(e) = dispatcher.dispatch(&config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

// ============================================================================
// EXAMPLE USAGE
// ============================================================================

/*
# Show version (using sw-cli version module)
$ build-config-dispatch-demo --version
Version: 0.1.0
Copyright (c) 2025 Software Wrighter
MIT License: https://github.com/softwarewrighter/sw-cli/blob/main/LICENSE
Build: abc123d @ hostname (2025-11-22T22:43:30+00:00)

# Show help (generated from clap)
$ build-config-dispatch-demo --help
Demonstrates Builder-Config-Dispatcher pattern
...

# Count lines in a file
$ build-config-dispatch-demo --count -i README.md
45

# Count lines with verbose output
$ build-config-dispatch-demo -v --count -i README.md
Processing: README.md
Total lines: 45

# Dry-run: show what would be done
$ build-config-dispatch-demo -n -i input.txt -o output.txt
Would process: input.txt
Would write to: output.txt

# Reverse file contents
$ build-config-dispatch-demo --reverse -i input.txt -o reversed.txt

# Process stdin to stdout (pipes)
$ cat input.txt | build-config-dispatch-demo > output.txt

# Multiple input files
$ build-config-dispatch-demo -i file1.txt -i file2.txt -o combined.txt

# Quiet mode (minimal output)
$ build-config-dispatch-demo -q --count -i large-file.txt
1000000
*/
"#;
