# SW-CLI Modular Refactoring Plan

## Executive Summary

This document outlines a refactoring plan to transform `sw-cli` from a monolithic library into a modular, extensible workspace that can support multiple CLI utility functions beyond version information.

## Current Structure Analysis

### Workspace Layout
```
sw-cli/
├── src/
│   ├── lib.rs              # Re-exports macros and version module
│   └── version.rs          # BuildInfo and Version structs
├── sw-cli-macros/          # Procedural macro crate
│   └── src/lib.rs          # version!(), define_build_info!(), create_version!()
├── examples/
│   └── demo-cli/           # Example demonstrating version functionality
└── docs/
    └── version-usage.md    # User documentation
```

### Current Issues
1. **Tight Coupling**: Build and version functionality are mixed in a single `version.rs` module
2. **Limited Modularity**: Difficult to add new CLI utilities (argument parsing, config management, etc.)
3. **Naming Inconsistency**: `sw-cli-macros` vs workspace naming patterns
4. **Example Specificity**: `demo-cli` doesn't indicate it's version-specific
5. **Future Constraints**: No clear path to add non-version related subcrates

## Proposed Modular Architecture

### New Workspace Structure
```
sw-cli/
├── crates/
│   ├── cli-utilities/      # Core utilities subcrate
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── version/    # Version module
│   │   │   │   └── mod.rs  # Version struct and display
│   │   │   └── build/      # Build module
│   │   │       └── mod.rs  # BuildInfo struct and metadata
│   │   └── Cargo.toml
│   │
│   └── cli-macros/         # Procedural macro crate (renamed)
│       ├── src/
│       │   ├── lib.rs      # Macro implementations
│       │   ├── version.rs  # version!() macro
│       │   └── build.rs    # define_build_info!() macro
│       └── Cargo.toml
│
├── examples/
│   └── cli-version-demo/   # Renamed, version-specific example
│       ├── src/main.rs
│       ├── build.rs
│       ├── COPYRIGHT
│       └── Cargo.toml
│
├── src/
│   └── lib.rs              # Root crate re-exports from cli-utilities
├── docs/
│   ├── version-usage.md    # Existing version documentation
│   └── plan.md             # This document
├── Cargo.toml              # Workspace definition
└── build.rs                # Root build script
```

### Future Extensibility
This structure allows for easy addition of new functionality:
```
crates/
├── cli-utilities/          # Existing
├── cli-macros/             # Existing
├── cli-config/             # Future: Configuration management
├── cli-args/               # Future: Advanced argument parsing
├── cli-logging/            # Future: Structured logging utilities
└── cli-term/               # Future: Terminal UI helpers
```

## Detailed Refactoring Steps

### Phase 1: Create New Crate Structure

#### 1.1 Create `crates/cli-utilities` Subcrate
- Initialize new crate: `crates/cli-utilities/`
- Separate concerns:
  - **build module** (`crates/cli-utilities/src/build/mod.rs`)
    - `BuildInfo` struct
    - Build metadata collection
    - Display formatting for build info
    - Tests for build info formatting

  - **version module** (`crates/cli-utilities/src/version/mod.rs`)
    - `Version` struct
    - Version composition (combines build info)
    - Display formatting for full version output
    - Tests for version display

- Dependencies:
  - `chrono = "0.4"` (for timestamp handling)

#### 1.2 Rename and Restructure Macros
- Rename `sw-cli-macros/` to `crates/cli-macros/`
- Split macro implementations:
  - `crates/cli-macros/src/version.rs` - `version!()` macro
  - `crates/cli-macros/src/build.rs` - `define_build_info!()` macro
  - `crates/cli-macros/src/lib.rs` - Public re-exports

- Update macro paths to reference `cli_utilities` crate:
  ```rust
  // In version!() macro
  ::cli_utilities::version::Version::new(...)
  ::cli_utilities::build::BuildInfo::new(...)
  ```

#### 1.3 Update Root Crate
- Modify `sw-cli/src/lib.rs` to re-export from new crates:
  ```rust
  // Re-export utilities for convenience
  pub use cli_utilities::{build, version};

  // Re-export macros
  pub use cli_macros::{define_build_info, version};

  // Deprecated but maintained for compatibility
  #[deprecated(since = "0.2.0", note = "Use version!() instead")]
  pub use cli_macros::create_version;
  ```

### Phase 2: Rename Example

#### 2.1 Rename Example Crate
- Move `examples/demo-cli/` to `examples/cli-version-demo/`
- Update `Cargo.toml`:
  ```toml
  [package]
  name = "cli-version-demo"
  version = "0.1.0"
  ```

#### 2.2 Update Example Documentation
- Update comments in `examples/cli-version-demo/src/main.rs`
- Add clear documentation about what the example demonstrates
- Reference version-specific functionality

### Phase 3: Update Workspace Configuration

#### 3.1 Update Root `Cargo.toml`
```toml
[workspace]
members = [
    "crates/cli-utilities",
    "crates/cli-macros",
    "examples/cli-version-demo"
]
resolver = "2"

[package]
name = "sw-cli"
version = "0.2.0"  # Bump for breaking changes
edition = "2024"

[dependencies]
cli-utilities = { path = "crates/cli-utilities" }
cli-macros = { path = "crates/cli-macros" }
```

#### 3.2 Update `cli-utilities/Cargo.toml`
```toml
[package]
name = "cli-utilities"
version = "0.2.0"
edition = "2024"

[dependencies]
chrono = "0.4"
```

#### 3.3 Update `cli-macros/Cargo.toml`
```toml
[package]
name = "cli-macros"
version = "0.2.0"
edition = "2024"

[lib]
proc-macro = true

[dependencies]
quote = "1.0"
syn = { version = "2.0", features = ["full"] }
proc-macro2 = "1.0"
```

### Phase 4: Update Build Scripts and Documentation

#### 4.1 Update Build Scripts
- Modify `scripts/build.sh` to handle new crate names
- Update `scripts/test.sh` to test all new subcrates
- Update `scripts/demo.sh` to reference `cli-version-demo`

#### 4.2 Update Documentation
- Update `docs/version-usage.md` with new import paths:
  ```rust
  // Old (still works via re-export)
  use sw_cli::version::Version;

  // New (direct access)
  use cli_utilities::version::Version;
  use cli_utilities::build::BuildInfo;
  ```

- Update `CLAUDE.md` to reflect new architecture:
  - Document the modular structure
  - Explain the separation of concerns
  - Update workspace member list
  - Document future extensibility plans

### Phase 5: Migration and Testing

#### 5.1 Code Migration Checklist
- [ ] Create `crates/cli-utilities/src/build/mod.rs` with `BuildInfo`
- [ ] Create `crates/cli-utilities/src/version/mod.rs` with `Version`
- [ ] Move tests from `src/version.rs` to appropriate modules
- [ ] Create `crates/cli-macros/` from `sw-cli-macros/`
- [ ] Update macro implementations to use new paths
- [ ] Update root `lib.rs` with re-exports
- [ ] Rename example directory and update references
- [ ] Update all `Cargo.toml` files

#### 5.2 Testing Strategy
1. **Unit Tests**: Ensure all existing tests pass in new locations
2. **Integration Tests**: Verify `version!()` macro works with new structure
3. **Example Test**: Run `cli-version-demo` and verify output format
4. **Build Test**: Ensure `scripts/build.sh` builds all crates
5. **Compatibility Test**: Verify old import paths still work via re-exports

## Breaking Changes and Migration Guide

### Breaking Changes
1. Internal module structure reorganized (users relying on direct paths affected)
2. Crate names changed (`sw-cli-macros` → `cli-macros`)
3. Version bumped to 0.2.0

### Migration for Users
Most users won't need to change anything due to re-exports in root crate:

```rust
// This still works (recommended for users)
use sw_cli::version!;

// This is new (for advanced users)
use cli_utilities::version::Version;
use cli_utilities::build::BuildInfo;
```

### Deprecation Timeline
- `create_version!()` macro: Deprecated in 0.2.0, remove in 0.3.0
- Old module paths: Supported via re-exports indefinitely

## Benefits of Refactoring

### Immediate Benefits
1. **Separation of Concerns**: Build and version logic are properly separated
2. **Better Organization**: Clear module hierarchy matches domain concepts
3. **Naming Consistency**: All crates follow `cli-*` naming pattern
4. **Example Clarity**: `cli-version-demo` clearly indicates purpose

### Long-term Benefits
1. **Extensibility**: Easy to add new CLI utility crates
2. **Maintainability**: Each module has clear, single responsibility
3. **Testability**: Isolated modules are easier to test
4. **Reusability**: Individual utilities can be used independently
5. **Documentation**: Clearer structure improves documentation

## Future Features: Builder, Config, and Dispatcher

### Overview
Beyond version information, the modular architecture enables a complete CLI framework with standardized patterns for argument parsing, configuration management, and command dispatch. This section details the design for three core components that will be added to `crates/cli-utilities/`:

1. **Config**: State management for common CLI flags and options
2. **Builder**: Clap-based CLI construction with trait-based extensibility
3. **Dispatcher**: Command execution based on configuration state

### Config Module Design

#### Purpose
Provide a standardized way to manage common CLI flag states across all CLI applications.

#### Location
`crates/cli-utilities/src/config/mod.rs`

#### Core Structure
```rust
/// Configuration state for common CLI flags
#[derive(Debug, Clone, Default)]
pub struct CliConfig {
    /// Common flags
    pub flags: CommonFlags,

    /// Input/output configuration
    pub io: IoConfig,

    /// Extension point for custom configuration
    pub custom: HashMap<String, String>,
}

/// Standard flags present in most CLI applications
#[derive(Debug, Clone, Default)]
pub struct CommonFlags {
    /// Dry-run mode: show what would be done without doing it
    pub dry_run: bool,

    /// Verbose output: provide detailed logging/output
    pub verbose: bool,

    /// Help requested: show usage information
    pub help: bool,

    /// Version requested: show version information
    pub version: bool,

    /// Quiet mode: suppress non-essential output
    pub quiet: bool,

    /// Force mode: bypass confirmations/warnings
    pub force: bool,
}

/// Input/output file configuration
#[derive(Debug, Clone, Default)]
pub struct IoConfig {
    /// Input file path(s)
    pub input: Option<Vec<PathBuf>>,

    /// Output file path
    pub output: Option<PathBuf>,

    /// Use stdin for input
    pub use_stdin: bool,

    /// Use stdout for output
    pub use_stdout: bool,

    /// Append to output file instead of overwriting
    pub append: bool,
}

impl CliConfig {
    /// Create a new empty configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if help was requested
    pub fn wants_help(&self) -> bool {
        self.flags.help
    }

    /// Check if version was requested
    pub fn wants_version(&self) -> bool {
        self.flags.version
    }

    /// Check if in dry-run mode
    pub fn is_dry_run(&self) -> bool {
        self.flags.dry_run
    }

    /// Get verbosity level (0 = normal, 1 = verbose, -1 = quiet)
    pub fn verbosity(&self) -> i8 {
        if self.flags.quiet {
            -1
        } else if self.flags.verbose {
            1
        } else {
            0
        }
    }
}
```

#### Usage Example
```rust
use cli_utilities::config::CliConfig;

fn main() {
    let config = CliConfig::new();

    if config.wants_help() {
        print_help();
        return;
    }

    if config.is_dry_run() {
        println!("DRY RUN MODE");
    }

    match config.verbosity() {
        1 => println!("Verbose mode enabled"),
        -1 => { /* quiet mode */ },
        _ => { /* normal mode */ }
    }
}
```

### Builder Module Design

#### Purpose
Provide a trait-based system for building CLI applications with clap, allowing each CLI to define its own argument structure while maintaining consistency.

#### Location
`crates/cli-utilities/src/builder/mod.rs`

#### Core Structure
```rust
use clap::{Command, Arg, ArgMatches};
use crate::config::CliConfig;

/// Trait for CLI applications to implement their specific argument structure
pub trait CliBuilder {
    /// Define the CLI structure using clap
    fn build_cli() -> Command;

    /// Parse arguments into configuration
    fn parse_args(matches: ArgMatches) -> CliConfig;

    /// Optional: customize standard flags
    fn with_standard_flags(cmd: Command) -> Command {
        cmd
            .arg(Arg::new("dry-run")
                .short('n')
                .long("dry-run")
                .help("Show what would be done without doing it")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("help")
                .short('h')
                .long("help")
                .help("Print help information")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("version")
                .short('V')
                .long("version")
                .help("Print version information")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress non-essential output")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("force")
                .short('f')
                .long("force")
                .help("Force operation, bypass confirmations")
                .action(clap::ArgAction::SetTrue))
    }

    /// Optional: add standard I/O arguments
    fn with_io_args(cmd: Command) -> Command {
        cmd
            .arg(Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input file(s)")
                .action(clap::ArgAction::Append))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file"))
            .arg(Arg::new("append")
                .short('a')
                .long("append")
                .help("Append to output file instead of overwriting")
                .action(clap::ArgAction::SetTrue))
    }

    /// Parse standard flags from ArgMatches
    fn parse_standard_flags(matches: &ArgMatches) -> CommonFlags {
        CommonFlags {
            dry_run: matches.get_flag("dry-run"),
            verbose: matches.get_flag("verbose"),
            help: matches.get_flag("help"),
            version: matches.get_flag("version"),
            quiet: matches.get_flag("quiet"),
            force: matches.get_flag("force"),
        }
    }

    /// Parse I/O configuration from ArgMatches
    fn parse_io_config(matches: &ArgMatches) -> IoConfig {
        let input = matches.get_many::<String>("input")
            .map(|vals| vals.map(PathBuf::from).collect());

        let output = matches.get_one::<String>("output")
            .map(PathBuf::from);

        IoConfig {
            input,
            output,
            use_stdin: input.is_none(),
            use_stdout: output.is_none(),
            append: matches.get_flag("append"),
        }
    }
}

/// Standard builder for simple CLIs
pub struct StandardCliBuilder;

impl CliBuilder for StandardCliBuilder {
    fn build_cli() -> Command {
        let cmd = Command::new("cli-app")
            .version("1.0.0")
            .about("A CLI application");

        Self::with_standard_flags(
            Self::with_io_args(cmd)
        )
    }

    fn parse_args(matches: ArgMatches) -> CliConfig {
        CliConfig {
            flags: Self::parse_standard_flags(&matches),
            io: Self::parse_io_config(&matches),
            custom: HashMap::new(),
        }
    }
}
```

#### Usage Example
```rust
use cli_utilities::builder::{CliBuilder, StandardCliBuilder};
use clap::Command;

// Simple usage with standard builder
fn main() {
    let matches = StandardCliBuilder::build_cli().get_matches();
    let config = StandardCliBuilder::parse_args(matches);

    // Use config...
}

// Custom CLI with additional arguments
struct MyCliBuilder;

impl CliBuilder for MyCliBuilder {
    fn build_cli() -> Command {
        let cmd = Command::new("my-cli")
            .version("1.0.0")
            .about("My custom CLI")
            .arg(Arg::new("custom-flag")
                .long("custom")
                .help("Custom functionality"));

        // Add standard flags
        Self::with_standard_flags(
            Self::with_io_args(cmd)
        )
    }

    fn parse_args(matches: ArgMatches) -> CliConfig {
        let mut config = CliConfig {
            flags: Self::parse_standard_flags(&matches),
            io: Self::parse_io_config(&matches),
            custom: HashMap::new(),
        };

        // Handle custom flags
        if matches.get_flag("custom-flag") {
            config.custom.insert("custom".to_string(), "true".to_string());
        }

        config
    }
}
```

### Dispatcher Module Design

#### Purpose
Execute different commands based on CLI configuration, providing a standardized pattern for command dispatch.

#### Location
`crates/cli-utilities/src/dispatcher/mod.rs`

#### Core Structure
```rust
use crate::config::CliConfig;
use std::error::Error;

/// Result type for command execution
pub type CommandResult = Result<(), Box<dyn Error>>;

/// Trait for commands that can be executed
pub trait Command {
    /// Execute the command with the given configuration
    fn execute(&self, config: &CliConfig) -> CommandResult;

    /// Check if this command should handle the given configuration
    fn can_handle(&self, config: &CliConfig) -> bool;

    /// Get command name for logging/debugging
    fn name(&self) -> &str;
}

/// Dispatcher that routes to appropriate command based on configuration
pub struct Dispatcher {
    commands: Vec<Box<dyn Command>>,
    default_command: Option<Box<dyn Command>>,
}

impl Dispatcher {
    /// Create a new dispatcher
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            default_command: None,
        }
    }

    /// Register a command
    pub fn register<C: Command + 'static>(mut self, command: C) -> Self {
        self.commands.push(Box::new(command));
        self
    }

    /// Set default command (runs if no other command matches)
    pub fn with_default<C: Command + 'static>(mut self, command: C) -> Self {
        self.default_command = Some(Box::new(command));
        self
    }

    /// Dispatch to appropriate command based on configuration
    pub fn dispatch(&self, config: &CliConfig) -> CommandResult {
        // Check for command that can handle this configuration
        for command in &self.commands {
            if command.can_handle(config) {
                if config.verbosity() > 0 {
                    eprintln!("Executing command: {}", command.name());
                }
                return command.execute(config);
            }
        }

        // Fall back to default command if set
        if let Some(default) = &self.default_command {
            if config.verbosity() > 0 {
                eprintln!("Executing default command: {}", default.name());
            }
            return default.execute(config);
        }

        Err("No command found to handle configuration".into())
    }
}

/// Built-in command for handling --version flag
pub struct VersionCommand {
    version_string: String,
}

impl VersionCommand {
    pub fn new(version_string: String) -> Self {
        Self { version_string }
    }
}

impl Command for VersionCommand {
    fn execute(&self, _config: &CliConfig) -> CommandResult {
        println!("{}", self.version_string);
        Ok(())
    }

    fn can_handle(&self, config: &CliConfig) -> bool {
        config.wants_version()
    }

    fn name(&self) -> &str {
        "version"
    }
}

/// Built-in command for handling --help flag
pub struct HelpCommand {
    help_text: String,
}

impl HelpCommand {
    pub fn new(help_text: String) -> Self {
        Self { help_text }
    }
}

impl Command for HelpCommand {
    fn execute(&self, _config: &CliConfig) -> CommandResult {
        println!("{}", self.help_text);
        Ok(())
    }

    fn can_handle(&self, config: &CliConfig) -> bool {
        config.wants_help()
    }

    fn name(&self) -> &str {
        "help"
    }
}
```

#### Usage Example
```rust
use cli_utilities::dispatcher::{Dispatcher, Command, CommandResult, VersionCommand, HelpCommand};
use cli_utilities::config::CliConfig;

// Define a custom command
struct ProcessCommand;

impl Command for ProcessCommand {
    fn execute(&self, config: &CliConfig) -> CommandResult {
        if config.is_dry_run() {
            println!("Would process files...");
        } else {
            println!("Processing files...");
            // Actual processing logic
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

fn main() {
    // Build configuration (from previous examples)
    let config = get_config();

    // Create dispatcher with commands
    let dispatcher = Dispatcher::new()
        .register(VersionCommand::new(sw_cli::version!()))
        .register(HelpCommand::new(get_help_text()))
        .with_default(ProcessCommand);

    // Dispatch to appropriate command
    if let Err(e) = dispatcher.dispatch(&config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

### Integration Pattern

#### Complete CLI Application Structure
```rust
use sw_cli::version;
use cli_utilities::{
    builder::{CliBuilder, StandardCliBuilder},
    config::CliConfig,
    dispatcher::{Dispatcher, Command, CommandResult, VersionCommand, HelpCommand},
};

// 1. Define your custom commands
struct MyCommand;

impl Command for MyCommand {
    fn execute(&self, config: &CliConfig) -> CommandResult {
        // Your logic here
        Ok(())
    }

    fn can_handle(&self, config: &CliConfig) -> bool {
        !config.wants_version() && !config.wants_help()
    }

    fn name(&self) -> &str {
        "my-command"
    }
}

// 2. Main function ties it all together
fn main() {
    // Build CLI and parse arguments
    let matches = StandardCliBuilder::build_cli().get_matches();
    let config = StandardCliBuilder::parse_args(matches);

    // Create dispatcher
    let dispatcher = Dispatcher::new()
        .register(VersionCommand::new(version!()))
        .register(HelpCommand::new(StandardCliBuilder::build_cli().render_help().to_string()))
        .with_default(MyCommand);

    // Execute
    if let Err(e) = dispatcher.dispatch(&config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

### Dependencies

Add to `crates/cli-utilities/Cargo.toml`:
```toml
[dependencies]
chrono = "0.4"
clap = { version = "4.5", features = ["derive"] }
```

### Testing Strategy

#### Config Module Tests
- Test flag state management
- Test verbosity level calculation
- Test I/O configuration
- Test custom configuration storage

#### Builder Module Tests
- Test standard flag creation
- Test I/O argument creation
- Test trait implementation
- Test custom builder extensions

#### Dispatcher Module Tests
- Test command registration
- Test command selection logic
- Test default command fallback
- Test error handling
- Test dry-run mode behavior

### Example Crate

The `examples/build-config-dispatch-demo/` example demonstrates the complete integration pattern:

#### Features Demonstrated
- **Custom CliBuilder**: Extends `StandardCliBuilder` with custom arguments (`--count`, `--reverse`)
- **Command Implementation**: Three distinct commands (Version, Help, Process)
- **Standard Flag Usage**: All common flags (-n, -v, -h, -V, -q, -f)
- **I/O Handling**: Multi-file input (-i), single output (-o), stdin/stdout, append mode (-a)
- **Dispatcher Routing**: Priority-based command selection
- **Dry-Run Mode**: Shows what would be done without executing
- **Verbosity Levels**: Adapts output based on -v (verbose) or -q (quiet)
- **Custom Commands**: Line counting and reversing functionality

#### Example Commands
```bash
# Show version information
build-config-dispatch-demo --version

# Count lines in files
build-config-dispatch-demo --count -i README.md

# Process with dry-run
build-config-dispatch-demo -n -i input.txt -o output.txt

# Reverse file contents with verbose output
build-config-dispatch-demo -v --reverse -i input.txt -o reversed.txt

# Process multiple inputs
build-config-dispatch-demo -i file1.txt -i file2.txt -o combined.txt
```

#### Implementation Status
**NOTE**: This example represents the FUTURE state after v0.3.0 refactoring.
Currently, the Config, Builder, and Dispatcher modules don't exist yet.
The example serves as a specification and design document for the final implementation.

See `examples/build-config-dispatch-demo/src/main.rs` for the complete code
and detailed inline documentation.

## Future Roadmap

### Version 0.2.0 (Initial Refactoring)
- Modular crate structure
- Separated build and version modules
- Renamed example to cli-version-demo

### Version 0.3.0 (Core CLI Framework)
- Add **config** module to `cli-utilities`
  - `CliConfig` with standard flags
  - `CommonFlags` struct
  - `IoConfig` for input/output handling

- Add **builder** module to `cli-utilities`
  - `CliBuilder` trait
  - Integration with clap 4.x
  - Standard flag templates
  - I/O argument templates

- Add **dispatcher** module to `cli-utilities`
  - `Command` trait
  - `Dispatcher` routing logic
  - Built-in `VersionCommand` and `HelpCommand`
  - Default command support

- Implement `examples/build-config-dispatch-demo` (specification already exists)
- Add clap dependency to cli-utilities
- Remove deprecated `create_version!()` macro

### Version 0.4.0 (Advanced Features)
- Add `cli-logging` crate for structured logging
  - Integration with config verbosity levels
  - Structured output formats (JSON, plain text)
  - Log level filtering

- Add `cli-term` crate for terminal UI helpers
  - Progress bars
  - Colored output
  - Interactive prompts
  - Table formatting

- Enhanced I/O handling
  - Stream processing support
  - Compression/decompression
  - Format detection

### Version 0.5.0 (Polish and Extensions)
- Add `cli-testing` utilities
  - Test fixtures for CLI testing
  - Mock I/O handling
  - Assertion helpers

- Performance optimizations
- Comprehensive documentation
- Best practices guide
- Migration guide from other CLI frameworks

## Implementation Timeline

### Recommended Approach
Execute phases sequentially to minimize risk:

1. **Phase 1**: Create new crate structure (no deletions yet)
2. **Phase 2**: Rename example
3. **Phase 3**: Update workspace configuration
4. **Phase 4**: Update documentation
5. **Phase 5**: Test thoroughly, then remove old code

### Rollback Plan
- Keep git tags at each phase
- Maintain feature branch until all tests pass
- Document any issues discovered during migration

## Conclusion

This refactoring transforms `sw-cli` from a single-purpose version library into a modular workspace ready for expansion. The new structure maintains backward compatibility while providing a clear path for adding new CLI utilities. The separation of build and version concerns improves code organization, and the consistent naming scheme (`cli-*`) establishes a pattern for future additions.
