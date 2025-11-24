# Macro CLI Demo

A demonstration of the `cli_app!()` and `cli_command!()` macros for building CLI applications with minimal boilerplate.

## Quick Start

```bash
# Build and run
cargo build -p macro-cli-demo
cargo run -p macro-cli-demo -- --version
cargo run -p macro-cli-demo -- --count -i file.txt
```

## Boilerplate Reduction

This example implements the same functionality as `working-cli-demo` but with **62% less boilerplate**:

### Manual Implementation (working-cli-demo)
- **Config**: 23 lines (config.rs)
- **Builder**: 28 lines (builder.rs)
- **Args**: 32 lines (args.rs)
- **Commands**: ~100 lines of boilerplate (struct defs + trait impls)
- **Total Framework Code**: ~183 lines

### Macro-Based Implementation (macro-cli-demo)
- **Config + Builder**: ~10 lines (`cli_app!` macro)
- **Commands**: ~60 lines (`cli_command!` macros)
- **Total Framework Code**: ~70 lines

**Result**: From 183 lines → 70 lines (62% reduction)

## Code Comparison

### Defining Config and Builder

**Manual (working-cli-demo)**:
```rust
// config.rs (23 lines)
#[derive(Debug, Clone)]
pub struct DemoConfig {
    pub base: BaseConfig,
    pub input: Option<Vec<PathBuf>>,
    pub output: Option<PathBuf>>,
    pub pattern: Option<String>,
    pub count: bool,
    pub reverse: bool,
}

impl CliConfigTrait for DemoConfig {
    fn base(&self) -> &BaseConfig { &self.base }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

// builder.rs (28 lines)
pub fn build_cli() -> Command {
    Command::new("working-cli-demo")
        .disable_version_flag(true)
        .disable_help_flag(true)
        .about("...")
        .args(sw_cli::builder::standard_args())
        .args(args::custom_args())
}

pub fn parse_config(matches: &ArgMatches) -> DemoConfig {
    DemoConfig {
        base: sw_cli::builder::parse_base_config(matches),
        input: matches.get_many::<String>("input")...
        // ... more parsing
    }
}

// args.rs (32 lines)
pub fn custom_args() -> Vec<Arg> {
    vec![
        Arg::new("input")
            .short('i')
            .long("input")
            .action(ArgAction::Append)
            .help("Input file(s)"),
        // ... more args
    ]
}
```

**Macro-Based (macro-cli-demo)**:
```rust
// Just 10 lines!
cli_app! {
    name: "macro-cli-demo",
    about: "Macro-based CLI demo",
    config: DemoConfig,
    fields: {
        input: Option<Vec<PathBuf>>, short = 'i', long = "input", help = "Input file(s)", action = Append,
        output: Option<PathBuf>, short = 'o', long = "output", help = "Output file",
        pattern: Option<String>, short = 'p', long = "pattern", help = "Pattern to search for",
        count: bool, long = "count", help = "Count lines",
        reverse: bool, long = "reverse", help = "Reverse lines",
    }
}
```

### Defining Commands

**Manual (working-cli-demo)**:
```rust
// count.rs (59 lines)
pub struct CountCommand;

impl Command for CountCommand {
    fn can_handle(&self, config: &dyn CliConfig) -> bool {
        config.as_any()
            .downcast_ref::<DemoConfig>()
            .is_some_and(|c| c.count)
    }

    fn execute(&self, config: &dyn CliConfig) -> Result<(), Box<dyn Error>> {
        let demo_config = config.as_any()
            .downcast_ref::<DemoConfig>()
            .unwrap();

        if let Some(inputs) = &demo_config.input {
            for path in inputs {
                count_file(path, config)?;
            }
        } else {
            count_stdin(config)?;
        }
        Ok(())
    }
}
```

**Macro-Based (macro-cli-demo)**:
```rust
// Just 15 lines!
cli_command! {
    name: CountCommand,
    config: DemoConfig,
    can_handle: |c: &DemoConfig| c.count,
    execute: |config: &DemoConfig| {
        if let Some(inputs) = &config.input {
            for path in inputs {
                count_file(path, config)?;
            }
        } else {
            count_stdin(config)?;
        }
        Ok(())
    }
}
```

## Features

All standard CLI features work out of the box:

- ✅ **Version**: `--version` / `-V`
- ✅ **Help**: `-h` (short) / `--help` (long)
- ✅ **Verbose**: `-v` / `--verbose`
- ✅ **Dry-run**: `-n` / `--dry-run`
- ✅ **Input/Output**: `-i` / `-o` with stdin/stdout support
- ✅ **Custom Commands**: count, grep, reverse, copy
- ✅ **Command Priority**: Automatic dispatcher routing

## Usage Examples

```bash
# Version information
$ macro-cli-demo --version
Version: 0.1.0
Copyright (c) 2025 Software Wrighter
MIT License: https://github.com/softwarewrighter/sw-cli/blob/main/LICENSE
Build: b916833 @ runsc (2025-11-24T18:54:39.066+00:00)

# Count lines
$ echo -e "line1\nline2\nline3" | macro-cli-demo --count
3

# Grep pattern
$ echo -e "hello\nworld\ntest" | macro-cli-demo -p "world"
world

# Reverse lines
$ echo -e "line1\nline2\nline3" | macro-cli-demo --reverse
line3
line2
line1

# Verbose + Dry-run
$ macro-cli-demo -v -n --count -i file.txt
Would count lines in: file.txt
```

## Project Structure

```
macro-cli-demo/
├── src/
│   ├── main.rs              # 200 lines (vs 400 in working-cli-demo)
│   ├── short-help.txt       # Short help text
│   └── long-help.txt        # Detailed help text
├── build.rs                 # 3 lines
├── COPYRIGHT                # Copyright notice
└── Cargo.toml               # Package manifest
```

## Key Benefits

1. **Less Boilerplate**: 62% reduction in framework code
2. **Single File**: Everything in one `main.rs` instead of 15 files
3. **Clearer Intent**: Declarative macro syntax vs imperative trait impls
4. **Type Safety**: Full type checking, no stringly-typed lookups
5. **Standard Features**: All sw-cli features work automatically
6. **Easy Maintenance**: Less code = easier to read and modify

## Comparison Table

| Aspect | Working-CLI-Demo | Macro-CLI-Demo | Reduction |
|--------|------------------|----------------|-----------|
| Files | 15 files | 1 file | 93% |
| Lines (total) | ~400 lines | ~200 lines | 50% |
| Lines (framework) | ~183 lines | ~70 lines | 62% |
| Config definition | 23 lines | Part of 10 lines | ~95% |
| Builder | 28 lines | Part of 10 lines | ~95% |
| Per-command overhead | ~25 lines | ~5 lines | 80% |

## When to Use Each Approach

### Use Macros When:
- Building a new CLI from scratch
- Standard patterns fit your needs
- You want minimal boilerplate
- Single-file projects are acceptable

### Use Manual Approach When:
- Very complex custom logic needed
- Non-standard argument patterns
- You need fine-grained control
- Team prefers explicit code over macros

## Next Steps

To convert your own CLI to use macros:

1. Replace config/builder/args modules with `cli_app!`
2. Replace command trait impls with `cli_command!`
3. Keep helper functions as-is
4. Test thoroughly!

See `working-cli-demo` for the equivalent manual implementation.
