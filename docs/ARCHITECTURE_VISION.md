# Architecture Vision: Complete System Design

## Executive Summary

This document synthesizes the complete architecture vision for transforming sw-cli into a comprehensive toolkit that supports CLI applications, web UIs, and other contexts while maintaining extreme simplicity for users.

## Three Core Design Principles

### 1. **Conciseness Through Macros**
Users write minimal code; macros generate the boilerplate.

### 2. **Shared Core, Context-Specific Presentation**
Version/build info is defined once, displayed differently (CLI text, HTML footer, JSON API).

### 3. **Power Through Composition**
Traits + macros + chain of responsibility = flexible, extensible system.

## Complete Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         User's Application                       │
│  ┌────────────────────┐              ┌──────────────────────┐  │
│  │   CLI App          │              │   Web App            │  │
│  │   (main.rs)        │              │   (server.rs)        │  │
│  │                    │              │                      │  │
│  │  cli_app! {        │              │  web_footer!()       │  │
│  │    args: {...}     │              │                      │  │
│  │    commands: [...]  │              │  HTML output         │  │
│  │  }                 │              │                      │  │
│  └────────────────────┘              └──────────────────────┘  │
└─────────────┬────────────────────────────────┬─────────────────┘
              │                                │
              │                                │
    ┌─────────▼──────────┐          ┌─────────▼──────────┐
    │  cli-framework     │          │ web-ui-components  │
    │  ├── config/       │          │  ├── footer/       │
    │  ├── builder/      │          │  └── header/       │
    │  ├── dispatcher/   │          └────────────────────┘
    │  └── commands/     │                    │
    └────────────────────┘                    │
              │                                │
              │                                │
              └────────────┬───────────────────┘
                           │
                  ┌────────▼──────────┐
                  │   version-info    │
                  │   ├── version.rs  │
                  │   ├── build.rs    │
                  │   └── display/    │
                  │       ├── cli.rs  │
                  │       ├── html.rs │
                  │       └── json.rs │
                  └───────────────────┘
```

## How It All Works Together

### Scenario 1: CLI Application

**User writes** (50 lines):
```rust
use sw_cli::prelude::*;

cli_app! {
    name: "my-tool",
    help: {
        short: "src/short-help.txt",
        long: "src/long-help.txt",
    },
    args: {
        pattern: String => (short = 'p'),
        count: bool => (long = "count"),
    },
    commands: [CountCommand, GrepCommand]
}

cli_command! {
    name: CountCommand,
    can_handle: |c| c.count,
    execute: |c| { /* count logic */ }
}

cli_command! {
    name: GrepCommand,
    can_handle: |c| c.pattern.is_some(),
    execute: |c| { /* grep logic */ }
}
```

**Macro generates** (~200 lines):
1. Config struct with BaseConfig + custom fields
2. Clap command with all flags (standard + custom)
3. Config parser (ArgMatches → Config)
4. Help text inclusion from files
5. Main function with dispatcher
6. Command trait implementations

**Execution flow**:
```
User runs: my-tool -V
    ↓
main() parses args into Config
    ↓
Dispatcher tries each command in priority order:
    1. VersionCommand.can_handle(config) → true! (config.version = true)
    ↓
VersionCommand.execute(config)
    ↓
Calls version!() macro
    ↓
Loads version_info.rs from build
    ↓
Creates Version + BuildInfo objects
    ↓
Formats using display::cli::format_version()
    ↓
Prints 4-line output
```

### Scenario 2: Web Application

**User writes**:
```rust
use sw_cli::web::prelude::*;
use axum::{Router, routing::get};

async fn index() -> Html<String> {
    let footer = web_footer!(style = "rich");

    Html(format!(r#"
        <html>
            <body>
                <h1>My App</h1>
                <main>...</main>
                {footer}
            </body>
        </html>
    "#))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    // ... start server
}
```

**Build-time** (build.rs):
```rust
fn main() {
    sw_cli::define_build_info!();  // Same macro as CLI!
}
```

**Execution flow**:
```
User requests page
    ↓
web_footer! macro expands to:
    let version = version!();  // Reuses CLI version macro
    display::html::format_footer_rich(&version, "app-footer")
    ↓
Generates HTML with version info
    ↓
Returns styled footer component
```

**Output HTML**:
```html
<footer class="app-footer">
    <div class="version-info">
        <span class="version">v0.1.0</span>
        <span class="copyright">© 2025 Software Wrighter</span>
        <a href="https://github.com/.../LICENSE">MIT License</a>
        <span class="build-info" title="Build: abc123d @ hostname (2025-11-22T...)">
            Build abc123d
        </span>
    </div>
</footer>
```

### Scenario 3: CLI + Web (Same Codebase)

**Build once, use everywhere**:
```rust
// shared.rs - Version info defined once
pub fn app_version() -> Version {
    // Use the version!() macro that works everywhere
    version_info::parse_version_string(&sw_cli::version!())
}

// cli/main.rs
use sw_cli::cli::prelude::*;

cli_app! {
    // CLI using shared version
}

// web/server.rs
use sw_cli::web::prelude::*;

async fn footer() -> Html<String> {
    // Web using shared version
    Html(web_footer!())
}

// api/routes.rs
use sw_cli::version::display::json;

async fn version_endpoint() -> Json<Value> {
    // API using shared version
    Json(json::format_json(&app_version()))
}
```

## The Macro System in Detail

### 1. `define_build_info!()` - Build-Time Macro

**Purpose**: Generate version_info.rs at build time

**What it does**:
1. Read COPYRIGHT file
2. Extract Cargo.toml metadata (version, license, repo)
3. Execute shell commands (hostname, git SHA)
4. Generate timestamp
5. Write version_info.rs to $OUT_DIR

**Generated file**:
```rust
// $OUT_DIR/version_info.rs
pub const VERSION: &str = "0.1.0";
pub const COPYRIGHT: &str = "Copyright (c) 2025 ...";
pub const LICENSE_NAME: &str = "MIT";
pub const LICENSE_URL: &str = "https://...";
pub const BUILD_HOST: &str = "hostname";
pub const GIT_COMMIT_SHA: &str = "abc123def456...";
pub const BUILD_TIMESTAMP: i64 = 1732310000000;
```

### 2. `version!()` - Runtime Macro

**Purpose**: Create formatted version string

**What it does**:
1. Include version_info.rs from $OUT_DIR
2. Create Version and BuildInfo objects
3. Format using appropriate display formatter

**Expands to**:
```rust
{
    mod __version_info {
        include!(concat!(env!("OUT_DIR"), "/version_info.rs"));
    }

    let build_info = BuildInfo::new(
        __version_info::BUILD_HOST.to_string(),
        __version_info::GIT_COMMIT_SHA.to_string(),
        __version_info::BUILD_TIMESTAMP,
    );

    let version = Version::new(
        __version_info::VERSION.to_string(),
        __version_info::COPYRIGHT.to_string(),
        __version_info::LICENSE_NAME.to_string(),
        __version_info::LICENSE_URL.to_string(),
        build_info,
    );

    format!("{}", version)  // Uses Display trait
}
```

### 3. `cli_app!()` - CLI Definition Macro

**Purpose**: Generate complete CLI application

**Input**:
```rust
cli_app! {
    name: "my-tool",
    about: "Description",
    help: {
        short: "src/short-help.txt",
        long: "src/long-help.txt",
    },
    args: {
        pattern: String => (short = 'p'),
        count: bool => (long = "count"),
    },
    commands: [GrepCommand, CountCommand]
}
```

**Generates**:

1. **Config struct**:
```rust
pub struct MyToolConfig {
    pub base: BaseConfig,
    pub pattern: Option<String>,
    pub count: bool,
}
```

2. **Clap builder**:
```rust
fn build_cli() -> clap::Command {
    use clap::{Command, Arg};

    let cmd = Command::new("my-tool")
        .about("Description")
        .after_help(include_str!("src/short-help.txt"))
        .long_about(include_str!("src/long-help.txt"));

    // Add standard flags
    let cmd = sw_cli::builder::with_standard_flags(cmd);
    let cmd = sw_cli::builder::with_io_args(cmd);

    // Add custom args
    cmd.arg(Arg::new("pattern").short('p'))
       .arg(Arg::new("count").long("count"))
}
```

3. **Parser**:
```rust
fn parse_config(matches: ArgMatches) -> MyToolConfig {
    MyToolConfig {
        base: sw_cli::builder::parse_base_config(&matches),
        pattern: matches.get_one("pattern").cloned(),
        count: matches.get_flag("count"),
    }
}
```

4. **Main function**:
```rust
fn main() {
    let matches = build_cli().get_matches();
    let config = parse_config(matches);

    let dispatcher = sw_cli::Dispatcher::new()
        .register(sw_cli::VersionCommand::new())
        .register(sw_cli::HelpCommand::new(build_cli))
        .register(GrepCommand)
        .register(CountCommand);

    if let Err(e) = dispatcher.dispatch(&config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
```

### 4. `cli_command!()` - Command Definition Macro

**Purpose**: Simplify command implementation

**Input**:
```rust
cli_command! {
    name: GrepCommand,
    can_handle: |config: &MyToolConfig| config.pattern.is_some(),
    execute: |config: &MyToolConfig| {
        // Implementation
        Ok(())
    }
}
```

**Generates**:
```rust
pub struct GrepCommand;

impl sw_cli::Command for GrepCommand {
    fn can_handle(&self, config: &dyn CliConfig) -> bool {
        let config = config.as_any()
            .downcast_ref::<MyToolConfig>()
            .unwrap();
        config.pattern.is_some()
    }

    fn execute(&self, config: &dyn CliConfig) -> sw_cli::Result<()> {
        let config = config.as_any()
            .downcast_ref::<MyToolConfig>()
            .unwrap();
        // User's implementation
        Ok(())
    }
}
```

### 5. `web_footer!()` - Web Footer Macro

**Purpose**: Generate HTML footer with version info

**Input**:
```rust
web_footer!(style = "rich", class = "app-footer")
```

**Generates**:
```rust
{
    let version_string = sw_cli::version!();
    let version = sw_cli::version_info::parse_version_string(&version_string);
    sw_cli::web::footer::format_rich(&version, "app-footer")
}
```

## The Clap Integration

**Key insight**: The macros generate **pure clap code**. Users can:

1. **Use pure macro** (simplest)
2. **Mix macro + manual clap** (flexible)
3. **Use manual clap + helper functions** (full control)

### Example: Mixed Approach
```rust
cli_app! {
    name: "my-tool",

    // Customize the clap builder
    builder: |cmd| {
        cmd.about("Custom text")
           .long_about(include_str!("long-help.txt"))
           // Add complex arg the macro doesn't support
           .arg(Arg::new("config")
                .conflicts_with("pattern")
                .value_parser(value_parser!(PathBuf)))
    },

    args: {
        pattern: String => (short = 'p'),  // Macro handles simple args
    },

    commands: [...]
}
```

## Help Text System

**Two-level help**:
- `-h, --help` → Brief (clap defaults + short-help.txt)
- `--help-long` → Detailed (clap defaults + long-help.txt)

**Flow**:
```
User runs: my-tool -h
    ↓
Clap parses: config.base.help = true
    ↓
Dispatcher: HelpCommand.can_handle() → true
    ↓
HelpCommand.execute():
    - Calls build_cli().render_help()
    - Appends content of short-help.txt
    - Prints combined help
```

## Project Structure

```
sw-cli/  (or sw-toolkit)
├── crates/
│   ├── cli-framework/      ← CLI-specific (config, builder, dispatcher, commands)
│   ├── cli-macros/         ← Procedural macros (cli_app!, cli_command!, etc.)
│   ├── version-info/       ← Shared (Version, BuildInfo, formatters)
│   ├── web-ui-components/  ← Web-specific (footer, header components)
│   ├── test-fixtures/      ← Testing utilities
│   └── core-utilities/     ← General utilities
├── examples/
├── docs/
└── src/lib.rs             ← Facade that re-exports everything
```

## Summary: What Makes This Powerful

1. **One Build, Many Outputs**: Version info defined once, used in CLI, web, API
2. **Minimal User Code**: Macros generate hundreds of lines from tens
3. **Type-Safe**: Strong typing throughout, no string-based lookups
4. **Clap Native**: Full power of clap available
5. **Extensible**: Add commands, add crates, add contexts
6. **Testable**: Each component isolated and testable
7. **Professional**: Standard flags, two-level help, proper error handling
8. **Flexible**: From simple (macro-only) to complex (manual control)

## Next Steps

1. **Phase 1**: Restructure into crates/ (preserve backward compat)
2. **Phase 2**: Implement version-info display formatters
3. **Phase 3**: Implement cli-framework (config, dispatcher)
4. **Phase 4**: Implement cli_app! and cli_command! macros
5. **Phase 5**: Implement web-ui-components
6. **Phase 6**: Add examples demonstrating all patterns
