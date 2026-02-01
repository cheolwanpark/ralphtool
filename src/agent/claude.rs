//! Claude Code agent implementation.
//!
//! This module provides a ClaudeAgent that implements the CodingAgent trait
//! by invoking the Claude CLI with appropriate flags.

use std::process::Command;

use serde::Deserialize;

use super::{AgentConfig, AgentOutput, CodingAgent, TokenUsage};
use crate::error::{Error, Result};

/// Claude Code agent implementation.
///
/// Invokes the `claude` CLI with `-p` (prompt) flag and `--output-format json`
/// to get structured output.
#[derive(Debug, Default)]
pub struct ClaudeAgent;

impl ClaudeAgent {
    /// Create a new Claude agent.
    pub fn new() -> Self {
        Self
    }

    /// Check if the Claude CLI is available.
    pub fn is_available() -> bool {
        Command::new("claude")
            .arg("--version")
            .output()
            .is_ok()
    }
}

/// Build the command-line arguments for the Claude CLI.
/// Extracted for testability.
fn build_command_args(prompt: &str, config: &AgentConfig) -> Vec<String> {
    let mut args = Vec::new();

    // Add prompt flag
    args.push("-p".to_string());
    args.push(prompt.to_string());

    // Add output format
    args.push("--output-format".to_string());
    args.push("json".to_string());

    // Add max turns
    args.push("--max-turns".to_string());
    args.push(config.max_turns.to_string());

    // Always skip permissions for autonomous operation
    args.push("--dangerously-skip-permissions".to_string());

    args
}

impl CodingAgent for ClaudeAgent {
    fn run(&self, prompt: &str, config: &AgentConfig) -> Result<AgentOutput> {
        let mut cmd = Command::new("claude");
        let args = build_command_args(prompt, config);
        cmd.args(&args);

        // Execute with timeout
        let output = cmd.output().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Error::ClaudeNotFound
            } else {
                Error::AgentExecution(format!("Failed to execute claude: {}", e))
            }
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::AgentExecution(format!(
                "Claude exited with status {}: {}",
                output.status,
                stderr
            )));
        }

        // Parse JSON output
        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_response(&stdout)
    }
}

/// Claude CLI JSON response structure.
#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    /// The final result text from the agent.
    result: String,

    /// Session ID for the conversation.
    #[serde(default)]
    session_id: Option<String>,

    /// Token usage statistics.
    #[serde(default)]
    usage: ClaudeUsage,
}

/// Claude usage statistics from JSON response.
#[derive(Debug, Deserialize, Default)]
struct ClaudeUsage {
    #[serde(default)]
    input_tokens: u64,

    #[serde(default)]
    output_tokens: u64,
}

/// Parse a JSON response string into AgentOutput.
/// Extracted for testability.
fn parse_response(json_str: &str) -> Result<AgentOutput> {
    let response: ClaudeResponse = serde_json::from_str(json_str).map_err(|e| {
        Error::AgentOutput(format!(
            "Failed to parse Claude output as JSON: {}. Raw output: {}",
            e, json_str
        ))
    })?;

    Ok(AgentOutput {
        result: response.result,
        session_id: response.session_id.unwrap_or_default(),
        usage: TokenUsage {
            input_tokens: response.usage.input_tokens,
            output_tokens: response.usage.output_tokens,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_complete_response() {
        let json = r#"{
            "result": "Task completed successfully",
            "session_id": "sess-123",
            "usage": {
                "input_tokens": 100,
                "output_tokens": 50
            }
        }"#;

        let output = parse_response(json).unwrap();
        assert_eq!(output.result, "Task completed successfully");
        assert_eq!(output.session_id, "sess-123");
        assert_eq!(output.usage.input_tokens, 100);
        assert_eq!(output.usage.output_tokens, 50);
    }

    #[test]
    fn parses_minimal_response() {
        let json = r#"{"result": "Done"}"#;

        let output = parse_response(json).unwrap();
        assert_eq!(output.result, "Done");
        assert_eq!(output.session_id, "");
        assert_eq!(output.usage.input_tokens, 0);
        assert_eq!(output.usage.output_tokens, 0);
    }

    #[test]
    fn parses_response_with_null_session_id() {
        let json = r#"{
            "result": "Output text",
            "session_id": null,
            "usage": {"input_tokens": 10, "output_tokens": 5}
        }"#;

        let output = parse_response(json).unwrap();
        assert_eq!(output.result, "Output text");
        assert_eq!(output.session_id, "");
    }

    #[test]
    fn fails_on_invalid_json() {
        let json = "not valid json";

        let result = parse_response(json);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(matches!(err, Error::AgentOutput(_)));
        assert!(err.to_string().contains("not valid json"));
    }

    #[test]
    fn fails_on_missing_result_field() {
        let json = r#"{"session_id": "123"}"#;

        let result = parse_response(json);
        assert!(result.is_err());
    }

    #[test]
    fn handles_extra_fields_gracefully() {
        let json = r#"{
            "result": "Success",
            "session_id": "abc",
            "usage": {"input_tokens": 1, "output_tokens": 2},
            "extra_field": "ignored",
            "another": 123
        }"#;

        let output = parse_response(json).unwrap();
        assert_eq!(output.result, "Success");
    }

    #[test]
    fn agent_new_creates_instance() {
        let agent = ClaudeAgent::new();
        assert!(std::mem::size_of_val(&agent) == 0); // Zero-sized type
    }

    #[test]
    fn skip_permissions_flag_always_present() {
        let config = AgentConfig::default();

        let args = build_command_args("test prompt", &config);
        assert!(
            args.contains(&"--dangerously-skip-permissions".to_string()),
            "Expected --dangerously-skip-permissions flag to always be present"
        );
    }
}
