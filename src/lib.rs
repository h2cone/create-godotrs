use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Template files embedded in the binary
pub mod templates {
    pub const PROJECT_GITIGNORE: &str = include_str!("../templates/Project.gitignore");
    pub const GODOT_GITIGNORE: &str = include_str!("../templates/Godot.gitignore");
    pub const RUST_GITIGNORE: &str = include_str!("../templates/Rust.gitignore");
    pub const RUST_GDEXTENSION: &str = include_str!("../templates/rust.gdextension");
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectTemplate {
    Basic,
    Proto,
}

/// Configuration for creating a new project
#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub base_path: PathBuf,
    pub template: ProjectTemplate,
}

impl ProjectConfig {
    pub fn new(name: String) -> Self {
        Self {
            name,
            base_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            template: ProjectTemplate::Basic,
        }
    }

    pub fn with_base_path(mut self, path: PathBuf) -> Self {
        self.base_path = path;
        self
    }

    pub fn with_template(mut self, template: ProjectTemplate) -> Self {
        self.template = template;
        self
    }

    pub fn project_path(&self) -> PathBuf {
        self.base_path.join(&self.name)
    }
}

/// Create a new Godot project with Rust
pub fn create_project(config: &ProjectConfig) -> Result<(), CreateError> {
    let project_path = config.project_path();

    if project_path.exists() {
        return Err(CreateError::ProjectAlreadyExists(project_path));
    }

    create_directory_structure(&project_path)?;

    let result = (|| {
        generate_template_files(&project_path)?;
        initialize_godot_project(&project_path, &config.name)?;
        if config.template == ProjectTemplate::Proto {
            initialize_proto_template(&project_path)?;
        }
        initialize_rust_project(&project_path)?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_dir_all(&project_path);
    }

    result
}

fn create_directory_structure(project_path: &Path) -> Result<(), CreateError> {
    fs::create_dir_all(project_path)?;
    fs::create_dir(project_path.join("godot"))?;
    fs::create_dir(project_path.join("rust"))?;
    fs::create_dir(project_path.join("rust/src"))?;
    Ok(())
}

fn generate_template_files(project_path: &Path) -> Result<(), CreateError> {
    fs::write(
        project_path.join(".gitignore"),
        templates::PROJECT_GITIGNORE,
    )?;
    fs::write(
        project_path.join("godot/.gitignore"),
        templates::GODOT_GITIGNORE,
    )?;
    fs::write(
        project_path.join("godot/rust.gdextension"),
        templates::RUST_GDEXTENSION,
    )?;
    fs::write(
        project_path.join("rust/.gitignore"),
        templates::RUST_GITIGNORE,
    )?;
    Ok(())
}

fn initialize_godot_project(project_path: &Path, project_name: &str) -> Result<(), CreateError> {
    let godot_project_content = format!("[application]\nconfig/name=\"{}-godot\"\n", project_name);
    fs::write(
        project_path.join("godot/project.godot"),
        godot_project_content,
    )?;
    Ok(())
}

/// Initialize the proto template structure with directory skeleton only
fn initialize_proto_template(project_path: &Path) -> Result<(), CreateError> {
    let godot_path = project_path.join("godot");
    let directories = [
        godot_path.join("entity"),
        godot_path.join("player"),
        godot_path.join("ui"),
        godot_path.join("pipeline/aseprite/scripts"),
        godot_path.join("pipeline/aseprite/src"),
        godot_path.join("pipeline/aseprite/wizard"),
        godot_path.join("pipeline/ldtk"),
        godot_path.join("addons/AsepriteWizard"),
        godot_path.join("addons/ldtk-importer"),
    ];

    for dir in directories {
        fs::create_dir_all(dir)?;
    }

    Ok(())
}

fn initialize_rust_project(project_path: &Path) -> Result<(), CreateError> {
    let lib_rs_content = r#"use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
"#;

    fs::write(project_path.join("rust/src/lib.rs"), lib_rs_content)?;

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

        let project_path = config.project_path();
        assert!(project_path.exists());
        assert!(project_path.join("godot").exists());
        assert!(project_path.join("rust").exists());
        assert!(project_path.join("rust/src").exists());

        assert!(project_path.join(".gitignore").exists());
        assert!(project_path.join("godot/.gitignore").exists());
        assert!(project_path.join("godot/rust.gdextension").exists());
        assert!(project_path.join("godot/project.godot").exists());
        assert!(project_path.join("rust/.gitignore").exists());
        assert!(project_path.join("rust/Cargo.toml").exists());
        assert!(project_path.join("rust/src/lib.rs").exists());

        let godot_content = fs::read_to_string(project_path.join("godot/project.godot")).unwrap();
        assert!(godot_content.contains("config/name=\"testproject-godot\""));
    }

    #[test]
    fn test_create_project_already_exists() {
        let temp_dir = TempDir::new().unwrap();
        let config = ProjectConfig::new("testproject".to_string())
            .with_base_path(temp_dir.path().to_path_buf());

        create_project(&config).unwrap();

        let result = create_project(&config);
        assert!(result.is_err());
        match result {
            Err(CreateError::ProjectAlreadyExists(_)) => {}
            _ => panic!("Expected ProjectAlreadyExists error"),
        }
    }

    #[test]
    fn test_create_project_with_proto_template() {
        let temp_dir = TempDir::new().unwrap();

        let config = ProjectConfig::new("templated-project".to_string())
            .with_base_path(temp_dir.path().to_path_buf())
            .with_template(ProjectTemplate::Proto);

        create_project(&config).unwrap();

        let project_path = config.project_path();
        assert!(project_path.join("godot/entity").exists());
        assert!(project_path.join("godot/player").exists());
        assert!(project_path.join("godot/ui").exists());
        assert!(project_path.join("godot/pipeline/aseprite").exists());
        assert!(
            project_path
                .join("godot/pipeline/aseprite/scripts")
                .exists()
        );
        assert!(project_path.join("godot/pipeline/aseprite/src").exists());
        assert!(project_path.join("godot/pipeline/aseprite/wizard").exists());
        assert!(project_path.join("godot/pipeline/ldtk").exists());
        assert!(project_path.join("godot/addons/AsepriteWizard").exists());
        assert!(project_path.join("godot/addons/ldtk-importer").exists());
    }

    #[test]
    fn test_project_config_project_path() {
        let config =
            ProjectConfig::new("myproject".to_string()).with_base_path(PathBuf::from("/test/path"));

        assert_eq!(config.project_path(), PathBuf::from("/test/path/myproject"));
        assert_eq!(config.template, ProjectTemplate::Basic);
    }
}
