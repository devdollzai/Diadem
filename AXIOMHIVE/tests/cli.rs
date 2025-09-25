// @AXIOMHIVE @DEVDOLLZAI
// Integration tests for the CLI

use std::process::Command;
use std::path::Path;

#[test]
fn test_weaver_compiles_data_moat_service() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("Generate a lightweight web service to serve as the core of a new data moat.")
        .output()
        .expect("Failed to execute cargo run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("SOVEREIGN BINARY COMPILED"));
    assert!(stdout.contains("target/release/"));
}

#[test]
fn test_generated_project_exists() {
    // Assuming the test above runs first
    let output_dir = Path::new("output");
    assert!(output_dir.exists());
    // Check for a generated project directory
    let entries = std::fs::read_dir(output_dir).unwrap();
    let has_project = entries.count() > 0;
    assert!(has_project);
}