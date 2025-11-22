# Multi-Crate Project Refactoring Plan

## Vision: General-Purpose Toolkit

Transform `sw-cli` from a CLI-specific library into a general-purpose toolkit that supports CLI, web UI, testing, and other contexts.

## New Project Structure

```
sw-toolkit/  (or keep as sw-cli but expand scope)
├── crates/
│   ├── cli-framework/          # CLI-specific utilities
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── config/         # BaseConfig, CliConfig trait
│   │   │   ├── builder/        # Clap builders, standard flags
│   │   │   ├── dispatcher/     # Command dispatcher, chain of responsibility
│   │   │   ├── commands/       # Built-in commands (Version, Help)
│   │   │   ├── macros.rs       # Re-export procedural macros
│   │   │   └── prelude.rs      # Common imports
│   │   └── Cargo.toml
│   │
│   ├── cli-macros/             # Procedural macros for CLI
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── cli_app.rs      # cli_app! macro
│   │   │   ├── cli_command.rs  # cli_command! macro
│   │   │   ├── version.rs      # version! macro (existing)
│   │   │   └── build_info.rs   # define_build_info! macro (existing)
│   │   └── Cargo.toml
│   │
│   ├── version-info/           # Version/build info (shared by CLI and Web)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── version.rs      # Version struct
│   │   │   ├── build.rs        # BuildInfo struct
│   │   │   └── display/        # Display formatters
│   │   │       ├── cli.rs      # CLI text formatting (existing 4-line format)
│   │   │       ├── html.rs     # HTML formatting for web
│   │   │       └── json.rs     # JSON formatting for APIs
│   │   └── Cargo.toml
│   │
│   ├── web-ui-components/      # Web UI utilities (parallel to CLI)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── footer/         # Footer component with version info
│   │   │   │   ├── mod.rs
│   │   │   │   ├── simple.rs   # Simple text footer
│   │   │   │   └── rich.rs     # Rich HTML footer with links
│   │   │   ├── header/         # Header component
│   │   │   └── macros.rs       # web_footer! macro
│   │   └── Cargo.toml
│   │
│   ├── test-fixtures/          # Testing utilities and fixtures
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── builders/       # Test data builders
│   │   │   ├── fixtures/       # Common test fixtures
│   │   │   ├── assertions/     # Custom assertions
│   │   │   └── temp_files/     # Temporary file management
│   │   └── Cargo.toml
│   │
│   └── core-utilities/         # Shared utilities (non-CLI, non-web)
│       ├── src/
│       │   ├── lib.rs
│       │   ├── config/         # Configuration file handling
│       │   ├── logging/        # Logging setup
│       │   ├── error/          # Error types
│       │   └── fs/             # Filesystem utilities
│       └── Cargo.toml
│
├── examples/
│   ├── cli-simple/             # Simple CLI using cli-framework
│   ├── cli-advanced/           # Advanced CLI with custom help
│   ├── web-footer-demo/        # Web UI with footer
│   └── cli-with-tests/         # CLI with test fixtures
│
├── src/
│   └── lib.rs                  # Root crate re-exports
│
├── docs/
│   ├── cli-framework.md
│   ├── web-ui-components.md
│   ├── version-info.md
│   └── architecture.md
│
├── Cargo.toml                  # Workspace definition
└── build.rs                    # Root build script
```

## Crate Descriptions

### 1. `cli-framework` (formerly main sw-cli)

**Purpose**: Complete framework for building CLIs with minimal boilerplate

**Exports**:
```rust
// Config system
pub mod config {
    pub struct BaseConfig { ... }
    pub trait CliConfig { ... }
}

// Builder system
pub mod builder {
    pub fn with_standard_flags(cmd: Command) -> Command;
    pub fn with_io_args(cmd: Command) -> Command;
}

// Dispatcher
pub mod dispatcher {
    pub struct Dispatcher { ... }
    pub trait Command { ... }
}

// Built-in commands
pub mod commands {
    pub struct VersionCommand { ... }
    pub struct HelpCommand { ... }
}

// Macros (re-exported from cli-macros)
pub use cli_macros::{cli_app, cli_command, version, define_build_info};

// Prelude for easy imports
pub mod prelude {
    pub use super::config::*;
    pub use super::builder::*;
    pub use super::dispatcher::*;
    pub use super::commands::*;
    pub use super::{cli_app, cli_command, version, define_build_info};
}
```

**Dependencies**:
```toml
[dependencies]
cli-macros = { path = "../cli-macros" }
version-info = { path = "../version-info" }
clap = "4.0"
```

### 2. `version-info` (shared library)

**Purpose**: Version and build information handling for ANY context (CLI, web, API)

**Exports**:
```rust
pub struct Version {
    pub version: String,
    pub copyright: String,
    pub license_name: String,
    pub license_url: String,
    pub build_info: BuildInfo,
}

pub struct BuildInfo {
    pub build_host: String,
    pub commit_sha: String,
    pub build_timestamp_ms: i64,
}

// Display formatters for different contexts
pub mod display {
    pub mod cli {
        // Existing 4-line CLI format
        pub fn format_version(v: &Version) -> String;
    }

    pub mod html {
        // HTML footer format
        pub fn format_footer(v: &Version) -> String;
        pub fn format_footer_rich(v: &Version, css_class: &str) -> String;
    }

    pub mod json {
        // JSON format for APIs
        pub fn format_json(v: &Version) -> serde_json::Value;
    }
}
```

**Example usage in CLI**:
```rust
use version_info::{Version, BuildInfo};
use version_info::display::cli::format_version;

let version = Version::new(...);
println!("{}", format_version(&version));
```

**Example usage in Web**:
```rust
use version_info::{Version, BuildInfo};
use version_info::display::html::format_footer;

let version = Version::new(...);
let footer_html = format_footer(&version);
```

### 3. `web-ui-components`

**Purpose**: Web UI components that parallel CLI functionality

**Exports**:
```rust
pub mod footer {
    pub struct Footer {
        version: Version,
        style: FooterStyle,
    }

    pub enum FooterStyle {
        Simple,      // Plain text
        Rich,        // HTML with links and styling
        Minimal,     // Just version number
    }

    impl Footer {
        pub fn to_html(&self) -> String;
        pub fn to_html_with_class(&self, class: &str) -> String;
    }
}

// Macro for easy footer creation
pub use web_ui_macros::web_footer;
```

**Example usage**:
```rust
use web_ui_components::prelude::*;

// In your web framework (Axum, Actix, etc.)
async fn index() -> Html<String> {
    let footer = web_footer!(style = "rich");

    Html(format!(r#"
        <html>
            <body>
                <h1>My App</h1>
                {footer}
            </body>
        </html>
    "#))
}
```

**Generated HTML**:
```html
<footer class="app-footer">
    <div class="version-info">
        <span class="version">v0.1.0</span>
        <span class="copyright">© 2025 Software Wrighter</span>
        <a href="https://github.com/user/repo/blob/main/LICENSE" class="license">MIT License</a>
        <span class="build-info" title="Build: abc123d @ hostname (2025-11-22T...)">
            Build abc123d
        </span>
    </div>
</footer>
```

### 4. `test-fixtures`

**Purpose**: Shared testing utilities and fixtures

**Exports**:
```rust
pub mod builders {
    // Test data builders
    pub struct ConfigBuilder { ... }
    pub struct VersionBuilder { ... }
}

pub mod fixtures {
    // Common test files
    pub fn sample_text_file() -> TempFile;
    pub fn sample_config() -> Config;
}

pub mod assertions {
    // Custom assertions for CLI testing
    pub fn assert_cli_output(output: &str, expected: &str);
    pub fn assert_version_format(output: &str);
}

pub struct TempFile {
    path: PathBuf,
    // Auto-cleanup on drop
}
```

**Example usage**:
```rust
#[cfg(test)]
mod tests {
    use test_fixtures::prelude::*;

    #[test]
    fn test_cli_version() {
        let output = run_cli(&["--version"]);
        assert_version_format(&output);
    }

    #[test]
    fn test_file_processing() {
        let temp = sample_text_file();
        let output = run_cli(&["--count", "-i", temp.path()]);
        assert_eq!(output.trim(), "42");
    }
}
```

### 5. `core-utilities`

**Purpose**: General utilities used by multiple crates

**Exports**:
```rust
pub mod config {
    // Configuration file handling (TOML, JSON, etc.)
    pub fn load_config<T>(path: &Path) -> Result<T>;
}

pub mod error {
    // Common error types
    pub type Result<T> = std::result::Result<T, Error>;
    pub enum Error { ... }
}

pub mod fs {
    // Filesystem utilities
    pub fn ensure_dir(path: &Path) -> Result<()>;
    pub fn atomic_write(path: &Path, content: &[u8]) -> Result<()>;
}
```

## Root Crate (Facade Pattern)

The root `src/lib.rs` re-exports everything for convenience:

```rust
// Root lib.rs - facade for all crates

// CLI framework (most common use case)
pub use cli_framework as cli;

// Version info (shared)
pub use version_info as version;

// Web UI components
#[cfg(feature = "web")]
pub use web_ui_components as web;

// Test utilities (dev-dependencies only)
#[cfg(test)]
pub use test_fixtures as test;

// Core utilities
pub use core_utilities as util;

// Convenient prelude
pub mod prelude {
    pub use cli_framework::prelude::*;
    pub use version_info::{Version, BuildInfo};

    #[cfg(feature = "web")]
    pub use web_ui_components::prelude::*;
}
```

## Workspace Cargo.toml

```toml
[workspace]
resolver = "2"
members = [
    "crates/cli-framework",
    "crates/cli-macros",
    "crates/version-info",
    "crates/web-ui-components",
    "crates/test-fixtures",
    "crates/core-utilities",
    "examples/*",
]

[workspace.package]
edition = "2021"
license = "MIT"
repository = "https://github.com/softwarewrighter/sw-cli"

[workspace.dependencies]
# Shared dependencies
clap = "4.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"

# Internal dependencies
cli-macros = { path = "crates/cli-macros" }
version-info = { path = "crates/version-info" }
core-utilities = { path = "crates/core-utilities" }

[package]
name = "sw-cli"
version = "0.2.0"
edition.workspace = true
license.workspace = true
repository.workspace = true

[dependencies]
cli-framework = { path = "crates/cli-framework" }
version-info = { path = "crates/version-info" }
web-ui-components = { path = "crates/web-ui-components", optional = true }
core-utilities = { path = "crates/core-utilities" }

[features]
default = ["cli"]
cli = ["cli-framework"]
web = ["web-ui-components"]
full = ["cli", "web"]

[dev-dependencies]
test-fixtures = { path = "crates/test-fixtures" }
```

## Usage Examples

### CLI Application
```toml
# User's Cargo.toml
[dependencies]
sw-cli = "0.2"  # Just CLI framework by default
```

```rust
// User's main.rs
use sw_cli::prelude::*;  // Gets CLI stuff

cli_app! {
    name: "my-tool",
    // ...
}
```

### Web Application with Footer
```toml
# User's Cargo.toml
[dependencies]
sw-cli = { version = "0.2", features = ["web"] }
axum = "0.7"
```

```rust
// User's web app
use sw_cli::web::prelude::*;

async fn index() -> Html<String> {
    let footer = web_footer!();
    // ...
}
```

### CLI + Web (Full Stack)
```toml
[dependencies]
sw-cli = { version = "0.2", features = ["full"] }
```

```rust
// Shared version info
use sw_cli::version::{Version, BuildInfo};

// CLI
use sw_cli::cli::prelude::*;

// Web
use sw_cli::web::prelude::*;
```

## Migration Path

### Phase 1: Restructure (v0.2.0)
- Move existing code to `crates/cli-framework`
- Create `version-info` crate from existing version module
- Update imports, preserve backward compatibility

### Phase 2: Web Components (v0.3.0)
- Add `web-ui-components` crate
- Add `web_footer!` macro
- Add HTML/JSON formatters

### Phase 3: Test Fixtures (v0.3.1)
- Add `test-fixtures` crate
- Extract common test patterns

### Phase 4: CLI Framework (v0.4.0)
- Add `cli_app!` and `cli_command!` macros
- Add dispatcher and command system
- Add help file integration

## Benefits

1. **Separation of Concerns**: Each crate has single responsibility
2. **Reusability**: Version info used by both CLI and web
3. **Optional Features**: Users only pull what they need
4. **Parallel Development**: Different contexts (CLI, web) developed independently
5. **Clear Dependencies**: Dependency graph is explicit
6. **Future Extensibility**: Easy to add new crates (desktop-ui, mobile, etc.)
7. **Better Testing**: test-fixtures crate supports all other crates
