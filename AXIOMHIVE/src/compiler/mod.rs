// @AXIOMHIVE @DEVDOLLZAI
// The Weaver Core: Generates and compiles a new Rust project from templates.

use minijinja::{context, Environment};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Context, Result};

pub struct Compiler;

impl Compiler {
    pub fn compile_data_moat_service() -> Result<PathBuf> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let project_name = format!("output/{}_data_moat_service", timestamp);
        let project_path = Path::new(&project_name);
        fs::create_dir_all(project_path.join("src"))?;

        let mut env = Environment::new();

        // Template for Cargo.toml
        let cargo_template = fs::read_to_string("src/templates/cargo_template.toml.j2")?;
        env.add_template("cargo", &cargo_template)?;
        let tmpl = env.get_template("cargo")?;
        let cargo_content = tmpl.render(context! {})?;
        fs::write(project_path.join("Cargo.toml"), cargo_content)?;

        // Template for main.rs
        let main_template = fs::read_to_string("src/templates/main_data_moat.rs.j2")?;
        env.add_template("main", &main_template)?;
        let tmpl = env.get_template("main")?;
        let main_content = tmpl.render(context! {})?;
        fs::write(project_path.join("src/main.rs"), main_content)?;
        
        println!("COMPILER :: Generated Rust project at '{}'", project_path.display());

        // Compile the generated project
        println!("COMPILER :: Compiling generated binary... (this may take a moment)");
        let output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&project_path)
            .output()
            .context("Failed to execute cargo build")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Cargo build failed:\n{}", stderr));
        }

        println!("COMPILER :: Compilation successful.");
        Ok(project_path.to_path_buf())
    }
}