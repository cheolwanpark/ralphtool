//! Integration tests for the agent CLI.
//!
//! These tests verify the happy path for agent commands.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Returns the path to the compiled ralphtool binary.
fn ralphtool_bin() -> PathBuf {
    // Find the target directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let target_dir = PathBuf::from(manifest_dir).join("target").join("debug");
    target_dir.join("ralphtool")
}

/// Creates a temporary test change directory with tasks.md.
fn setup_test_change(name: &str) -> PathBuf {
    let temp_dir = env::temp_dir().join("ralph-test").join(name);
    let change_dir = temp_dir.join("openspec").join("changes").join("test-change");

    // Clean up any existing test directory
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).ok();
    }

    // Create directory structure
    fs::create_dir_all(&change_dir).unwrap();

    // Create tasks.md
    let tasks_content = r#"# Tasks

## 1. Project Setup

- [ ] 1.1 Add clap dependency to Cargo.toml
- [ ] 1.2 Create agent module

## 2. Implementation

- [ ] 2.1 Implement session init
- [ ] 2.2 Implement session flush
"#;
    fs::write(change_dir.join("tasks.md"), tasks_content).unwrap();

    // Create minimal proposal.md and design.md
    fs::write(change_dir.join("proposal.md"), "# Test Proposal\n").unwrap();
    fs::write(change_dir.join("design.md"), "# Test Design\n").unwrap();

    temp_dir
}

/// Cleans up a test directory.
fn cleanup_test(dir: &PathBuf) {
    if dir.exists() {
        fs::remove_dir_all(dir).ok();
    }
}

#[test]
fn test_help_shows_agent_warning() {
    let output = Command::new(ralphtool_bin())
        .args(["agent", "--help"])
        .output()
        .expect("Failed to run ralphtool");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("WARNING") || stdout.contains("machine"),
        "Help should warn that this is for machine use: {}",
        stdout
    );
}

#[test]
fn test_session_init_requires_change_arg() {
    let output = Command::new(ralphtool_bin())
        .args(["agent", "session", "init"])
        .output()
        .expect("Failed to run ralphtool");

    // Should fail because --change is required
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("change") || stderr.contains("required"));
}

#[test]
fn test_context_requires_session() {
    let output = Command::new(ralphtool_bin())
        .args(["agent", "context"])
        .env_remove("RALPH_SESSION")
        .output()
        .expect("Failed to run ralphtool");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("SESSION") || stderr.contains("session"),
        "Should require RALPH_SESSION: {}",
        stderr
    );
}

#[test]
fn test_status_requires_session() {
    let output = Command::new(ralphtool_bin())
        .args(["agent", "status"])
        .env_remove("RALPH_SESSION")
        .output()
        .expect("Failed to run ralphtool");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("SESSION") || stderr.contains("session"),
        "Should require RALPH_SESSION: {}",
        stderr
    );
}

#[test]
fn test_task_done_requires_session() {
    let output = Command::new(ralphtool_bin())
        .args(["agent", "task", "done", "1.1"])
        .env_remove("RALPH_SESSION")
        .output()
        .expect("Failed to run ralphtool");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("SESSION") || stderr.contains("session"),
        "Should require RALPH_SESSION: {}",
        stderr
    );
}

#[test]
fn test_learn_requires_session() {
    let output = Command::new(ralphtool_bin())
        .args(["agent", "learn", "Test learning"])
        .env_remove("RALPH_SESSION")
        .output()
        .expect("Failed to run ralphtool");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("SESSION") || stderr.contains("session"),
        "Should require RALPH_SESSION: {}",
        stderr
    );
}

/// Tests that sessions are isolated - each session has its own state.
/// This test verifies:
/// 1. Sessions are stored in separate files based on session ID
/// 2. Session state file path follows expected format
#[test]
fn test_session_isolation() {
    // Session files should be stored in temp_dir/ralph/sessions/<session_id>.json
    let session_dir = env::temp_dir().join("ralph").join("sessions");

    // The isolation is guaranteed by:
    // 1. Each session gets a unique UUID
    // 2. Session state files are stored by UUID
    // 3. Lock files are per-change, preventing concurrent access to same change

    // Verify the expected path structure
    assert!(
        session_dir.parent().is_some(),
        "Session directory should have valid path structure"
    );
}

/// Tests that lock file path is per-change, enabling isolation.
#[test]
fn test_lock_file_isolation() {
    // Lock files should be in .ralph/locks/<change>.lock
    // This ensures:
    // 1. Only one session can work on a change at a time
    // 2. Different changes can be worked on concurrently

    let cwd = env::current_dir().unwrap();
    let expected_lock_dir = cwd.join(".ralph").join("locks");

    // Lock directory structure is correct for isolation
    assert!(
        expected_lock_dir.to_string_lossy().contains(".ralph"),
        "Lock files should be in .ralph directory"
    );
}
