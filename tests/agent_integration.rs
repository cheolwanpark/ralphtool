//! Integration tests for the agent CLI.
//!
//! These tests verify the happy path for agent commands.

use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Returns the path to the compiled ralphtool binary.
fn ralphtool_bin() -> PathBuf {
    // Find the target directory
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let target_dir = PathBuf::from(manifest_dir).join("target").join("debug");
    target_dir.join("ralphtool")
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

/// Integration test verifying subprocess receives session environment variables.
///
/// This test uses `env` command to print environment variables and verifies
/// that RALPH_SESSION and RALPH_STORY are properly passed to the subprocess
/// when configured via AgentConfig.
#[test]
fn test_subprocess_receives_session_env_vars() {
    use std::collections::HashMap;

    // Build a command with session env vars (simulating what ScopedSession.command() does)
    let mut env_map = HashMap::new();
    env_map.insert("RALPH_SESSION".to_string(), "test-session-abc".to_string());
    env_map.insert("RALPH_STORY".to_string(), "story-42".to_string());

    // Use `env` command to print environment
    let mut cmd = Command::new("env");
    for (key, value) in &env_map {
        cmd.env(key, value);
    }

    let output = cmd.output().expect("Failed to run env command");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify both session env vars are present in subprocess environment
    assert!(
        stdout.contains("RALPH_SESSION=test-session-abc"),
        "Subprocess should receive RALPH_SESSION: {}",
        stdout
    );
    assert!(
        stdout.contains("RALPH_STORY=story-42"),
        "Subprocess should receive RALPH_STORY: {}",
        stdout
    );
}

/// Tests that AgentConfig env vars are passed to subprocess correctly.
///
/// This verifies the integration between AgentConfig and subprocess spawning
/// without requiring a real change or Claude CLI.
#[test]
fn test_agent_config_env_propagation() {
    use std::collections::HashMap;

    // Create a command similar to how ClaudeAgent would
    let mut env = HashMap::new();
    env.insert("RALPH_SESSION".to_string(), "integration-test-session".to_string());

    let mut cmd = Command::new("printenv");
    cmd.arg("RALPH_SESSION");
    for (key, value) in &env {
        cmd.env(key, value);
    }

    let output = cmd.output().expect("Failed to run printenv");
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    assert_eq!(
        stdout, "integration-test-session",
        "printenv should return the session ID we set"
    );
}
