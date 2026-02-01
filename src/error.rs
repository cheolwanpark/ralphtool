//! Custom error types for ralphtool.
//!
//! Provides a unified error type with machine-readable error codes.

use std::fmt;

/// Custom error type for ralphtool operations.
#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    /// Change not found.
    ChangeNotFound(String),
    /// Task not found.
    TaskNotFound(String),
    /// Story not found.
    StoryNotFound(String),
    /// IO error.
    Io(std::io::Error),
    /// JSON parsing error.
    Json(serde_json::Error),
    /// Command execution error.
    Command { cmd: String, stderr: String },
    /// Parse error.
    Parse(String),
    /// Claude CLI not found.
    ClaudeNotFound,
    /// Agent execution error.
    AgentExecution(String),
    /// Agent output error.
    AgentOutput(String),
}

/// Result type alias using ralphtool's Error.
pub type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code)]
impl Error {
    /// Returns a machine-readable error code.
    pub fn code(&self) -> &'static str {
        match self {
            Error::ChangeNotFound(_) => "CHANGE_NOT_FOUND",
            Error::TaskNotFound(_) => "TASK_NOT_FOUND",
            Error::StoryNotFound(_) => "STORY_NOT_FOUND",
            Error::Io(_) => "IO_ERROR",
            Error::Json(_) => "JSON_ERROR",
            Error::Command { .. } => "COMMAND_ERROR",
            Error::Parse(_) => "PARSE_ERROR",
            Error::ClaudeNotFound => "CLAUDE_NOT_FOUND",
            Error::AgentExecution(_) => "AGENT_EXECUTION_ERROR",
            Error::AgentOutput(_) => "AGENT_OUTPUT_ERROR",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ChangeNotFound(name) => write!(f, "Change not found: {}", name),
            Error::TaskNotFound(id) => write!(f, "Task not found: {}", id),
            Error::StoryNotFound(id) => write!(f, "Story not found: {}", id),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::Command { cmd, stderr } => {
                write!(f, "Command '{}' failed: {}", cmd, stderr)
            }
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::ClaudeNotFound => write!(
                f,
                "Claude CLI not found.\n\
                 Please ensure Claude Code is installed and in your PATH."
            ),
            Error::AgentExecution(msg) => write!(f, "Agent execution error: {}", msg),
            Error::AgentOutput(msg) => write!(f, "Agent output error: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_machine_readable() {
        assert_eq!(Error::ChangeNotFound("x".into()).code(), "CHANGE_NOT_FOUND");
        assert_eq!(Error::TaskNotFound("1.1".into()).code(), "TASK_NOT_FOUND");
        assert_eq!(Error::StoryNotFound("1".into()).code(), "STORY_NOT_FOUND");
    }

    #[test]
    fn io_error_converts() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert_eq!(err.code(), "IO_ERROR");
    }

    #[test]
    fn error_displays_correctly() {
        let err = Error::ChangeNotFound("my-change".into());
        assert!(err.to_string().contains("my-change"));
    }
}
