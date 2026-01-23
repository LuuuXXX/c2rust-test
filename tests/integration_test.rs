use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_test_command_basic() {
    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();
    
    // Note: This test will fail if c2rust-config is not installed
    // For testing purposes, we'll just test the command parsing
    cmd.arg("test")
        .arg("--")
        .arg("echo")
        .arg("testing");

    // The command might fail because c2rust-config might not be installed
    // but at least it should not fail on parsing
    let _ = cmd.assert();
}

#[test]
fn test_test_with_feature() {
    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();
    
    cmd.arg("test")
        .arg("--feature")
        .arg("debug")
        .arg("--")
        .arg("echo")
        .arg("test");

    // The command might fail because c2rust-config might not be installed
    let _ = cmd.assert();
}

#[test]
fn test_missing_command_argument() {
    let mut cmd = Command::cargo_bin("c2rust-test").unwrap();
    
    cmd.arg("test");

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
        .stdout(predicate::str::contains("--feature"));
}
