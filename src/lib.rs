use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Template files embedded in the binary
pub mod templates {
    pub const PROJECT_GITIGNORE: &str = include_str!("../templates/Project.gitignore");
    pub const GODOT_GITIGNORE: &str = include_str!("../templates/Godot.gitignore");
    pub const RUST_GITIGNORE: &str = include_str!("../templates/Rust.gitignore");
    pub const GODOT_GDEXTENSION: &str = include_str!("../templates/Godot.gdextension");
}

/// Errors that can occur during project creation
#[derive(Debug)]
pub enum CreateError {
    Io(io::Error),
    ProjectAlreadyExists(PathBuf),
}

impl From<io::Error> for CreateError {
    fn from(err: io::Error) -> Self {
        CreateError::Io(err)
    }
}

impl std::fmt::Display for CreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateError::Io(err) => write!(f, "IO error: {}", err),
            CreateError::ProjectAlreadyExists(path) => {
                write!(f, "Project directory already exists: {}", path.display())
            }
        }
    }
}

impl std::error::Error for CreateError {}

/// Configuration for creating a new project
#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub base_path: PathBuf,
}

impl ProjectConfig {
    pub fn new(name: String) -> Self {
        Self {
            name,
            base_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    pub fn with_base_path(mut self, path: PathBuf) -> Self {
        self.base_path = path;
        self
    }

    pub fn project_path(&self) -> PathBuf {
        self.base_path.join(&self.name)
    }
}

/// Create a new Godot project with Rust
pub fn create_project(config: &ProjectConfig) -> Result<(), CreateError> {
    let project_path = config.project_path();

    // Check if project already exists
    if project_path.exists() {
        return Err(CreateError::ProjectAlreadyExists(project_path));
    }

    // Create project directory structure
    create_directory_structure(&project_path)?;

    // Generate template files
    generate_template_files(&project_path, &config.name)?;

    // Initialize Godot project
    initialize_godot_project(&project_path, &config.name)?;

    // Initialize Rust project
    initialize_rust_project(&project_path)?;

    Ok(())
}

/// Create the directory structure for the project
fn create_directory_structure(project_path: &Path) -> Result<(), CreateError> {
    // Create main project directory
    fs::create_dir_all(project_path)?;

    // Create godot subdirectory
    fs::create_dir(project_path.join("godot"))?;

    // Create rust subdirectory and src
    fs::create_dir(project_path.join("rust"))?;
    fs::create_dir(project_path.join("rust/src"))?;

    Ok(())
}

/// Generate template files from embedded templates
fn generate_template_files(project_path: &Path, _project_name: &str) -> Result<(), CreateError> {
    // Write root .gitignore
    fs::write(
        project_path.join(".gitignore"),
        templates::PROJECT_GITIGNORE,
    )?;

    // Write godot .gitignore
    fs::write(
        project_path.join("godot/.gitignore"),
        templates::GODOT_GITIGNORE,
    )?;

    // Write godot .gdextension
    fs::write(
        project_path.join("godot/.gdextension"),
        templates::GODOT_GDEXTENSION,
    )?;

    // Write rust .gitignore
    fs::write(
        project_path.join("rust/.gitignore"),
        templates::RUST_GITIGNORE,
    )?;

    Ok(())
}

/// Initialize the Godot project
fn initialize_godot_project(project_path: &Path, project_name: &str) -> Result<(), CreateError> {
    let godot_project_content = format!("[application]\nconfig/name=\"{}-godot\"\n", project_name);

    fs::write(
        project_path.join("godot/project.godot"),
        godot_project_content,
    )?;

    Ok(())
}

/// Initialize the Rust project
fn initialize_rust_project(project_path: &Path) -> Result<(), CreateError> {
    // Write lib.rs
    let lib_rs_content = r#"use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
"#;

    fs::write(project_path.join("rust/src/lib.rs"), lib_rs_content)?;

    // Write Cargo.toml
    let cargo_toml_content = r#"[package]
name = "rust"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
godot = { git = "https://github.com/godot-rust/gdext" }

[profile.dev]
opt-level = 1
[profile.dev.package."*"]
opt-level = 1
"#;

    fs::write(project_path.join("rust/Cargo.toml"), cargo_toml_content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_project_success() {
        let temp_dir = TempDir::new().unwrap();
        let config = ProjectConfig::new("testproject".to_string())
            .with_base_path(temp_dir.path().to_path_buf());

        let result = create_project(&config);
        assert!(result.is_ok());

        // Verify directory structure
        let project_path = config.project_path();
        assert!(project_path.exists());
        assert!(project_path.join("godot").exists());
        assert!(project_path.join("rust").exists());
        assert!(project_path.join("rust/src").exists());

        // Verify template files
        assert!(project_path.join(".gitignore").exists());
        assert!(project_path.join("godot/.gitignore").exists());
        assert!(project_path.join("godot/.gdextension").exists());
        assert!(project_path.join("godot/project.godot").exists());
        assert!(project_path.join("rust/.gitignore").exists());
        assert!(project_path.join("rust/Cargo.toml").exists());
        assert!(project_path.join("rust/src/lib.rs").exists());

        // Verify content of project.godot
        let godot_content = fs::read_to_string(project_path.join("godot/project.godot")).unwrap();
        assert!(godot_content.contains("config/name=\"testproject-godot\""));

        // Verify content of lib.rs
        let lib_rs_content = fs::read_to_string(project_path.join("rust/src/lib.rs")).unwrap();
        assert!(lib_rs_content.contains("use godot::prelude::*"));
        assert!(lib_rs_content.contains("struct MyExtension"));
        assert!(lib_rs_content.contains("#[gdextension]"));

        // Verify content of Cargo.toml
        let cargo_content = fs::read_to_string(project_path.join("rust/Cargo.toml")).unwrap();
        assert!(cargo_content.contains("crate-type = [\"cdylib\"]"));
        assert!(
            cargo_content.contains("godot = { git = \"https://github.com/godot-rust/gdext\" }")
        );
    }

    #[test]
    fn test_create_project_already_exists() {
        let temp_dir = TempDir::new().unwrap();
        let config = ProjectConfig::new("testproject".to_string())
            .with_base_path(temp_dir.path().to_path_buf());

        // Create project first time
        create_project(&config).unwrap();

        // Try to create again
        let result = create_project(&config);
        assert!(result.is_err());
        match result {
            Err(CreateError::ProjectAlreadyExists(_)) => {}
            _ => panic!("Expected ProjectAlreadyExists error"),
        }
    }

    #[test]
    fn test_project_config_project_path() {
        let config =
            ProjectConfig::new("myproject".to_string()).with_base_path(PathBuf::from("/test/path"));

        assert_eq!(config.project_path(), PathBuf::from("/test/path/myproject"));
    }
}
