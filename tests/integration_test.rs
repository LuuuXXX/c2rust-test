use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::env;

// Helper function to create a mock c2rust-config for testing
fn setup_mock_c2rust_config() -> std::path::PathBuf {
    let mock_script = r#"#!/bin/bash
# Mock c2rust-config for testing purposes

case "$1" in
  --help)
    echo "c2rust-config (mock version)"
    exit 0
    ;;
  config)
    # Mock config command - just succeed
    exit 0
    ;;
  *)
    echo "Unknown command: $1" >&2
    exit 1
    ;;
esac
"#;
    
    // Create a unique mock file per test invocation
    let temp_dir = TempDir::new().unwrap();
    let mock_path = temp_dir.path().join("c2rust-config");
    std::fs::write(&mock_path, mock_script).unwrap();
    
    // Make it executable on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&mock_path).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&mock_path, perms).unwrap();
    }
    
    // Leak the TempDir to prevent cleanup during test execution
    // The OS will clean up when the process exits
    std::mem::forget(temp_dir);
    
    mock_path
}

#[test]
fn test_test_command_basic() {
    let temp_dir = TempDir::new().unwrap();
    let mock_config = setup_mock_c2rust_config();

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    // Set the current directory for the command to run in
    cmd.current_dir(temp_dir.path())
        .env("C2RUST_CONFIG", mock_config)
        .arg("test")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert().success();
}

#[test]
fn test_missing_command_argument() {
    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    cmd.arg("test");

    // Should fail because command is required
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("c2rust-test"))
        .stdout(predicate::str::contains("C project test execution tool"));
}

#[test]
fn test_test_subcommand_help() {
    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    cmd.arg("test").arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Execute test command"))
        .stdout(predicate::str::contains("TEST_CMD"));
}

#[test]
fn test_with_separator() {
    let temp_dir = TempDir::new().unwrap();
    let mock_config = setup_mock_c2rust_config();

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    // Set the current directory for the command to run in
    cmd.current_dir(temp_dir.path())
        .env("C2RUST_CONFIG", mock_config)
        .arg("test")
        .arg("--")
        .arg("cargo")
        .arg("--version");

    cmd.assert().success();
}

#[test]
fn test_command_with_hyphen() {
    let temp_dir = TempDir::new().unwrap();
    let mock_config = setup_mock_c2rust_config();

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    // Test that we can pass arguments starting with hyphen after --
    // Set the current directory for the command to run in
    cmd.current_dir(temp_dir.path())
        .env("C2RUST_CONFIG", mock_config)
        .arg("test")
        .arg("--")
        .arg("echo")
        .arg("-n")
        .arg("test");

    cmd.assert().success();
}

#[test]
fn test_with_feature_flag() {
    let temp_dir = TempDir::new().unwrap();
    let mock_config = setup_mock_c2rust_config();

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    cmd.current_dir(temp_dir.path())
        .env("C2RUST_CONFIG", &mock_config)
        .arg("test")
        .arg("--feature")
        .arg("custom")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert().success().stdout(predicate::str::contains("Feature: custom"));
}

#[test]
fn test_without_c2rust_config() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    // Don't set C2RUST_CONFIG, so it should fail
    cmd.current_dir(temp_dir.path())
        .env_remove("C2RUST_CONFIG")
        .arg("test")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("c2rust-config not found"));
}
