//! Claude Code agent implementation.
//!
//! This module provides a ClaudeAgent that implements the CodingAgent trait
//! by invoking the Claude CLI with streaming JSON output.

use std::io::{BufRead, BufReader, Lines};
use std::process::{Child, ChildStdout, Command, Stdio};

use serde::Deserialize;

use super::{CodingAgent, Prompt, Response, StreamEvent};
use crate::error::{Error, Result};

/// Streaming iterator over agent output.
///
/// Wraps a child process and parses NDJSON events from its stdout.
pub struct AgentStream {
    child: Child,
    lines: Lines<BufReader<ChildStdout>>,
    done: bool,
}

impl AgentStream {
    /// Create an AgentStream for testing purposes.
    #[cfg(test)]
    pub fn new_for_test(child: Child, lines: Lines<BufReader<ChildStdout>>) -> Self {
        Self {
            child,
            lines,
            done: false,
        }
    }
}

impl Iterator for AgentStream {
    type Item = StreamEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        loop {
            let line = match self.lines.next() {
                Some(Ok(line)) => line,
                Some(Err(_)) => {
                    self.done = true;
                    return None;
                }
                None => {
                    self.done = true;
                    return None;
                }
            };

            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }

            // Parse JSON event
            let event: ClaudeEvent = match serde_json::from_str(&line) {
                Ok(e) => e,
                Err(_) => continue, // Skip unparseable lines
            };

            match event {
                ClaudeEvent::System(_) => continue, // Ignore system events
                ClaudeEvent::Assistant(assistant) => {
                    // Extract text from first text content block
                    for content in assistant.message.content {
                        if let ClaudeContent::Text { text } = content {
                            return Some(StreamEvent::Message(text));
                        }
                    }
                    continue;
                }
                ClaudeEvent::Result(result) => {
                    self.done = true;
                    let response = Response {
                        content: result.result,
                        turns: result.num_turns,
                        tokens: result.usage.input_tokens + result.usage.output_tokens,
                        cost: result.total_cost_usd,
                    };
                    return Some(StreamEvent::Done(response));
                }
            }
        }
    }
}

impl Drop for AgentStream {
    fn drop(&mut self) {
        // Kill the child process if still running
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Claude Code agent implementation.
///
/// Invokes the `claude` CLI with `-p` (prompt) flag and `--output-format stream-json`
/// to get streaming NDJSON output.
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
fn build_command_args(prompt: &Prompt) -> Vec<String> {
    let mut args = vec![
        "-p".to_string(),
        prompt.user.clone(),
        "--output-format".to_string(),
        "stream-json".to_string(),
        "--verbose".to_string(),
        "--dangerously-skip-permissions".to_string(),
    ];

    // Add system prompt if non-empty
    if !prompt.system.is_empty() {
        args.push("--append-system-prompt".to_string());
        args.push(prompt.system.clone());
    }

    args
}

impl CodingAgent for ClaudeAgent {
    fn run(&self, prompt: &Prompt) -> Result<AgentStream> {
        let mut cmd = Command::new("claude");
        let args = build_command_args(prompt);
        cmd.args(&args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::null());

        let mut child = cmd.spawn().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Error::ClaudeNotFound
            } else {
                Error::AgentExecution(format!("Failed to spawn claude: {}", e))
            }
        })?;

        let stdout = child.stdout.take().ok_or_else(|| {
            Error::AgentExecution("Failed to capture stdout from claude process".to_string())
        })?;

        let reader = BufReader::new(stdout);
        let lines = reader.lines();

        Ok(AgentStream {
            child,
            lines,
            done: false,
        })
    }
}

/// Claude CLI streaming event wrapper.
/// The event type is determined by the "type" field.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ClaudeEvent {
    #[serde(rename = "system")]
    System(ClaudeSystemEvent),
    #[serde(rename = "assistant")]
    Assistant(ClaudeAssistantEvent),
    #[serde(rename = "result")]
    Result(ClaudeResultEvent),
}

/// System event from Claude CLI (ignored).
#[derive(Debug, Deserialize)]
struct ClaudeSystemEvent {}

/// Assistant message event from Claude CLI.
#[derive(Debug, Deserialize)]
struct ClaudeAssistantEvent {
    message: ClaudeMessage,
}

/// Message structure in assistant event.
#[derive(Debug, Deserialize)]
struct ClaudeMessage {
    content: Vec<ClaudeContent>,
}

/// Content block in a message.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ClaudeContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(other)]
    Other,
}

/// Result event from Claude CLI (final response).
#[derive(Debug, Deserialize)]
struct ClaudeResultEvent {
    result: String,
    #[serde(default)]
    num_turns: u32,
    #[serde(default)]
    total_cost_usd: f64,
    #[serde(default)]
    usage: ClaudeUsage,
}

/// Claude usage statistics from streaming result.
#[derive(Debug, Deserialize, Default)]
struct ClaudeUsage {
    #[serde(default)]
    input_tokens: u32,
    #[serde(default)]
    output_tokens: u32,
}

/// Parse a streaming event JSON line into a ClaudeEvent.
/// Extracted for testability.
fn parse_event(json_str: &str) -> std::result::Result<ClaudeEvent, serde_json::Error> {
    serde_json::from_str(json_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_system_event() {
        let json = r#"{"type":"system","subtype":"init","session_id":"abc","tools":[]}"#;
        let event = parse_event(json).unwrap();
        assert!(matches!(event, ClaudeEvent::System(_)));
    }

    #[test]
    fn parses_assistant_event() {
        let json = r#"{"type":"assistant","message":{"content":[{"type":"text","text":"Hello world"}]}}"#;
        let event = parse_event(json).unwrap();
        match event {
            ClaudeEvent::Assistant(assistant) => {
                assert_eq!(assistant.message.content.len(), 1);
                if let ClaudeContent::Text { text } = &assistant.message.content[0] {
                    assert_eq!(text, "Hello world");
                } else {
                    panic!("Expected text content");
                }
            }
            _ => panic!("Expected assistant event"),
        }
    }

    #[test]
    fn parses_result_event() {
        let json = r#"{"type":"result","result":"Done","num_turns":3,"total_cost_usd":0.05,"usage":{"input_tokens":100,"output_tokens":50}}"#;
        let event = parse_event(json).unwrap();
        match event {
            ClaudeEvent::Result(result) => {
                assert_eq!(result.result, "Done");
                assert_eq!(result.num_turns, 3);
                assert!((result.total_cost_usd - 0.05).abs() < 0.001);
                assert_eq!(result.usage.input_tokens, 100);
                assert_eq!(result.usage.output_tokens, 50);
            }
            _ => panic!("Expected result event"),
        }
    }

    #[test]
    fn parses_result_event_with_defaults() {
        let json = r#"{"type":"result","result":"Done"}"#;
        let event = parse_event(json).unwrap();
        match event {
            ClaudeEvent::Result(result) => {
                assert_eq!(result.result, "Done");
                assert_eq!(result.num_turns, 0);
                assert_eq!(result.total_cost_usd, 0.0);
                assert_eq!(result.usage.input_tokens, 0);
                assert_eq!(result.usage.output_tokens, 0);
            }
            _ => panic!("Expected result event"),
        }
    }

    #[test]
    fn parses_assistant_with_tool_use_content() {
        let json = r#"{"type":"assistant","message":{"content":[{"type":"tool_use","id":"123","name":"read"}]}}"#;
        let event = parse_event(json).unwrap();
        match event {
            ClaudeEvent::Assistant(assistant) => {
                assert_eq!(assistant.message.content.len(), 1);
                assert!(matches!(assistant.message.content[0], ClaudeContent::Other));
            }
            _ => panic!("Expected assistant event"),
        }
    }

    #[test]
    fn agent_new_creates_instance() {
        let agent = ClaudeAgent::new();
        assert!(std::mem::size_of_val(&agent) == 0); // Zero-sized type
    }

    #[test]
    fn build_args_includes_required_flags() {
        let prompt = Prompt {
            system: String::new(),
            user: "test prompt".to_string(),
        };

        let args = build_command_args(&prompt);
        assert!(args.contains(&"-p".to_string()));
        assert!(args.contains(&"test prompt".to_string()));
        assert!(args.contains(&"--output-format".to_string()));
        assert!(args.contains(&"stream-json".to_string()));
        assert!(args.contains(&"--verbose".to_string()));
        assert!(args.contains(&"--dangerously-skip-permissions".to_string()));
    }

    #[test]
    fn build_args_includes_system_prompt_when_provided() {
        let prompt = Prompt {
            system: "You are helpful".to_string(),
            user: "test prompt".to_string(),
        };

        let args = build_command_args(&prompt);
        assert!(args.contains(&"--append-system-prompt".to_string()));
        assert!(args.contains(&"You are helpful".to_string()));
    }

    #[test]
    fn build_args_excludes_system_prompt_when_empty() {
        let prompt = Prompt {
            system: String::new(),
            user: "test prompt".to_string(),
        };

        let args = build_command_args(&prompt);
        assert!(!args.contains(&"--append-system-prompt".to_string()));
    }
}
