# Mini CLI Demo

The **simplest possible** example of using `cli_app!()` and `cli_command!()` macros.

## Overview

This example demonstrates how to build a complete CLI application in just **~80 lines of code** using sw-cli macros.

### Features

- ✅ Echo text to output
- ✅ Convert text to uppercase
- ✅ Repeat text N times
- ✅ All standard flags (version, help, verbose, dry-run)
- ✅ Proper command dispatch

### Code Size

```
mini-cli-demo:    ~80 lines total
Traditional CLI:  ~150+ lines

Savings: 47% less code!
```

## Quick Start

```bash
# Build and run
cargo build -p mini-cli-demo
cargo run -p mini-cli-demo -- --version

# Echo text (default)
cargo run -p mini-cli-demo -- --text "Hello, World!"

# Uppercase conversion
cargo run -p mini-cli-demo -- -u --text "hello"

# Repeat text
cargo run -p mini-cli-demo -- -r 3 --text "test"

# Verbose mode
cargo run -p mini-cli-demo -- -v --text "test"

# Dry-run mode
cargo run -p mini-cli-demo -- -n -r 5 --text "test"
```

## Complete Source Code

The entire application is in `src/main.rs`:

```rust
use sw_cli::{cli_app, cli_command, dispatch, CliConfig};

// Define CLI config - just 6 lines!
cli_app! {
    name: "mini-cli-demo",
    about: "Minimal CLI example",
    config: MiniConfig,
    fields: {
        text: Option<String>, short = 't', long = "text", help = "Text to process",
        uppercase: bool, short = 'u', long = "uppercase", help = "Convert to uppercase",
        repeat: Option<usize>, short = 'r', long = "repeat", help = "Repeat N times",
    }
}

// Define commands - ~10 lines each
cli_command! {
    name: UppercaseCommand,
    config: MiniConfig,
    can_handle: |c: &MiniConfig| c.uppercase,
    execute: |config: &MiniConfig| {
        let text = config.text.as_deref().unwrap_or("Hello, World!");
        println!("{}", text.to_uppercase());
        Ok(())
    }
}

// ... more commands ...

fn main() {
    let matches = build_cli().get_matches();
    let config = parse_config(&matches);
    let dispatcher = dispatch!(UppercaseCommand, RepeatCommand, EchoCommand);

    if let Err(e) = dispatcher.dispatch(&config) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
```

That's it! A complete, production-ready CLI in ~80 lines.

## What Gets Generated

The `cli_app!` macro generates:

```rust
// 1. Config struct
pub struct MiniConfig {
    pub base: BaseConfig,
    pub text: Option<String>,
    pub uppercase: bool,
    pub repeat: Option<usize>,
}

// 2. CliConfig trait implementation
impl CliConfig for MiniConfig { ... }

// 3. Clap builder
pub fn build_cli() -> Command { ... }

// 4. Parser
pub fn parse_config(matches: &ArgMatches) -> MiniConfig { ... }
```

The `cli_command!` macro generates:

```rust
// 1. Command struct
pub struct UppercaseCommand;

// 2. Command trait implementation with type-safe downcasting
impl Command for UppercaseCommand {
    fn can_handle(&self, config: &dyn CliConfig) -> bool { ... }
    fn execute(&self, config: &dyn CliConfig) -> Result<...> { ... }
}
```

## Example Output

```bash
$ mini-cli-demo --version
Version: 0.1.0
Copyright (c) 2025 Software Wrighter
MIT License: https://github.com/softwarewrighter/sw-cli/blob/main/LICENSE
Build: 65ca719 @ hostname (2025-11-24T19:10:00.000+00:00)

$ mini-cli-demo --text "hello"
hello

$ mini-cli-demo -u --text "hello"
HELLO

$ mini-cli-demo -r 3 --text "test"
test
test
test

$ mini-cli-demo -v -u --text "hello"
Converting to uppercase...
HELLO

$ mini-cli-demo -n -r 5 --text "test"
Would repeat 'test' 5 times
```

## Benefits

### 1. Extremely Concise

- **Config + Builder**: 6 lines (vs ~50 lines manually)
- **Commands**: ~10 lines each (vs ~30 lines manually)
- **Total**: ~80 lines (vs ~150+ lines manually)

### 2. Type Safe

- Full type checking at compile time
- No stringly-typed argument lookups
- IDE autocomplete works perfectly

### 3. Standard Features

All sw-cli features work automatically:
- Version info (`--version`)
- Two-level help (`-h` / `--help`)
- Verbose mode (`-v`)
- Dry-run mode (`-n`)

### 4. Easy to Extend

Adding a new command is just ~10 more lines:

```rust
cli_command! {
    name: LowercaseCommand,
    config: MiniConfig,
    can_handle: |c: &MiniConfig| c.lowercase,
    execute: |config: &MiniConfig| {
        let text = config.text.as_deref().unwrap_or("Hello");
        println!("{}", text.to_lowercase());
        Ok(())
    }
}
```

Add the field to `cli_app!`:

```rust
fields: {
    // ... existing fields ...
    lowercase: bool, short = 'l', long = "lowercase", help = "Convert to lowercase",
}
```

Done!

## Comparison

| Aspect | Manual Implementation | With Macros | Savings |
|--------|----------------------|-------------|---------|
| Config definition | ~25 lines | 6 lines | 76% |
| Builder | ~30 lines | Part of config | 100% |
| Per command | ~30 lines | ~10 lines | 67% |
| **Total** | **~150 lines** | **~80 lines** | **47%** |

## Learning Path

1. Start here with `mini-cli-demo` (simplest)
2. Check `macro-cli-demo` (realistic app with file I/O)
3. Study `working-cli-demo` (manual implementation for comparison)

## When to Use This Pattern

✅ **Use macros when:**
- Building a new CLI from scratch
- You want minimal boilerplate
- Standard patterns fit your needs
- Rapid prototyping

❌ **Use manual approach when:**
- Very complex custom logic
- Non-standard argument patterns
- You need fine-grained control
- Team prefers explicit code

## Next Steps

To build your own CLI:

1. Copy this example as a template
2. Modify the `fields` in `cli_app!`
3. Add your own `cli_command!` implementations
4. Enjoy the reduced boilerplate!

See also:
- `macro-cli-demo` - More realistic example with file I/O
- `working-cli-demo` - Manual implementation for comparison
- `docs/` - Architecture documentation
