# Repository Guidelines

This document guides contributors working on `create-godotrs`, a Rust CLI that scaffolds a Godot + Rust project using gdext.

## Project Structure & Module Organization
- Root: `Cargo.toml`, `README.md`, `SPEC.md`, `AGENTS.md`.
- Source: `src/` — CLI entry in `src/main.rs`, core logic and tests in `src/lib.rs`.
- Templates: `templates/` — embedded files written into generated projects (`rust.gdextension`, `Godot.gitignore`, `Rust.gitignore`, `Project.gitignore`).
- Build artifacts: `target/` (ignored in VCS).

## Build, Test, and Development Commands
- `cargo build` — compile in debug mode.
- `cargo build --release` — optimized binary in `target/release/create-godotrs`.
- `cargo run -- <project-name>` — run the generator locally (e.g., `cargo run -- mygame`).
- `cargo test` — run unit tests in `src/lib.rs`.
- `cargo install --path .` — install the CLI locally as `create-godotrs`.

## Coding Style & Naming Conventions
- Language: Rust (edition 2024). Use 4‑space indentation, `snake_case` for modules/functions, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Formatting: `rustfmt` (use `cargo fmt`). Linting: `clippy` (use `cargo clippy --all-targets --all-features -D warnings`).
- Error handling: prefer typed errors (`CreateError`) and `Result` returns over panics in library code.

## Testing Guidelines
- Framework: Rust built‑in test harness (`#[test]`).
- Location: co‑located unit tests in `src/lib.rs` under `#[cfg(test)]`.
- Style: name tests `test_<behavior>()` and assert filesystem output (dirs/files and key contents).
- Aim for coverage of: directory creation, template writing, idempotency/error (`ProjectAlreadyExists`).

## Commit & Pull Request Guidelines
- Commits: use Conventional Commits (e.g., `feat: add rust Cargo.toml generator`, `fix: correct rust.gdextension paths`).
- PRs: include a concise description, linked issues, before/after notes, and CLI examples (`cargo run -- mygame`). Add screenshots only when helpful.
- Tests: required for behavior changes to scaffolding, templates, or errors.

## Agent‑Specific Notes
- Keep templates in `templates/` the source of truth; update tests when changing them.
- Avoid invoking external tools for scaffolding; use Rust stdlib for IO and paths.
- Never overwrite an existing project directory; return `ProjectAlreadyExists`.
