# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`create-godotrs` is a CLI scaffolding tool that generates Godot + Rust projects using the gdext library. It creates a dual-module structure with a Godot project and a Rust cdylib that integrates via GDExtension.

## Essential Commands

### Development
```bash
# Run tests (TDD approach is used in this project)
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run a single test
cargo test test_create_project_success

# Build release binary
cargo build --release

# Test the CLI tool
cargo run -- <project-name>

# Install locally
cargo install --path .

# Format code
cargo fmt

# Lint code
cargo clippy --all-targets --all-features
```

### Manual Testing
```bash
# Create a test project
cargo run -- testproject

# Clean up test projects
rm -rf testproject
```

## Architecture

### Code Organization

The project follows a library + binary pattern:

- **`src/lib.rs`**: Contains all core logic for project creation (src/lib.rs:1)
  - Public API: `create_project()`, `ProjectConfig`
  - Template embedding via `include_str!()` macros (src/lib.rs:6-11)
  - Four-stage project creation pipeline: directory structure → templates → Godot init → Rust init

- **`src/main.rs`**: Thin CLI wrapper using clap (src/main.rs:1)
  - Single positional argument for project name
  - Error handling with exit codes

### Template System

Templates are embedded at compile-time from `templates/` directory:
- `Project.gitignore`: Root-level gitignore
- `Godot.gitignore`: Godot-specific ignores
- `Rust.gitignore`: Rust-specific ignores
- `rust.gdextension`: GDExtension configuration with platform-specific library paths

Templates are accessed via the `templates` module (src/lib.rs:6) and written directly to disk without modification, except for:
- `project.godot`: Project name is interpolated as `{name}-godot`

### Project Creation Flow

The `create_project()` function (src/lib.rs:65) orchestrates four steps:

1. **Existence Check**: Fails fast if target directory exists (src/lib.rs:69)
2. **Directory Structure** (src/lib.rs:89): Creates `godot/` and `rust/src/` subdirectories
3. **Template Generation** (src/lib.rs:104): Copies embedded templates to appropriate locations
4. **Initialization**:
   - Godot (src/lib.rs:133): Generates `project.godot` with interpolated name
   - Rust (src/lib.rs:148): Generates `Cargo.toml` and `lib.rs` with gdext boilerplate

### Testing Strategy

Tests use `tempfile::TempDir` for isolated filesystem operations (src/lib.rs:184-254):
- `test_create_project_success`: Verifies complete project structure and file contents
- `test_create_project_already_exists`: Tests error handling for existing directories
- `test_project_config_project_path`: Unit test for path construction

Tests verify both file existence and content correctness (checking for specific strings in generated files).

### Configuration Model

`ProjectConfig` (src/lib.rs:40) separates concerns:
- `name`: User-provided project name
- `base_path`: Where to create the project (defaults to current directory)
- `project_path()`: Computed property joining base_path and name

This design enables testing with temporary directories while keeping the CLI simple.

## Key Design Decisions

1. **No External Commands**: Uses Rust stdlib for all file operations instead of shelling out to `cargo` or `mkdir`
2. **Embedded Templates**: Templates are compiled into the binary, eliminating runtime dependencies
3. **Library-First**: Core logic in `lib.rs` enables testing without invoking the CLI
4. **Rust Edition 2024**: Uses latest edition for all generated and tool code
5. **Static Rust Configuration**: Generated Rust projects always use `cdylib` crate type and gdext from git

## Requirements Specification

See `SPEC.md` for the original Chinese requirements. Key constraints:
- Project name must be parameterized throughout generated files
- Generated Rust lib must use edition 2024
- No interactive CLI features
- Prefer standard library over external commands
