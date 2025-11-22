# Example: Complete CLI Using sw-cli Macros

## The Goal: Ultra-Concise CLI Code

This document shows what a real CLI looks like when using the sw-cli macro framework. The entire working CLI is about 50 lines of actual code.

## Complete Working Example

```rust
// main.rs - EVERYTHING for a working CLI with multiple commands
use sw_cli::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

// ============================================================================
// CLI DEFINITION - This is ALL you need to define your CLI
// ============================================================================

cli_app! {
    name: "text-tool",
    version: "0.1.0",
    about: "A text processing utility demonstrating sw-cli framework",

    // Define CLI-specific arguments (beyond standard -v, -n, -h, -V, -q, -i, -o)
    args: {
        pattern: String => (
            short = 'p',
            long = "pattern",
            help = "Pattern to search for"
        ),
        count: bool => (
            long = "count",
            help = "Count matching lines"
        ),
        reverse: bool => (
            long = "reverse",
            help = "Reverse line order"
        ),
        upper: bool => (
            long = "upper",
            help = "Convert to uppercase"
        ),
    },

    // Register command handlers (checked in order, first match wins)
    commands: [
        CountCommand,      // Handles --count
        ReverseCommand,    // Handles --reverse
        UpperCommand,      // Handles --upper
        GrepCommand,       // Handles --pattern
        CopyCommand,       // Default: just copy input to output
    ]
}

// ============================================================================
// COMMAND IMPLEMENTATIONS - One struct per command
// ============================================================================

cli_command! {
    name: CountCommand,

    // When should this command handle the request?
    can_handle: |config: &TextToolConfig| {
        config.count
    },

    // What should this command do?
    execute: |config: &TextToolConfig| {
        for_each_input(config, |reader, _path| {
            let count: usize = reader.lines().count();

            if config.verbosity() > 0 {
                println!("Total lines: {}", count);
            } else {
                println!("{}", count);
            }

            Ok(())
        })
    }
}

cli_command! {
    name: ReverseCommand,

    can_handle: |config: &TextToolConfig| config.reverse,

    execute: |config: &TextToolConfig| {
        for_each_input(config, |reader, _path| {
            let lines: Result<Vec<_>, _> = reader.lines().collect();
            let lines = lines?;

            for line in lines.iter().rev() {
                println!("{}", line);
            }

            Ok(())
        })
    }
}

cli_command! {
    name: UpperCommand,

    can_handle: |config: &TextToolConfig| config.upper,

    execute: |config: &TextToolConfig| {
        for_each_input(config, |reader, _path| {
            for line in reader.lines() {
                println!("{}", line?.to_uppercase());
            }

            Ok(())
        })
    }
}

cli_command! {
    name: GrepCommand,

    can_handle: |config: &TextToolConfig| config.pattern.is_some(),

    execute: |config: &TextToolConfig| {
        let pattern = config.pattern.as_ref().unwrap();

        for_each_input(config, |reader, path| {
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

            Ok(())
        })
    }
}

cli_command! {
    name: CopyCommand,

    can_handle: |_config: &TextToolConfig| true,  // Default handler

    execute: |config: &TextToolConfig| {
        for_each_input(config, |reader, _path| {
            for line in reader.lines() {
                println!("{}", line?);
            }

            Ok(())
        })
    }
}

// ============================================================================
// HELPER FUNCTIONS - Shared logic
// ============================================================================

/// Process each input file/stdin with the given function
fn for_each_input<F>(config: &TextToolConfig, mut process: F) -> CliResult
where
    F: FnMut(BufReader<Box<dyn Read>>, &Path) -> CliResult,
{
    if let Some(inputs) = &config.base.input {
        for path in inputs {
            if config.is_dry_run() {
                println!("Would process: {}", path.display());
                continue;
            }

            if config.verbosity() > 0 {
                eprintln!("Processing: {}", path.display());
            }

            let file = File::open(path)?;
            let reader = BufReader::new(Box::new(file) as Box<dyn Read>);
            process(reader, path)?;
        }
    } else {
        // Read from stdin
        if config.verbosity() > 0 {
            eprintln!("Reading from stdin...");
        }

        let stdin = std::io::stdin();
        let reader = BufReader::new(Box::new(stdin) as Box<dyn Read>);
        process(reader, Path::new("<stdin>"))?;
    }

    Ok(())
}
```

## What Gets Generated by the Macros

### The `cli_app!` macro generates:

1. **Config Struct**:
```rust
pub struct TextToolConfig {
    pub base: BaseConfig,     // -v, -n, -h, -V, -q, -i, -o, -a
    pub pattern: Option<String>,
    pub count: bool,
    pub reverse: bool,
    pub upper: bool,
}
```

2. **CliConfig Trait Implementation**:
```rust
impl CliConfig for TextToolConfig {
    fn base(&self) -> &BaseConfig { &self.base }
    // ... all the standard methods
}
```

3. **Clap Command Builder**:
```rust
fn build_cli() -> clap::Command {
    Command::new("text-tool")
        .version("0.1.0")
        .about("A text processing utility...")
        // All standard flags: -v, -n, -h, -V, -q, -i, -o, -a
        // All custom flags: -p/--pattern, --count, --reverse, --upper
}
```

4. **Config Parser**:
```rust
fn parse_config(matches: clap::ArgMatches) -> TextToolConfig {
    // Parses all standard + custom flags into config struct
}
```

5. **Main Function**:
```rust
fn main() {
    let matches = build_cli().get_matches();
    let config = parse_config(matches);

    let dispatcher = Dispatcher::new()
        .register(VersionCommand::new())   // Built-in
        .register(HelpCommand::new(...))   // Built-in
        .register(CountCommand)            // Your command
        .register(ReverseCommand)          // Your command
        .register(UpperCommand)            // Your command
        .register(GrepCommand)             // Your command
        .register(CopyCommand);            // Your command

    if let Err(e) = dispatcher.dispatch(&config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

### The `cli_command!` macro generates:

For each command, it creates a struct with the Command trait:

```rust
pub struct CountCommand;

impl Command for CountCommand {
    fn can_handle(&self, config: &dyn CliConfig) -> bool {
        let config = config.as_any()
            .downcast_ref::<TextToolConfig>()
            .expect("Invalid config type");
        // Your can_handle logic here
    }

    fn execute(&self, config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
        let config = config.as_any()
            .downcast_ref::<TextToolConfig>()
            .expect("Invalid config type");
        // Your execute logic here
    }
}
```

## Usage Examples

```bash
# Show version (handled by built-in VersionCommand)
$ text-tool --version
Version: 0.1.0
Copyright (c) 2025 Software Wrighter
MIT License: https://github.com/...
Build: abc123d @ hostname (2025-11-22T...)

# Show help (handled by built-in HelpCommand)
$ text-tool --help
A text processing utility demonstrating sw-cli framework
...

# Count lines (handled by CountCommand)
$ text-tool --count -i file.txt
42

# Count with verbose output
$ text-tool -v --count -i file.txt
Processing: file.txt
Total lines: 42

# Grep for pattern (handled by GrepCommand)
$ text-tool --pattern "error" -i logs.txt
Error: connection failed
Error: timeout occurred

# Grep with verbose (shows filename)
$ text-tool -v --pattern "error" -i logs.txt
logs.txt: Error: connection failed
logs.txt: Error: timeout occurred

# Reverse lines (handled by ReverseCommand)
$ text-tool --reverse -i file.txt
last line
middle line
first line

# Convert to uppercase (handled by UpperCommand)
$ text-tool --upper -i file.txt
ALL TEXT BECOMES UPPERCASE

# Dry-run mode (config.is_dry_run() = true)
$ text-tool -n --count -i file.txt
Would process: file.txt

# Read from stdin (handled by CopyCommand, default)
$ echo "hello" | text-tool
hello

# Process multiple files
$ text-tool --count -i file1.txt -i file2.txt -i file3.txt
42
17
89

# Quiet mode (minimal output)
$ text-tool -q --count -i file.txt
42
```

## Key Advantages

### 1. **Minimal Boilerplate**
- No manual clap setup
- No manual config parsing
- No manual dispatch logic

### 2. **Type Safety**
```rust
// Config is strongly typed
let pattern: Option<String> = config.pattern;

// Not string-based like:
let pattern = matches.get_one::<String>("pattern");
```

### 3. **Standard Behavior**
All CLIs automatically get:
- `-v/--verbose` - Increase output detail
- `-n/--dry-run` - Show what would be done
- `-q/--quiet` - Minimize output
- `-h/--help` - Show help (auto-generated)
- `-V/--version` - Show version (from build metadata)
- `-i/--input` - Input file(s), repeatable
- `-o/--output` - Output file
- `-a/--append` - Append to output file

### 4. **Extensible Commands**
```rust
// Add new command by:
// 1. Adding field to args block
// 2. Creating command struct
// 3. Registering in commands list

args: {
    newfeature: bool => (long = "new-feature", help = "New feature"),
}

commands: [
    NewFeatureCommand,  // Just add it here
    // ... existing commands
]
```

### 5. **Clear Separation**
- **Config**: What was requested
- **Commands**: What to do about it
- **Helpers**: How to do it

### 6. **Easy Testing**
```rust
#[test]
fn test_count_command() {
    let config = TextToolConfig {
        base: BaseConfig::default(),
        count: true,
        pattern: None,
        reverse: false,
        upper: false,
    };

    let cmd = CountCommand;
    assert!(cmd.can_handle(&config));
}
```

## File Structure

```
my-cli/
├── COPYRIGHT              # Required by sw-cli
├── Cargo.toml            # Must have version, license, repository
├── build.rs              # sw_cli::define_build_info!();
└── src/
    └── main.rs           # Your ~50 lines using cli_app! macro
```

That's it. No complex project structure needed.
