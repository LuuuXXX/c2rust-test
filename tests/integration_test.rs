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

    cmd.arg("test")
        .arg("--test.dir")
        .arg(dir_path)
        .arg("--test.cmd")
        .arg("cargo")
        .arg("--version");

    cmd.assert().success();
}

#[test]
fn test_missing_dir_argument() {
    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    cmd.arg("test")
        .arg("--test.cmd")
        .arg("echo")
        .arg("test");

    // Should fail because --test.dir is required
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_missing_command_argument() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    cmd.arg("test").arg("--test.dir").arg(dir_path);

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
        .stdout(predicate::str::contains("--test.dir"));
}
