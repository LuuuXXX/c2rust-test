use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_test_command_basic() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    // Set the current directory for the command to run in
    cmd.current_dir(temp_dir.path())
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

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    // Set the current directory for the command to run in
    cmd.current_dir(temp_dir.path())
        .arg("test")
        .arg("--")
        .arg("cargo")
        .arg("--version");

    cmd.assert().success();
}

#[test]
fn test_command_with_hyphen() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();

    // Test that we can pass arguments starting with hyphen after --
    // Set the current directory for the command to run in
    cmd.current_dir(temp_dir.path())
        .arg("test")
        .arg("--")
        .arg("echo")
        .arg("-n")
        .arg("test");

    cmd.assert().success();
}
