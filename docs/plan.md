# SW-CLI Modular Refactoring Plan

## Executive Summary

This document outlines a refactoring plan to transform `sw-cli` from a monolithic library into a modular, extensible workspace that can support multiple CLI utility functions beyond version information.

## Current Structure Analysis

### Workspace Layout
```
sw-cli/
├── src/
│   ├── lib.rs              # Re-exports macros and version module
│   └── version.rs          # BuildInfo and Version structs
├── sw-cli-macros/          # Procedural macro crate
│   └── src/lib.rs          # version!(), define_build_info!(), create_version!()
├── examples/
│   └── demo-cli/           # Example demonstrating version functionality
└── docs/
    └── version-usage.md    # User documentation
```

### Current Issues
1. **Tight Coupling**: Build and version functionality are mixed in a single `version.rs` module
2. **Limited Modularity**: Difficult to add new CLI utilities (argument parsing, config management, etc.)
3. **Naming Inconsistency**: `sw-cli-macros` vs workspace naming patterns
4. **Example Specificity**: `demo-cli` doesn't indicate it's version-specific
5. **Future Constraints**: No clear path to add non-version related subcrates

## Proposed Modular Architecture

### New Workspace Structure
```
sw-cli/
├── crates/
│   ├── cli-utilities/      # Core utilities subcrate
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── version/    # Version module
│   │   │   │   └── mod.rs  # Version struct and display
│   │   │   └── build/      # Build module
│   │   │       └── mod.rs  # BuildInfo struct and metadata
│   │   └── Cargo.toml
│   │
│   └── cli-macros/         # Procedural macro crate (renamed)
│       ├── src/
│       │   ├── lib.rs      # Macro implementations
│       │   ├── version.rs  # version!() macro
│       │   └── build.rs    # define_build_info!() macro
│       └── Cargo.toml
│
├── examples/
│   └── cli-version-demo/   # Renamed, version-specific example
│       ├── src/main.rs
│       ├── build.rs
│       ├── COPYRIGHT
│       └── Cargo.toml
│
├── src/
│   └── lib.rs              # Root crate re-exports from cli-utilities
├── docs/
│   ├── version-usage.md    # Existing version documentation
│   └── plan.md             # This document
├── Cargo.toml              # Workspace definition
└── build.rs                # Root build script
```

### Future Extensibility
This structure allows for easy addition of new functionality:
```
crates/
├── cli-utilities/          # Existing
├── cli-macros/             # Existing
├── cli-config/             # Future: Configuration management
├── cli-args/               # Future: Advanced argument parsing
├── cli-logging/            # Future: Structured logging utilities
└── cli-term/               # Future: Terminal UI helpers
```

## Detailed Refactoring Steps

### Phase 1: Create New Crate Structure

#### 1.1 Create `crates/cli-utilities` Subcrate
- Initialize new crate: `crates/cli-utilities/`
- Separate concerns:
  - **build module** (`crates/cli-utilities/src/build/mod.rs`)
    - `BuildInfo` struct
    - Build metadata collection
    - Display formatting for build info
    - Tests for build info formatting

  - **version module** (`crates/cli-utilities/src/version/mod.rs`)
    - `Version` struct
    - Version composition (combines build info)
    - Display formatting for full version output
    - Tests for version display

- Dependencies:
  - `chrono = "0.4"` (for timestamp handling)

#### 1.2 Rename and Restructure Macros
- Rename `sw-cli-macros/` to `crates/cli-macros/`
- Split macro implementations:
  - `crates/cli-macros/src/version.rs` - `version!()` macro
  - `crates/cli-macros/src/build.rs` - `define_build_info!()` macro
  - `crates/cli-macros/src/lib.rs` - Public re-exports

- Update macro paths to reference `cli_utilities` crate:
  ```rust
  // In version!() macro
  ::cli_utilities::version::Version::new(...)
  ::cli_utilities::build::BuildInfo::new(...)
  ```

#### 1.3 Update Root Crate
- Modify `sw-cli/src/lib.rs` to re-export from new crates:
  ```rust
  // Re-export utilities for convenience
  pub use cli_utilities::{build, version};

  // Re-export macros
  pub use cli_macros::{define_build_info, version};

  // Deprecated but maintained for compatibility
  #[deprecated(since = "0.2.0", note = "Use version!() instead")]
  pub use cli_macros::create_version;
  ```

### Phase 2: Rename Example

#### 2.1 Rename Example Crate
- Move `examples/demo-cli/` to `examples/cli-version-demo/`
- Update `Cargo.toml`:
  ```toml
  [package]
  name = "cli-version-demo"
  version = "0.1.0"
  ```

#### 2.2 Update Example Documentation
- Update comments in `examples/cli-version-demo/src/main.rs`
- Add clear documentation about what the example demonstrates
- Reference version-specific functionality

### Phase 3: Update Workspace Configuration

#### 3.1 Update Root `Cargo.toml`
```toml
[workspace]
members = [
    "crates/cli-utilities",
    "crates/cli-macros",
    "examples/cli-version-demo"
]
resolver = "2"

[package]
name = "sw-cli"
version = "0.2.0"  # Bump for breaking changes
edition = "2024"

[dependencies]
cli-utilities = { path = "crates/cli-utilities" }
cli-macros = { path = "crates/cli-macros" }
```

#### 3.2 Update `cli-utilities/Cargo.toml`
```toml
[package]
name = "cli-utilities"
version = "0.2.0"
edition = "2024"

[dependencies]
chrono = "0.4"
```

#### 3.3 Update `cli-macros/Cargo.toml`
```toml
[package]
name = "cli-macros"
version = "0.2.0"
edition = "2024"

[lib]
proc-macro = true

[dependencies]
quote = "1.0"
syn = { version = "2.0", features = ["full"] }
proc-macro2 = "1.0"
```

### Phase 4: Update Build Scripts and Documentation

#### 4.1 Update Build Scripts
- Modify `scripts/build.sh` to handle new crate names
- Update `scripts/test.sh` to test all new subcrates
- Update `scripts/demo.sh` to reference `cli-version-demo`

#### 4.2 Update Documentation
- Update `docs/version-usage.md` with new import paths:
  ```rust
  // Old (still works via re-export)
  use sw_cli::version::Version;

  // New (direct access)
  use cli_utilities::version::Version;
  use cli_utilities::build::BuildInfo;
  ```

- Update `CLAUDE.md` to reflect new architecture:
  - Document the modular structure
  - Explain the separation of concerns
  - Update workspace member list
  - Document future extensibility plans

### Phase 5: Migration and Testing

#### 5.1 Code Migration Checklist
- [ ] Create `crates/cli-utilities/src/build/mod.rs` with `BuildInfo`
- [ ] Create `crates/cli-utilities/src/version/mod.rs` with `Version`
- [ ] Move tests from `src/version.rs` to appropriate modules
- [ ] Create `crates/cli-macros/` from `sw-cli-macros/`
- [ ] Update macro implementations to use new paths
- [ ] Update root `lib.rs` with re-exports
- [ ] Rename example directory and update references
- [ ] Update all `Cargo.toml` files

#### 5.2 Testing Strategy
1. **Unit Tests**: Ensure all existing tests pass in new locations
2. **Integration Tests**: Verify `version!()` macro works with new structure
3. **Example Test**: Run `cli-version-demo` and verify output format
4. **Build Test**: Ensure `scripts/build.sh` builds all crates
5. **Compatibility Test**: Verify old import paths still work via re-exports

## Breaking Changes and Migration Guide

### Breaking Changes
1. Internal module structure reorganized (users relying on direct paths affected)
2. Crate names changed (`sw-cli-macros` → `cli-macros`)
3. Version bumped to 0.2.0

### Migration for Users
Most users won't need to change anything due to re-exports in root crate:

```rust
// This still works (recommended for users)
use sw_cli::version!;

// This is new (for advanced users)
use cli_utilities::version::Version;
use cli_utilities::build::BuildInfo;
```

### Deprecation Timeline
- `create_version!()` macro: Deprecated in 0.2.0, remove in 0.3.0
- Old module paths: Supported via re-exports indefinitely

## Benefits of Refactoring

### Immediate Benefits
1. **Separation of Concerns**: Build and version logic are properly separated
2. **Better Organization**: Clear module hierarchy matches domain concepts
3. **Naming Consistency**: All crates follow `cli-*` naming pattern
4. **Example Clarity**: `cli-version-demo` clearly indicates purpose

### Long-term Benefits
1. **Extensibility**: Easy to add new CLI utility crates
2. **Maintainability**: Each module has clear, single responsibility
3. **Testability**: Isolated modules are easier to test
4. **Reusability**: Individual utilities can be used independently
5. **Documentation**: Clearer structure improves documentation

## Future Roadmap

### Version 0.2.0 (This Refactoring)
- Modular crate structure
- Separated build and version modules
- Renamed example

### Version 0.3.0 (Future)
- Add `cli-config` crate for configuration file handling
- Add `cli-args` crate for advanced argument parsing
- Remove deprecated `create_version!()` macro

### Version 0.4.0 (Future)
- Add `cli-logging` crate for structured logging
- Add `cli-term` crate for terminal UI helpers
- Comprehensive integration examples

## Implementation Timeline

### Recommended Approach
Execute phases sequentially to minimize risk:

1. **Phase 1**: Create new crate structure (no deletions yet)
2. **Phase 2**: Rename example
3. **Phase 3**: Update workspace configuration
4. **Phase 4**: Update documentation
5. **Phase 5**: Test thoroughly, then remove old code

### Rollback Plan
- Keep git tags at each phase
- Maintain feature branch until all tests pass
- Document any issues discovered during migration

## Conclusion

This refactoring transforms `sw-cli` from a single-purpose version library into a modular workspace ready for expansion. The new structure maintains backward compatibility while providing a clear path for adding new CLI utilities. The separation of build and version concerns improves code organization, and the consistent naming scheme (`cli-*`) establishes a pattern for future additions.
