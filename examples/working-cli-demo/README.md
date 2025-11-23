# Working CLI Demo

A modular, production-ready CLI demonstrating the Builder-Config-Dispatcher pattern.

## Quick Start

```bash
# Build and test
cargo build -p working-cli-demo
cargo test -p working-cli-demo

# Run examples
echo -e "line1\nline2\nline3" > /tmp/test.txt
cargo run -p working-cli-demo -- --version
cargo run -p working-cli-demo -- --count -i /tmp/test.txt
cargo run -p working-cli-demo -- -p "line2" -i /tmp/test.txt
cargo run -p working-cli-demo -- --reverse -i /tmp/test.txt
cargo run -p working-cli-demo -- -v --count -i /tmp/test.txt
cargo run -p working-cli-demo -- -n --count -i /tmp/test.txt
```

## Project Structure

```
working-cli-demo/
├── src/
│   ├── lib.rs              # Public API exports (8 lines)
│   ├── main.rs             # Application entry point (22 lines)
│   ├── config.rs           # Configuration types (29 lines)
│   ├── builder.rs          # Clap CLI builder (59 lines)
│   ├── dispatcher.rs       # Command dispatcher (30 lines)
│   └── actions/
│       ├── mod.rs          # Action module exports (9 lines)
│       ├── version.rs      # Version command (18 lines)
│       ├── help.rs         # Help command (24 lines)
│       ├── count.rs        # Count lines command (53 lines)
│       ├── grep.rs         # Grep command (55 lines)
│       ├── reverse.rs      # Reverse command (36 lines)
│       └── copy.rs         # Copy command (28 lines)
├── tests/
│   ├── config_test.rs      # Config tests (60 lines)
│   └── dispatcher_test.rs  # Dispatcher tests (55 lines)
├── build.rs                # Build script (3 lines)
├── COPYRIGHT               # Copyright notice
├── Cargo.toml              # Package manifest
└── README.md               # This file
```

**Total**: ~400 lines across 15 files (~27 lines/file average)

## Architecture

### 1. Config Layer (`config.rs`)
- `BaseConfig`: Standard flags (-v, -n, -q, -h, -V, -i, -o)
- `CliConfig`: Extends BaseConfig with custom fields (pattern, count, reverse)
- Helper methods: `verbosity()`, `is_dry_run()`

### 2. Builder Layer (`builder.rs`)
- `build_cli()`: Constructs clap Command
- `parse_config()`: Converts ArgMatches → CliConfig
- Helper functions: `standard_args()`, `custom_args()`, `arg()`

### 3. Dispatcher Layer (`dispatcher.rs`)
- `Command` trait: `can_handle()`, `execute()`, `priority()`
- `Dispatcher`: Chain of responsibility pattern
- Commands sorted by priority, first match executes

### 4. Actions Layer (`actions/`)
Each action is a separate module with a single Command implementation:
- **version**: Shows version info (priority 0, uses sw_cli::version!())
- **help**: Shows help text (priority 1)
- **count**: Counts lines in files
- **grep**: Searches for patterns
- **reverse**: Reverses line order
- **copy**: Default action, copies input to output

### 5. Main (`main.rs`)
Minimal orchestration: build CLI → parse config → create dispatcher → dispatch

## Standard Flags (BaseConfig)

All CLIs get these flags automatically:

| Flag | Short | Description |
|------|-------|-------------|
| `--verbose` | `-v` | Increase output verbosity |
| `--dry-run` | `-n` | Show what would be done |
| `--quiet` | `-q` | Suppress non-essential output |
| `--help` | `-h` | Show help information |
| `--version` | `-V` | Show version (from sw-cli) |
| `--input FILE` | `-i` | Input file(s), repeatable |
| `--output FILE` | `-o` | Output file |

## Custom Flags (CliConfig)

This demo adds:

| Flag | Short | Description |
|------|-------|-------------|
| `--pattern PATTERN` | `-p` | Pattern to search for |
| `--count` | | Count lines in input |
| `--reverse` | | Reverse line order |

## Usage Examples

### Version Information
```bash
$ cargo run -p working-cli-demo -- --version
Version: 0.1.0
Copyright (c) 2025 Software Wrighter
MIT License: https://github.com/softwarewrighter/sw-cli/blob/main/LICENSE
Build: 1549013 @ manager (2025-11-23T00:12:45.123+00:00)
```

### Help Text
```bash
$ cargo run -p working-cli-demo -- --help
Builder-Config-Dispatcher pattern demo

Usage: working-cli-demo [OPTIONS]

Options:
  -V, --version            Show version information
  -h, --help               Show help information
  -v, --verbose            Increase output verbosity
  -n, --dry-run            Show what would be done
  -q, --quiet              Suppress non-essential output
  -i, --input <FILE>       Input file(s)
  -o, --output <FILE>      Output file
  -p, --pattern <PATTERN>  Pattern to search for
      --count              Count lines in input
      --reverse            Reverse line order
```

### Count Lines
```bash
# Basic count
$ cargo run -p working-cli-demo -- --count -i /tmp/test.txt
3

# Count with verbose output
$ cargo run -p working-cli-demo -- -v --count -i /tmp/test.txt
Processing: /tmp/test.txt
/tmp/test.txt: 3 lines

# Dry-run mode
$ cargo run -p working-cli-demo -- -n --count -i /tmp/test.txt
Would count lines in: /tmp/test.txt
```

### Search Pattern
```bash
# Basic grep
$ cargo run -p working-cli-demo -- -p "line2" -i /tmp/test.txt
line2

# Grep with filename (verbose)
$ cargo run -p working-cli-demo -- -v -p "line2" -i /tmp/test.txt
/tmp/test.txt: line2
```

### Reverse Lines
```bash
$ cargo run -p working-cli-demo -- --reverse -i /tmp/test.txt
line3
line2
line1
```

### Stdin/Stdout
```bash
# Read from stdin, write to stdout
$ echo -e "hello\nworld" | cargo run -p working-cli-demo
hello
world

# Count lines from stdin
$ echo -e "hello\nworld" | cargo run -p working-cli-demo -- --count
2

# Pipe through grep
$ cat /tmp/test.txt | cargo run -p working-cli-demo -- -p "line2"
line2
```

## Testing

```bash
# Run all tests
cargo test -p working-cli-demo

# Run specific test file
cargo test -p working-cli-demo --test config_test
cargo test -p working-cli-demo --test dispatcher_test

# Run with verbose output
cargo test -p working-cli-demo -- --nocapture
```

## Design Goals Achieved

✅ **Modularity**: 15 files, ~27 lines/file average
✅ **Single Responsibility**: Each file has one clear purpose
✅ **Testability**: Separate test files for each module
✅ **Type Safety**: Strongly-typed config, no string lookups
✅ **Extensibility**: Add commands by creating new files in actions/
✅ **Readability**: No file exceeds 60 lines
✅ **Maintainability**: Clear structure, minimal coupling

## Future: Macro-Based Version

This manual implementation (~400 lines) will be reduced to ~50 lines with macros:

```rust
use sw_cli::prelude::*;

cli_app! {
    name: "working-cli-demo",
    args: {
        pattern: String => (short = 'p'),
        count: bool => (long = "count"),
        reverse: bool => (long = "reverse"),
    },
    commands: [CountCommand, GrepCommand, ReverseCommand, CopyCommand]
}

cli_command! {
    name: CountCommand,
    can_handle: |c| c.count,
    execute: |c| { /* count logic from count.rs */ }
}

// ... other commands
```

The macro will generate: config.rs, builder.rs, dispatcher.rs, and command boilerplate.
