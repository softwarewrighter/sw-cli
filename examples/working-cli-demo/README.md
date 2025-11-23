# Working CLI Demo - Builder-Config-Dispatcher Pattern

This is a **WORKING** example that demonstrates the Builder-Config-Dispatcher pattern using current sw-cli capabilities.

## What This Demonstrates

This example shows the architecture that the future `cli_app!` macro will generate automatically. It implements:

1. **BaseConfig**: Standard flags common to all CLIs
   - `-v, --verbose` - Verbose output
   - `-n, --dry-run` - Show what would be done
   - `-q, --quiet` - Minimal output
   - `-h, --help` - Show help
   - `-V, --version` - Show version (uses `sw_cli::version!()`)
   - `-i, --input` - Input files (repeatable)
   - `-o, --output` - Output file

2. **Custom Config**: CLI-specific fields
   - `--count` - Count lines
   - `-p, --pattern` - Search pattern
   - `--reverse` - Reverse lines

3. **Builder Pattern**: Constructing clap Command
   - Standard flags added manually (will be macro-generated)
   - Custom flags defined per-CLI

4. **Dispatcher**: Chain of responsibility
   - Commands checked in priority order
   - First matching command executes

5. **Commands**: Modular command handlers
   - `VersionCommand` (priority 0) - Uses `sw_cli::version!()`
   - `HelpCommand` (priority 1) - Shows clap-generated help
   - `CountCommand` - Counts lines in files
   - `GrepCommand` - Searches for pattern
   - `ReverseCommand` - Reverses line order
   - `CopyCommand` - Default, copies input to output

## Usage Examples

```bash
# Show version (uses sw-cli version! macro)
cargo run -p working-cli-demo -- --version

# Show help
cargo run -p working-cli-demo -- --help

# Count lines in file
echo -e "line1\nline2\nline3" > /tmp/test.txt
cargo run -p working-cli-demo -- --count -i /tmp/test.txt
# Output: 3

# Count with verbose output
cargo run -p working-cli-demo -- -v --count -i /tmp/test.txt
# Output:
# Processing: /tmp/test.txt
# /tmp/test.txt: 3 lines

# Search for pattern
cargo run -p working-cli-demo -- -p "line2" -i /tmp/test.txt
# Output: line2

# Search with verbose (shows filename)
cargo run -p working-cli-demo -- -v -p "line2" -i /tmp/test.txt
# Output: /tmp/test.txt: line2

# Reverse lines
cargo run -p working-cli-demo -- --reverse -i /tmp/test.txt
# Output:
# line3
# line2
# line1

# Dry-run mode (shows what would be done)
cargo run -p working-cli-demo -- -n --count -i /tmp/test.txt
# Output: Would count lines in: /tmp/test.txt

# Read from stdin
echo -e "hello\nworld" | cargo run -p working-cli-demo
# Output:
# hello
# world

# Read from stdin and count
echo -e "hello\nworld" | cargo run -p working-cli-demo -- --count
# Output: 2
```

## Architecture Pattern

This example shows the manual implementation. Compare this with what the future macro will generate:

### Manual (Current - ~450 lines)

```rust
// Define BaseConfig struct
pub struct BaseConfig { ... }

// Define CLI-specific config
pub struct CliConfig {
    pub base: BaseConfig,
    pub pattern: Option<String>,
    pub count: bool,
    pub reverse: bool,
}

// Build clap Command manually
fn build_cli() -> Command {
    Command::new("working-cli-demo")
        .arg(Arg::new("verbose").short('v')...)
        .arg(Arg::new("dry-run").short('n')...)
        // ... 50+ lines of arg definitions
}

// Parse config manually
fn parse_config(matches: ArgMatches) -> CliConfig { ... }

// Define each command struct and trait impl
struct CountCommand;
impl CliCommand for CountCommand { ... }

struct GrepCommand;
impl CliCommand for GrepCommand { ... }

// ... more commands

// Build dispatcher and wire everything together
fn main() {
    let matches = build_cli().get_matches();
    let config = parse_config(matches);
    let dispatcher = Dispatcher::new()
        .register(VersionCommand)
        // ... register all commands
    dispatcher.dispatch(&config);
}
```

### With Macro (Future - ~50 lines)

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
    execute: |c| { /* count logic */ }
}

// ... other commands
```

The macro generates all the boilerplate!

## Code Organization

```
working-cli-demo/
├── src/
│   └── main.rs           # Complete implementation
├── build.rs              # sw_cli::define_build_info!()
├── COPYRIGHT             # Required by sw-cli
├── Cargo.toml            # Dependencies: sw-cli, clap
└── README.md             # This file
```

## Key Takeaways

1. **Version Info is Easy**: Just call `sw_cli::version!()` in VersionCommand
2. **Dispatcher Pattern Works**: Clean separation of concerns
3. **Standard Flags are Repetitive**: Perfect candidate for macro generation
4. **Chain of Responsibility is Powerful**: Priority-based command routing
5. **Type-Safe Config**: No string-based lookups, all strongly typed

## Next Steps

This example demonstrates what needs to be implemented:

1. **Phase 1**: Create `BaseConfig` in sw-cli library
2. **Phase 2**: Create helper functions for standard flags
3. **Phase 3**: Create `Dispatcher` and `Command` trait
4. **Phase 4**: Create `cli_app!` macro to generate all this boilerplate
5. **Phase 5**: Create `cli_command!` macro to simplify command definitions

Once complete, users will write ~50 lines instead of ~450!
