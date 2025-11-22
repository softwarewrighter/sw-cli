# Version Module Usage Guide

This guide explains how to integrate the `sw-cli` crate into your Rust CLI projects to provide standardized version information output.

## Quick Start

The `sw-cli` repository includes helper scripts for common tasks:

```bash
# Build all crates in the workspace
./scripts/build.sh

# Run the demo example (shows macro usage)
./scripts/demo.sh

# Run all tests and checks
./scripts/test.sh
```

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

### Recommended: Using Macros (Simplest Approach)

The easiest way to add version information is using the provided macros:

#### 1. Add build.rs to your project

```rust
use sw_cli::define_build_info;

fn main() {
    define_build_info!();
}
```

This single macro call automatically captures hostname, git commit SHA, and build timestamp.

#### 2. Use the macro in main.rs

```rust
use sw_cli::create_version;

fn main() {
    // Handle -V or --version flags
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "-V" || args[1] == "--version") {
        let version = create_version!(
            copyright: "Copyright (c) 2025 Your Name or Organization",
            license_name: "MIT",
            license_url: "https://github.com/yourusername/yourrepo/blob/main/LICENSE"
        );
        println!("{}", version);
        return;
    }

    // Rest of your CLI logic
}
```

See the `examples/demo-cli/` directory for a complete working example that demonstrates the macro usage.

To run the example:
```bash
./scripts/demo.sh
```

### Alternative: Manual Construction (More Control)

If you need more control over the build process, you can manually construct the version info:

#### 1. Create build.rs manually

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

    // Re-run if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
}
```

#### 2. Manually construct Version in main.rs

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
        "Copyright (c) 2025 Your Name or Organization".to_string(),
        "MIT".to_string(),
        "https://github.com/yourusername/yourrepo/blob/main/LICENSE".to_string(),
        build_info,
    )
}
```

### Example Output

When a user runs your CLI with `-V` or `--version`, they will see:

```
Version: 0.1.0
Copyright (c) 2025 Your Name or Organization
MIT License: https://github.com/yourusername/yourrepo/blob/main/LICENSE
Build: abc123d @ builder.local (2024-11-22T15:33:54+00:00)
```

## Integration with CLI Argument Parsers

### Using with `clap`

With macros:

```rust
use clap::Parser;
use sw_cli::create_version;

#[derive(Parser)]
#[command(name = "mycli")]
#[command(version = create_version_string())]
struct Cli {
    // Your CLI arguments
}

fn create_version_string() -> String {
    let version = create_version!(
        copyright: "Copyright (c) 2025 Your Name",
        license_name: "MIT",
        license_url: "https://github.com/yourusername/yourrepo/blob/main/LICENSE"
    );
    format!("{}", version)
}

fn main() {
    let cli = Cli::parse();
    // Your CLI logic
}
```

Or manually:

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
        env!("CARGO_PKG_VERSION").to_string(),
        "Copyright (c) 2025 Your Name".to_string(),
        "MIT".to_string(),
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
