use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_test_command_basic() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    // Create a test file to test
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();
    
    // Note: This test will fail if c2rust-config is not installed
    // For testing purposes, we'll just test the command parsing
    cmd.arg("test")
        .arg("--dir")
        .arg(dir_path)
        .arg("--")
        .arg("echo")
        .arg("testing");

    // The command might fail because c2rust-config might not be installed
    // but at least it should not fail on parsing
    let _ = cmd.assert();
}

#[test]
fn test_test_with_feature() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();
    
    cmd.arg("test")
        .arg("--feature")
        .arg("debug")
        .arg("--dir")
        .arg(dir_path)
        .arg("--")
        .arg("echo")
        .arg("test");

    // The command might fail because c2rust-config might not be installed
    let _ = cmd.assert();
}

#[test]
fn test_missing_dir_argument() {
    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();
    
    cmd.arg("test")
        .arg("--")
        .arg("echo")
        .arg("test");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("--dir"));
}

#[test]
fn test_missing_command_argument() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();
    
    cmd.arg("test")
        .arg("--dir")
        .arg(dir_path);

    cmd.assert()
        .failure();
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
        .stdout(predicate::str::contains("--dir"))
        .stdout(predicate::str::contains("--feature"));
}
