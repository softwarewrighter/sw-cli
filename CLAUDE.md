# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`sw-cli` is a Rust library designed to be used as a git submodule in CLI projects to provide standardized version information output. It provides a streamlined 4-step integration approach that generates formatted version information with build metadata.

## Workspace Structure

This is a Cargo workspace with three crates:

- **sw-cli** (main crate at root): Re-exports macros and contains the `version` module with `Version` and `BuildInfo` structs
- **sw-cli-macros** (procedural macro crate): Contains `version!()`, `define_build_info!()`, and deprecated `create_version!()` macros
- **examples/demo-cli** (example implementation): Demonstrates the 4-step integration pattern

**Important**: Procedural macros must be in a separate crate (Rust requirement). The main crate re-exports them for convenience, so users only depend on `sw-cli` and get everything.

## Architecture: How the Macros Work Together

### Build-time Flow (define_build_info! macro)

1. Called in user's `build.rs`
2. Reads `COPYRIGHT` file from project root
3. Extracts `version`, `license`, `repository` from `CARGO_PKG_*` env vars
4. Executes shell commands to get hostname and git commit SHA
5. Generates current timestamp
6. **Writes `version_info.rs` to `$OUT_DIR`** with const values:
   - `VERSION`, `COPYRIGHT`, `LICENSE_NAME`, `LICENSE_URL`
   - `BUILD_HOST`, `GIT_COMMIT_SHA`, `BUILD_TIMESTAMP`

### Runtime Flow (version! macro)

1. Called in user's code where version string is needed
2. Uses `include!()` to load generated `version_info.rs` from `$OUT_DIR`
3. Constructs `BuildInfo` and `Version` objects from the const values
4. Returns formatted string using `Display` trait (4 lines):
   - Version: X.Y.Z
   - Copyright notice
   - License name and URL
   - Build: short-SHA @ hostname (ISO 8601 timestamp)

### Key Design Decision

The macro generates a file in `$OUT_DIR` rather than using `cargo:rustc-env` because it allows reading external files (COPYRIGHT) and provides better separation between build-time and compile-time data generation.

## Development Commands

### Building
```bash
# Build all workspace crates
./scripts/build.sh

# Or build specific crate
cargo build -p sw-cli
cargo build -p sw-cli-macros
cargo build -p demo-cli
```

### Testing
```bash
# Run all tests and checks (includes fmt and clippy)
./scripts/test.sh

# Run tests for specific crate
cargo test -p sw-cli
cargo test -p sw-cli-macros

# Run single test
cargo test -p sw-cli test_version_display
```

### Demo
```bash
# Run demo showing version output
./scripts/demo.sh

# Or run directly
cargo run -p demo-cli -- --version
```

### Code Quality
```bash
# Format code
cargo fmt --all

# Check formatting without changes
cargo fmt --all -- --check

# Run clippy with warnings as errors
cargo clippy --all -- -D warnings
```

## Version Output Format

The library enforces a consistent 4-line version output format:
```
Version: 0.1.0
Copyright (c) 2025 Software Wrighter
MIT License: https://github.com/user/repo/blob/main/LICENSE
Build: abc123d @ hostname (2025-11-22T22:43:30.729+00:00)
```

- Git SHA is truncated to 7 characters (see `src/version.rs:30-34`)
- Timestamp is ISO 8601 format via chrono's `to_rfc3339()`
- All formatting is in `Display` trait implementations

## User Integration Requirements

When users integrate sw-cli, they must have:

1. **COPYRIGHT file** in project root (read by `define_build_info!()`)
2. **Cargo.toml fields**: `version`, `license`, `repository`
3. **build-dependencies** section with `sw-cli`
4. Git repository (for commit SHA) - falls back to "unknown" if not available

## Important Notes for Development

- **Edition**: Cargo.toml currently uses `edition = "2024"` which may not be stable - verify this is intentional
- **Backward compatibility**: `create_version!()` macro is deprecated but kept for compatibility - do not remove
- **Macro expansion**: The `version!()` macro creates an anonymous module `__version_info` to avoid namespace pollution
- **Generated files**: `version_info.rs` files in `target/` are build artifacts and should not be committed
- **No rerun-if-changed**: The macro deliberately does NOT use `cargo:rerun-if-changed` because each build should get a unique timestamp

## Documentation

The primary user-facing documentation is in `docs/version-usage.md`. When making changes to the macro API or integration steps, update this file to reflect the 4-step approach.
