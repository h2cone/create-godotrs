use clap::Parser;
use clap::ValueEnum;
use create_godotrs::{ProjectConfig, ProjectTemplate, create_project};
use std::process;

/// Create a new Godot project with Rust
#[derive(Parser, Debug)]
#[command(name = "create-godotrs")]
#[command(about = "Create a new Godot project with Rust", long_about = None)]
struct Args {
    /// Name of the project to create
    name: String,

    /// Scaffold template
    #[arg(long, value_enum, default_value_t = TemplateArg::Basic)]
    template: TemplateArg,
}

#[derive(Clone, Debug, ValueEnum)]
enum TemplateArg {
    Basic,
    Proto,
}

impl From<TemplateArg> for ProjectTemplate {
    fn from(value: TemplateArg) -> Self {
        match value {
            TemplateArg::Basic => ProjectTemplate::Basic,
            TemplateArg::Proto => ProjectTemplate::Proto,
        }
    }
}

fn main() {
    let args = Args::parse();

    let config = ProjectConfig::new(args.name).with_template(args.template.into());

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
