# Version Module Usage Guide

This guide explains how to integrate the `sw-cli` crate into your Rust CLI projects to provide standardized version information output.

## Adding sw-cli as a Git Submodule

### 1. Add the Submodule

From your CLI project root, add `sw-cli` as a submodule:

```bash
git submodule add https://github.com/softwarewrighter/sw-cli.git lib/sw-cli
git submodule update --init --recursive
```

This creates the submodule in the `lib/sw-cli` directory of your project.

### 2. Add Dependency in Cargo.toml

Add `sw-cli` as a path dependency in your `Cargo.toml`:

```toml
[dependencies]
sw-cli = { path = "lib/sw-cli" }
```

## Using the Version Module

### Basic Usage

In your CLI application's `main.rs`:

```rust
use sw_cli::version::{BuildInfo, Version};

fn main() {
    // Handle -V or --version flags
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "-V" || args[1] == "--version") {
        let version = create_version_info();
        println!("{}", version);
        return;
    }

    // Rest of your CLI logic
}

fn create_version_info() -> Version {
    let build_info = BuildInfo::new(
        env!("BUILD_HOST").to_string(),
        env!("GIT_COMMIT_SHA").to_string(),
        env!("BUILD_TIMESTAMP").parse().unwrap(),
    );

    Version::new(
        format!("Copyright (c) {} Your Name or Organization", env!("BUILD_YEAR")),
        "https://github.com/yourusername/yourrepo/blob/main/LICENSE".to_string(),
        build_info,
    )
}
```

### Setting Build-Time Environment Variables

Create a `build.rs` file in your project root to capture build information:

```rust
use std::process::Command;

fn main() {
    // Get hostname
    let hostname = Command::new("hostname")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=BUILD_HOST={}", hostname);

    // Get git commit SHA
    let commit_sha = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=GIT_COMMIT_SHA={}", commit_sha);

    // Get build timestamp (milliseconds since epoch)
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

    // Get current year for copyright
    let year = chrono::Utc::now().year();
    println!("cargo:rustc-env=BUILD_YEAR={}", year);

    // Re-run if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
}
```

If using `chrono` for the year, add it to your `Cargo.toml` build dependencies:

```toml
[build-dependencies]
chrono = "0.4"
```

Alternatively, you can hardcode the year in your `main.rs` instead of using `env!("BUILD_YEAR")`.

### Example Output

When a user runs your CLI with `-V` or `--version`, they will see:

```
Copyright (c) 2024 Your Name or Organization
License: https://github.com/yourusername/yourrepo/blob/main/LICENSE

Build Information:
  Host: builder.local
  Commit: abc123def456789...
  Timestamp: 1732291234567 ms
```

## Integration with CLI Argument Parsers

### Using with `clap`

```rust
use clap::Parser;
use sw_cli::version::{BuildInfo, Version};

#[derive(Parser)]
#[command(name = "mycli")]
#[command(version = create_version_string())]
struct Cli {
    // Your CLI arguments
}

fn create_version_string() -> String {
    let build_info = BuildInfo::new(
        env!("BUILD_HOST").to_string(),
        env!("GIT_COMMIT_SHA").to_string(),
        env!("BUILD_TIMESTAMP").parse().unwrap(),
    );

    let version = Version::new(
        format!("Copyright (c) {} Your Name", env!("BUILD_YEAR")),
        "https://github.com/yourusername/yourrepo/blob/main/LICENSE".to_string(),
        build_info,
    );

    format!("{}", version)
}

fn main() {
    let cli = Cli::parse();
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
