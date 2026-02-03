//! Async-safe command execution utilities.
//!
//! Provides async wrappers for executing external commands without blocking
//! tokio worker threads. Uses `tokio::task::spawn_blocking()` to run blocking
//! `std::process::Command` calls on a dedicated thread pool.

use std::process::{Command, Output};
use std::time::Duration;

use tokio::task::spawn_blocking;
use tokio::time::timeout;

use crate::error::{Error, Result};

/// Default timeout for command execution (30 seconds).
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Error types specific to async command execution.
#[derive(Debug)]
pub enum AsyncCmdError {
    /// Command execution timed out.
    Timeout,
    /// Command was not found in PATH.
    NotFound(String),
    /// Command execution failed.
    ExecutionFailed(String),
    /// Command exited with non-zero status.
    NonZeroExit { cmd: String, stderr: String },
}

impl std::fmt::Display for AsyncCmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsyncCmdError::Timeout => write!(f, "Command execution timed out"),
            AsyncCmdError::NotFound(program) => {
                write!(f, "Command not found: {}", program)
            }
            AsyncCmdError::ExecutionFailed(msg) => {
                write!(f, "Command execution failed: {}", msg)
            }
            AsyncCmdError::NonZeroExit { cmd, stderr } => {
                write!(f, "Command '{}' failed: {}", cmd, stderr)
            }
        }
    }
}

impl std::error::Error for AsyncCmdError {}

impl From<AsyncCmdError> for Error {
    fn from(err: AsyncCmdError) -> Self {
        match err {
            AsyncCmdError::Timeout => Error::Command {
                cmd: "timeout".to_string(),
                stderr: "Command execution timed out".to_string(),
            },
            AsyncCmdError::NotFound(program) => Error::Command {
                cmd: program.clone(),
                stderr: format!("Command not found: {}", program),
            },
            AsyncCmdError::ExecutionFailed(msg) => Error::Command {
                cmd: "unknown".to_string(),
                stderr: msg,
            },
            AsyncCmdError::NonZeroExit { cmd, stderr } => Error::Command { cmd, stderr },
        }
    }
}

/// Executes a command asynchronously with the default timeout.
///
/// Wraps the command in `tokio::task::spawn_blocking()` to avoid blocking
/// tokio worker threads, and applies a default timeout of 30 seconds.
///
/// # Arguments
/// * `program` - The program to execute
/// * `args` - Arguments to pass to the program
///
/// # Returns
/// * `Ok(Output)` - The command output on success
/// * `Err(Error)` - On timeout, command not found, or execution failure
///
/// # Example
/// ```ignore
/// let output = async_cmd::run("git", &["status"]).await?;
/// ```
#[allow(dead_code)]
pub async fn run(program: &str, args: &[&str]) -> Result<Output> {
    run_with_timeout(program, args, DEFAULT_TIMEOUT).await
}

/// Executes a command asynchronously with a configurable timeout.
///
/// Wraps the command in `tokio::task::spawn_blocking()` to avoid blocking
/// tokio worker threads, and applies the specified timeout.
///
/// # Arguments
/// * `program` - The program to execute
/// * `args` - Arguments to pass to the program
/// * `timeout_duration` - Maximum time to wait for the command
///
/// # Returns
/// * `Ok(Output)` - The command output on success
/// * `Err(Error)` - On timeout, command not found, or execution failure
pub async fn run_with_timeout(
    program: &str,
    args: &[&str],
    timeout_duration: Duration,
) -> Result<Output> {
    let program = program.to_string();
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let cmd_str = format!("{} {}", program, args.join(" "));

    // Spawn blocking task for command execution
    let handle = spawn_blocking(move || {
        Command::new(&program)
            .args(&args)
            .output()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    AsyncCmdError::NotFound(program.clone())
                } else {
                    AsyncCmdError::ExecutionFailed(e.to_string())
                }
            })
    });

    // Apply timeout
    let result = timeout(timeout_duration, handle).await;

    match result {
        Ok(Ok(Ok(output))) => {
            // Command executed, check exit status
            if output.status.success() {
                Ok(output)
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
                Err(AsyncCmdError::NonZeroExit {
                    cmd: cmd_str,
                    stderr,
                }
                .into())
            }
        }
        Ok(Ok(Err(async_err))) => Err(async_err.into()),
        Ok(Err(join_err)) => Err(Error::Command {
            cmd: cmd_str,
            stderr: format!("Task join error: {}", join_err),
        }),
        Err(_timeout_err) => Err(AsyncCmdError::Timeout.into()),
    }
}

/// Executes a command asynchronously and returns the stdout as a string.
///
/// This is a convenience wrapper that extracts stdout from successful commands.
///
/// # Arguments
/// * `program` - The program to execute
/// * `args` - Arguments to pass to the program
///
/// # Returns
/// * `Ok(String)` - The stdout output on success
/// * `Err(Error)` - On timeout, command not found, execution failure, or non-UTF8 output
#[allow(dead_code)]
pub async fn run_stdout(program: &str, args: &[&str]) -> Result<String> {
    let output = run(program, args).await?;
    String::from_utf8(output.stdout).map_err(|e| Error::Parse(format!("Invalid UTF-8: {}", e)))
}

/// Executes a command asynchronously with timeout and returns the stdout as a string.
///
/// # Arguments
/// * `program` - The program to execute
/// * `args` - Arguments to pass to the program
/// * `timeout_duration` - Maximum time to wait for the command
///
/// # Returns
/// * `Ok(String)` - The stdout output on success
/// * `Err(Error)` - On timeout, command not found, execution failure, or non-UTF8 output
pub async fn run_stdout_with_timeout(
    program: &str,
    args: &[&str],
    timeout_duration: Duration,
) -> Result<String> {
    let output = run_with_timeout(program, args, timeout_duration).await?;
    String::from_utf8(output.stdout).map_err(|e| Error::Parse(format!("Invalid UTF-8: {}", e)))
}

/// Executes a command asynchronously, returning the raw Output regardless of exit status.
///
/// Unlike `run()`, this does not treat non-zero exit codes as errors.
/// Useful when you need to inspect the output even on failure.
///
/// # Arguments
/// * `program` - The program to execute
/// * `args` - Arguments to pass to the program
///
/// # Returns
/// * `Ok(Output)` - The command output (success or failure)
/// * `Err(Error)` - On timeout, command not found, or execution failure
#[allow(dead_code)]
pub async fn run_unchecked(program: &str, args: &[&str]) -> Result<Output> {
    run_unchecked_with_timeout(program, args, DEFAULT_TIMEOUT).await
}

/// Executes a command asynchronously with timeout, returning raw Output regardless of exit status.
///
/// # Arguments
/// * `program` - The program to execute
/// * `args` - Arguments to pass to the program
/// * `timeout_duration` - Maximum time to wait for the command
///
/// # Returns
/// * `Ok(Output)` - The command output (success or failure)
/// * `Err(Error)` - On timeout, command not found, or execution failure
pub async fn run_unchecked_with_timeout(
    program: &str,
    args: &[&str],
    timeout_duration: Duration,
) -> Result<Output> {
    let program = program.to_string();
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let cmd_str = format!("{} {}", program, args.join(" "));

    let handle = spawn_blocking(move || {
        Command::new(&program)
            .args(&args)
            .output()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    AsyncCmdError::NotFound(program.clone())
                } else {
                    AsyncCmdError::ExecutionFailed(e.to_string())
                }
            })
    });

    let result = timeout(timeout_duration, handle).await;

    match result {
        Ok(Ok(Ok(output))) => Ok(output),
        Ok(Ok(Err(async_err))) => Err(async_err.into()),
        Ok(Err(join_err)) => Err(Error::Command {
            cmd: cmd_str,
            stderr: format!("Task join error: {}", join_err),
        }),
        Err(_timeout_err) => Err(AsyncCmdError::Timeout.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn run_executes_simple_command() {
        let output = run("echo", &["hello"]).await;
        assert!(output.is_ok());
        let output = output.unwrap();
        assert!(output.status.success());
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "hello");
    }

    #[tokio::test]
    async fn run_with_timeout_executes_command() {
        let output = run_with_timeout("echo", &["test"], Duration::from_secs(5)).await;
        assert!(output.is_ok());
    }

    #[tokio::test]
    async fn run_returns_error_for_nonexistent_command() {
        let result = run("nonexistent_command_xyz123", &[]).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("not found") || err.to_string().contains("NotFound"));
    }

    #[tokio::test]
    async fn run_returns_error_for_failed_command() {
        // `false` command always exits with status 1
        let result = run("false", &[]).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn run_stdout_returns_string() {
        let output = run_stdout("echo", &["hello world"]).await;
        assert!(output.is_ok());
        assert_eq!(output.unwrap().trim(), "hello world");
    }

    #[tokio::test]
    async fn run_unchecked_returns_output_on_failure() {
        // `false` exits with status 1
        let result = run_unchecked("false", &[]).await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.status.success());
    }

    #[tokio::test]
    async fn timeout_triggers_error() {
        // Use a very short timeout with a command that takes longer
        let result = run_with_timeout("sleep", &["10"], Duration::from_millis(50)).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("timed out"));
    }

    #[test]
    fn default_timeout_is_30_seconds() {
        assert_eq!(DEFAULT_TIMEOUT, Duration::from_secs(30));
    }

    #[test]
    fn async_cmd_error_displays_correctly() {
        let timeout_err = AsyncCmdError::Timeout;
        assert!(timeout_err.to_string().contains("timed out"));

        let not_found = AsyncCmdError::NotFound("git".to_string());
        assert!(not_found.to_string().contains("not found"));
        assert!(not_found.to_string().contains("git"));

        let exec_failed = AsyncCmdError::ExecutionFailed("permission denied".to_string());
        assert!(exec_failed.to_string().contains("permission denied"));

        let non_zero = AsyncCmdError::NonZeroExit {
            cmd: "test".to_string(),
            stderr: "error message".to_string(),
        };
        assert!(non_zero.to_string().contains("test"));
        assert!(non_zero.to_string().contains("error message"));
    }

    #[test]
    fn async_cmd_error_converts_to_error() {
        let timeout_err: Error = AsyncCmdError::Timeout.into();
        assert_eq!(timeout_err.code(), "COMMAND_ERROR");

        let not_found: Error = AsyncCmdError::NotFound("git".to_string()).into();
        assert_eq!(not_found.code(), "COMMAND_ERROR");
    }
}
