# Version Module Usage Guide

This guide explains how to integrate the `sw-cli` crate into your Rust CLI projects to provide standardized version information output.

## Quick Start (4 Steps)

Using sw-cli in a CLI project requires just 4 steps:

### 1. Add sw-cli as a submodule
```bash
git submodule add https://github.com/softwarewrighter/sw-cli.git lib/sw-cli
git submodule update --init --recursive
```

### 2. Update Cargo.toml dependencies
```toml
[package]
name = "your-cli"
version = "0.1.0"
license = "MIT"
repository = "https://github.com/yourusername/repo"

[dependencies]
sw-cli = { path = "lib/sw-cli" }

[build-dependencies]
sw-cli = { path = "lib/sw-cli" }
```

### 3. Add 1 line to build.rs
```rust
fn main() {
    sw_cli::define_build_info!();
}
```

### 4. Add 1 line to version printing code
```rust
println!("{}", sw_cli::version!());
```

### Prerequisites

**Required Files:**
- **COPYRIGHT** - Required by define_build_info! macro
  ```
  Copyright (c) 2025 Your Name
  ```

**Required Cargo.toml fields:**
- `version` - e.g., "0.1.0"
- `license` - e.g., "MIT"
- `repository` - e.g., "https://github.com/user/repo"

**Optional Files:**
- LICENSE file (not required - only LICENSE URL from repository field is used)
- VERSION file (not needed - uses CARGO_PKG_VERSION from Cargo.toml)

## Example Output

When a user runs your CLI with `-V` or `--version`, they will see a 4-line output:

```
Version: 0.1.0
Copyright (c) 2025 Your Name or Organization
MIT License: https://github.com/yourusername/yourrepo/blob/main/LICENSE
Build: abc123d @ hostname (2025-11-22T15:33:54.062+00:00)
```

## Complete Example

See the `examples/demo-cli/` directory for a complete working example.

The `sw-cli` repository includes helper scripts for common tasks:

```bash
# Build all crates in the workspace
./scripts/build.sh

# Run the demo example (shows macro usage)
./scripts/demo.sh

# Run all tests and checks
./scripts/test.sh
```

### Using with Manual Argument Parsing

```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && (args[1] == "-V" || args[1] == "--version") {
        println!("{}", sw_cli::version!());
        return;
    }

    // Rest of your CLI logic
}
```

## Integration with CLI Argument Parsers

### Using with `clap`

Note: Due to macro expansion limitations, you may need to use a const function approach:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "mycli")]
#[command(long_version = version_string())]
struct Cli {
    // Your CLI arguments
}

fn version_string() -> &'static str {
    // This will be called at runtime when --version is used
    Box::leak(Box::new(sw_cli::version!()))
}

fn main() {
    let cli = Cli::parse();
    // Your CLI logic
}
```

Alternatively, handle version manually before clap parsing:

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "mycli")]
#[command(disable_version_flag = true)]
struct Cli {
    #[arg(short = 'V', long)]
    version: bool,
    // Your other CLI arguments
}

fn main() {
    let cli = Cli::parse();

    if cli.version {
        println!("{}", sw_cli::version!());
        return;
    }

    // Your CLI logic
}
```

## Updating the Submodule

To update `sw-cli` to the latest version:

```bash
cd lib/sw-cli
git pull origin main
cd ../..
git add lib/sw-cli
git commit -m "Update sw-cli submodule"
```

## Cloning Projects with Submodules

When others clone your project, they need to initialize submodules:

```bash
git clone https://github.com/yourusername/yourrepo.git
cd yourrepo
git submodule update --init --recursive
```

Or clone with submodules in one step:

```bash
git clone --recurse-submodules https://github.com/yourusername/yourrepo.git
```
