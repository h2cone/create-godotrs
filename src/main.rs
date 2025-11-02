use clap::Parser;
use create_godotrs::{ProjectConfig, create_project};
use std::process;

/// Create a new Godot project with Rust
#[derive(Parser, Debug)]
#[command(name = "create-godotrs")]
#[command(about = "Create a new Godot project with Rust", long_about = None)]
struct Args {
    /// Name of the project to create
    name: String,
}

fn main() {
    let args = Args::parse();

    let config = ProjectConfig::new(args.name);

    match create_project(&config) {
        Ok(()) => {
            println!("Successfully created project: {}", config.name);
            println!("Project location: {}", config.project_path().display());
        }
        Err(err) => {
            eprintln!("Error creating project: {}", err);
            process::exit(1);
        }
    }
}
