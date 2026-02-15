# create-godotrs

A command-line tool for creating new Godot projects with Rust using the gdext library.

## Overview

`create-godotrs` is a project scaffolding tool that automatically sets up a new project combining Godot game engine with Rust bindings via the [gdext](https://github.com/godot-rust/gdext) library. It creates the necessary directory structure, configuration files, and boilerplate code to get you started quickly.

## Installation

### From Source

```bash
cargo install --path .
```

Or clone and build:

```bash
git clone <repository-url>
cd create-godotrs
cargo build --release
```

The binary will be available at `target/release/create-godotrs`.

## Usage

To create a new project:

```bash
create-godotrs <project-name>
```

To create with the proto template:

```bash
create-godotrs <project-name> --template proto
```

For example:

```bash
create-godotrs mygame
```

This will create a new directory called `mygame` with the following structure:

```
mygame/
├── .gitignore
├── godot/
│   ├── rust.gdextension
│   ├── .gitignore
│   └── project.godot
└── rust/
    ├── .gitignore
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

When `--template proto` is used, additional directories are created under `godot/`:

```
godot/
├── addons/
│   ├── AsepriteWizard/
│   └── ldtk-importer/
├── entity/
├── pipeline/
│   ├── aseprite/
│   └── ldtk/
├── player/
└── ui/
```

## Project Structure

### Godot Directory

The `godot/` directory contains your Godot project files:
- `project.godot`: The main Godot project configuration
- `rust.gdextension`: Configuration for the Rust GDExtension
- `.gitignore`: Git ignore rules specific to Godot

### Rust Directory

The `rust/` directory contains your Rust code:
- `Cargo.toml`: Rust package configuration with gdext dependency
- `src/lib.rs`: Entry point for your Rust extension with a basic template
- `.gitignore`: Git ignore rules specific to Rust

## Generated Code

### Rust Extension Template

The generated `rust/src/lib.rs` includes a minimal gdext setup:

```rust
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
```

### Cargo Configuration

The `Cargo.toml` is configured with:
- `cdylib` crate type for dynamic library compilation
- Latest gdext from GitHub
- Optimized dev profile settings for faster iteration

## Requirements

- Rust toolchain (edition 2024 or later)
- Godot 4.1 or later

## Development

### Running Tests

```bash
cargo test
```

The project includes comprehensive tests for all project creation functionality.

### Building

```bash
cargo build --release
```

## License

This project is distributed under the same license as your repository.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
