# Repository Guidelines

## Project Structure & Module Organization
This repository is a small Rust CLI that scaffolds Godot + Rust projects.

- `src/main.rs`: CLI entrypoint using `clap` (`create-godotrs <name> [--template proto]`).
- `src/lib.rs`: core project-generation logic, error types, template selection, and unit tests.
- `templates/`: embedded scaffold files loaded with `include_str!` (for `.gitignore` and `rust.gdextension`).
- `target/`: local build artifacts; do not commit.

Keep generation behavior centralized in `src/lib.rs` and keep `src/main.rs` focused on argument parsing and user output.

## Build, Test, and Development Commands
- `cargo build`: compile debug build.
- `cargo build --release`: build optimized binary in `target/release/create-godotrs`.
- `cargo run -- mygame`: run CLI locally and scaffold a basic project.
- `cargo run -- mygame --template proto`: scaffold the proto directory layout.
- `cargo test`: run unit tests (filesystem behavior and template layout checks).
- `cargo fmt`: format code with rustfmt.
- `cargo clippy --all-targets --all-features -D warnings`: lint and treat warnings as errors.

## Coding Style & Naming Conventions
- Rust edition: `2024` (see `Cargo.toml`).
- Use rustfmt defaults (4-space indentation, trailing commas where appropriate).
- Naming: `snake_case` for functions/modules, `CamelCase` for structs/enums, `UPPER_SNAKE_CASE` for constants.
- Prefer small, focused functions for file/directory creation steps; propagate errors with `Result`.

## Testing Guidelines
- Tests live in `src/lib.rs` under `#[cfg(test)]`.
- Test names follow `test_<expected_behavior>` (for example, `test_create_project_with_proto_template`).
- Use `tempfile::TempDir` for isolated filesystem tests.
- Cover both success and failure flows (existing path, template-specific directories, generated file contents).

## Commit & Pull Request Guidelines
- Follow the lightweight conventional style visible in history, e.g. `refactor: rename gdextension template to rust.gdextension`.
- Recommended format: `<type>: <short imperative summary>` (`feat`, `fix`, `refactor`, `test`, `docs`).
- PRs should include:
  - clear summary of behavior changes,
  - linked issue (if applicable),
  - local verification steps/results (at minimum `cargo test`).
