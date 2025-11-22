# Clap Integration and Custom Help Text Design

## How Clap is Used

The `cli_app!` macro generates **pure clap code**. Users can mix macro-generated and hand-written clap as needed.

### Three Levels of Clap Usage

#### Level 1: Pure Macro (Simplest)
```rust
cli_app! {
    name: "my-tool",
    about: "Does things",
    args: {
        pattern: String => (short = 'p', long = "pattern"),
    },
    commands: [...]
}
```

**Expands to pure clap:**
```rust
fn build_cli() -> clap::Command {
    use clap::{Command, Arg, ArgAction};

    Command::new("my-tool")
        .about("Does things")
        // Standard flags added by macro
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .action(ArgAction::SetTrue))
        // ... more standard flags
        // Custom args from macro
        .arg(Arg::new("pattern")
            .short('p')
            .long("pattern")
            .value_name("PATTERN"))
}
```

#### Level 2: Macro + Manual Clap (Flexible)
```rust
cli_app! {
    name: "my-tool",

    // Use builder_fn to customize clap command
    builder: |cmd| {
        cmd.about("Custom about text")
           .long_about(include_str!("long-help.txt"))
           .after_help("See --help for more details")
           // Add complex args the macro doesn't support
           .arg(Arg::new("config")
                .long("config")
                .value_name("FILE")
                .value_parser(clap::value_parser!(PathBuf))
                .conflicts_with("pattern"))
    },

    args: {
        pattern: String => (short = 'p'),
    },

    commands: [...]
}
```

#### Level 3: Full Clap Control (Maximum Power)
```rust
// Bypass macro entirely for the builder
fn build_cli() -> clap::Command {
    let base_cmd = Command::new("my-tool")
        .about("Custom text")
        .long_about(include_str!("long-help.txt"));

    // Add standard flags using helper
    let cmd = sw_cli::builder::with_standard_flags(base_cmd);

    // Add custom args
    cmd.arg(Arg::new("pattern").short('p'))
}

// Still use macro for config parsing and dispatch
fn main() {
    let matches = build_cli().get_matches();
    let config = parse_cli_config!(matches, MyToolConfig, {
        pattern: get_one<String>("pattern").cloned(),
    });

    // Use standard dispatcher
    sw_cli::dispatch(&config, vec![
        Box::new(CountCommand),
        Box::new(GrepCommand),
    ]);
}
```

## Custom Help Text Integration

### Design: Two-Level Help System

```
-h, --help          Show brief help (clap-generated + src/short-help.txt)
--help-long         Show detailed help (clap-generated + src/long-help.txt)
```

### File Structure
```
my-cli/
├── src/
│   ├── main.rs
│   ├── short-help.txt     # Brief additional help
│   └── long-help.txt      # Detailed documentation
├── build.rs
└── Cargo.toml
```

### Implementation in Macro

```rust
cli_app! {
    name: "my-tool",
    about: "Brief description for clap",

    // Macro looks for these files and includes them
    help: {
        short: "src/short-help.txt",    // Optional
        long: "src/long-help.txt",      // Optional
    },

    args: { ... },
    commands: [ ... ]
}
```

**Expands to:**

```rust
fn build_cli() -> clap::Command {
    let mut cmd = Command::new("my-tool")
        .about("Brief description for clap");

    // If short-help.txt exists, add it to after_help
    #[cfg(feature = "include-help")]
    {
        const SHORT_HELP: &str = include_str!("short-help.txt");
        cmd = cmd.after_help(SHORT_HELP);
    }

    // If long-help.txt exists, add it to long_about
    #[cfg(feature = "include-help")]
    {
        const LONG_HELP: &str = include_str!("long-help.txt");
        cmd = cmd.long_about(LONG_HELP);
    }

    // Add --help-long flag for extended help
    cmd = cmd.arg(
        Arg::new("help-long")
            .long("help-long")
            .action(ArgAction::SetTrue)
            .help("Show detailed help information")
    );

    // ... standard flags
    // ... custom args

    cmd
}
```

### Help Command Implementation

```rust
pub struct HelpCommand {
    short_help: String,
    long_help: Option<String>,
}

impl HelpCommand {
    pub fn new(cli_builder: fn() -> clap::Command) -> Self {
        let cmd = cli_builder();

        // Get clap-generated help
        let short_help = cmd.render_help().to_string();

        // Get long help if --help-long would be used
        let long_help = cmd.render_long_help().to_string();

        Self {
            short_help,
            long_help: Some(long_help),
        }
    }
}

impl Command for HelpCommand {
    fn can_handle(&self, config: &dyn CliConfig) -> bool {
        config.wants_help() || config.wants_help_long()
    }

    fn execute(&self, config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
        if config.wants_help_long() {
            if let Some(long) = &self.long_help {
                println!("{}", long);
            } else {
                println!("{}", self.short_help);
            }
        } else {
            println!("{}", self.short_help);
        }
        Ok(())
    }

    fn priority(&self) -> u8 { 1 }
}
```

### Example Help Files

**src/short-help.txt:**
```
EXAMPLES:
    # Count lines in a file
    my-tool --count -i file.txt

    # Search for pattern
    my-tool -p "error" -i logs.txt

    # Dry run mode
    my-tool -n -i input.txt

For more examples, use --help-long
```

**src/long-help.txt:**
```
DETAILED DESCRIPTION:
    my-tool is a text processing utility that supports multiple operations
    on text files including counting, searching, reversing, and more.

COMMON WORKFLOWS:

    1. Log Analysis
       Search logs for errors and count occurrences:
       $ my-tool -p "ERROR" -i server.log | my-tool --count

    2. File Processing Pipeline
       Reverse and uppercase text:
       $ my-tool --reverse -i input.txt | my-tool --upper

    3. Batch Processing
       Process multiple files:
       $ my-tool -i *.txt -o combined.txt

STANDARD FLAGS:
    -v, --verbose    Increase output verbosity
    -n, --dry-run    Show what would be done without doing it
    -q, --quiet      Suppress non-essential output
    -h, --help       Show brief help
    --help-long      Show this detailed help
    -V, --version    Show version information

INPUT/OUTPUT:
    -i, --input FILE     Input file(s), can be specified multiple times
    -o, --output FILE    Output file (default: stdout)
    -a, --append         Append to output file instead of overwriting

ADVANCED USAGE:
    # Combine flags for complex operations
    my-tool -v -n -p "pattern" -i file1.txt -i file2.txt -o output.txt

    # Use stdin/stdout for pipes
    cat input.txt | my-tool --reverse | grep "pattern"

    # Force overwrite with confirmation
    my-tool -f -i data.txt -o existing.txt

CONFIGURATION:
    my-tool reads configuration from the following locations (in order):
    1. /etc/my-tool/config.toml
    2. ~/.config/my-tool/config.toml
    3. ./my-tool.toml

SEE ALSO:
    Project homepage: https://github.com/user/my-tool
    Issue tracker: https://github.com/user/my-tool/issues
```

## Complete Example with Help Integration

```rust
// main.rs
cli_app! {
    name: "text-processor",
    version: "0.1.0",
    about: "A powerful text processing tool",

    // Help files augment clap-generated help
    help: {
        short: "src/short-help.txt",
        long: "src/long-help.txt",
    },

    args: {
        pattern: String => (
            short = 'p',
            long = "pattern",
            help = "Pattern to search for",
            value_name = "PATTERN"
        ),
        count: bool => (
            long = "count",
            help = "Count matching lines"
        ),
    },

    commands: [GrepCommand, CountCommand, CopyCommand]
}

// The macro generates:
// 1. Clap command with all flags
// 2. Help text from files included at compile time
// 3. HelpCommand that shows appropriate help level
// 4. Config struct with help/help_long fields
```

## Usage Examples

```bash
# Brief help (clap defaults + short-help.txt)
$ text-processor -h
A powerful text processing tool

Usage: text-processor [OPTIONS]

Options:
  -v, --verbose         Increase output verbosity
  -n, --dry-run        Show what would be done
  -h, --help           Show this help
  --help-long          Show detailed help
  -V, --version        Show version information
  -i, --input <FILE>   Input file(s)
  -o, --output <FILE>  Output file
  -p, --pattern <PAT>  Pattern to search for
  --count              Count matching lines

EXAMPLES:
    # Count lines in a file
    text-processor --count -i file.txt

    # Search for pattern
    text-processor -p "error" -i logs.txt

For more examples, use --help-long

# Detailed help (clap defaults + long-help.txt)
$ text-processor --help-long
A powerful text processing tool

Usage: text-processor [OPTIONS]

[Full clap-generated help with all options]

DETAILED DESCRIPTION:
    text-processor is a text processing utility...

[Full content of long-help.txt]

# Version (from sw-cli)
$ text-processor -V
Version: 0.1.0
Copyright (c) 2025 Software Wrighter
MIT License: https://github.com/...
Build: abc123d @ hostname (2025-11-22T...)
```

## BaseConfig Extensions for Help

```rust
pub struct BaseConfig {
    // Existing fields
    pub verbose: bool,
    pub dry_run: bool,
    pub quiet: bool,
    pub help: bool,
    pub version: bool,
    pub input: Option<Vec<PathBuf>>,
    pub output: Option<PathBuf>,
    pub append: bool,

    // New help field
    pub help_long: bool,   // --help-long flag
}

pub trait CliConfig {
    fn base(&self) -> &BaseConfig;
    fn wants_help(&self) -> bool { self.base().help }
    fn wants_help_long(&self) -> bool { self.base().help_long }
    fn wants_version(&self) -> bool { self.base().version }
    // ... other methods
}
```

## Advanced: Section-Based Help

For very complex CLIs, support multiple help sections:

```rust
cli_app! {
    name: "complex-tool",

    help: {
        // Multiple sections that can be shown individually
        short: "src/help/short.txt",
        long: "src/help/long.txt",
        examples: "src/help/examples.txt",
        config: "src/help/configuration.txt",
        api: "src/help/api-reference.txt",
    },

    // Additional help flags
    help_flags: {
        examples: "help-examples" => "Show usage examples",
        config: "help-config" => "Show configuration help",
        api: "help-api" => "Show API reference",
    },

    args: { ... },
    commands: [ ... ]
}
```

```bash
$ complex-tool --help-examples
# Shows just examples section

$ complex-tool --help-config
# Shows just configuration section

$ complex-tool --help-long
# Shows everything
```

## Benefits of This Design

1. **Clap Native**: All the power of clap is available
2. **DRY**: Standard flags defined once in sw-cli
3. **Customizable**: Can augment or override anything
4. **Compile-time**: Help text included at compile time (no runtime file reads)
5. **Flexible**: Support simple (macro-only) to complex (manual clap) use cases
6. **Professional**: Two-level help (brief/detailed) is standard practice
7. **Maintainable**: Help text in separate files, easier to edit
