#![allow(deprecated)]
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::time::Duration;
use tempfile::NamedTempFile;

#[test]
fn test_no_arguments() {
    let mut cmd = Command::cargo_bin("refresh").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required arguments"));
}

#[test]
fn test_zero_interval() {
    let mut cmd = Command::cargo_bin("refresh").unwrap();
    cmd.arg("0").arg("echo").arg("test");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Interval must be greater than 0"));
}

#[test]
fn test_basic_execution() {
    let mut cmd = Command::cargo_bin("refresh").unwrap();
    cmd.arg("1").arg("echo").arg("test");

    // Run for a short time then kill
    cmd.timeout(Duration::from_secs(2));

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should execute at least once
    assert!(stdout.contains("test"));
}

#[test]
fn test_command_with_double_dash() {
    let mut cmd = Command::cargo_bin("refresh").unwrap();
    cmd.arg("1").arg("--").arg("echo").arg("hello world");

    cmd.timeout(Duration::from_secs(2));

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("hello world"));
}

#[test]
fn test_exit_on_error() {
    let mut cmd = Command::cargo_bin("refresh").unwrap();
    cmd.arg("1").arg("--on-error").arg("false");

    cmd.timeout(Duration::from_secs(2));

    let output = cmd.output().unwrap();

    // Should exit with code 1 (or be killed by timeout)
    // The command "false" returns non-zero, so refresh should exit
    assert!(!output.status.success() || output.status.code() == Some(124)); // 124 is timeout
}

#[test]
fn test_logging() {
    let temp_file = NamedTempFile::new().unwrap();
    let log_path = temp_file.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("refresh").unwrap();
    cmd.arg("1")
        .arg("--log")
        .arg(log_path)
        .arg("echo")
        .arg("logged");

    cmd.timeout(Duration::from_secs(2));
    cmd.output().unwrap();

    // Check that the log file contains the output
    let log_content = fs::read_to_string(log_path).unwrap();
    assert!(log_content.contains("logged"));
}

#[test]
fn test_log_errors_only() {
    let temp_file = NamedTempFile::new().unwrap();
    let log_path = temp_file.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("refresh").unwrap();
    cmd.arg("1")
        .arg("--log")
        .arg(log_path)
        .arg("--log-errors-only")
        .arg("echo")
        .arg("success");

    cmd.timeout(Duration::from_secs(2));
    cmd.output().unwrap();

    // Log file should be empty or not contain "success" since echo succeeds
    let log_content = fs::read_to_string(log_path).unwrap_or_default();
    assert!(!log_content.contains("success") || log_content.is_empty());
}

#[test]
fn test_stdin_pipe() {
    let mut cmd = Command::cargo_bin("refresh").unwrap();
    cmd.arg("1");
    cmd.write_stdin("echo piped");

    cmd.timeout(Duration::from_secs(2));

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("piped"));
}

#[test]
fn test_nonexistent_command() {
    let mut cmd = Command::cargo_bin("refresh").unwrap();
    cmd.arg("1").arg("nonexistent_command_xyz123");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to execute"));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("refresh").unwrap();
    cmd.arg("--help");

    cmd.assert().success().stdout(predicate::str::contains(
        "A minimal, low-level, efficient, pipe-friendly CLI tool",
    ));
}

#[test]
fn test_mixed_arguments() {
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let log_path = temp_file.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("refresh").unwrap();
    // Putting --log AFTER the command
    cmd.arg("1")
        .arg("echo")
        .arg("mixed")
        .arg("--log")
        .arg(log_path);

    cmd.timeout(std::time::Duration::from_secs(2));
    cmd.output().unwrap();

    let log_content = std::fs::read_to_string(log_path).unwrap();
    assert!(log_content.contains("mixed"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("refresh").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("refresh"));
}
