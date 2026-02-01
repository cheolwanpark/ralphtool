//! Agent CLI module for machine-to-machine interaction.
//!
//! This module provides a structured CLI interface for coding agents (Claude, Amp)
//! to interact with Ralph state. All commands output JSON and require session management.
//!
//! **Note:** This interface is designed for machine use. Human users should use the TUI.

pub mod cli;
pub mod context;
pub mod progress;
pub mod session;
pub mod tasks;

use anyhow::Result;
use serde::Serialize;

use cli::AgentCommand;

/// Standard error response format for all agent commands.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

impl ErrorResponse {
    pub fn new(error: &anyhow::Error) -> Self {
        // Extract error code from the error chain
        let code = classify_error(error);
        Self {
            success: false,
            error: format!("{:#}", error),
            code,
        }
    }

    pub fn print(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            eprintln!("{}", json);
        }
    }
}

/// Classifies an error into a machine-readable code.
fn classify_error(error: &anyhow::Error) -> String {
    let msg = error.to_string();
    if msg.contains("RALPH_SESSION") {
        "SESSION_REQUIRED".to_string()
    } else if msg.contains("RALPH_STORY") {
        "STORY_REQUIRED".to_string()
    } else if msg.contains("not found") || msg.contains("not exist") {
        "NOT_FOUND".to_string()
    } else if msg.contains("locked") {
        "LOCKED".to_string()
    } else if msg.contains("not in the current story") {
        "SCOPE_ERROR".to_string()
    } else {
        "UNKNOWN".to_string()
    }
}

/// Runs the agent CLI with the given command.
///
/// On error, outputs a consistent JSON error response to stderr.
pub fn run(command: AgentCommand) -> Result<()> {
    let result = match command {
        AgentCommand::Session(cmd) => session::run(cmd),
        AgentCommand::Context => context::run(),
        AgentCommand::Task(cmd) => tasks::run(cmd),
        AgentCommand::Status => tasks::run_status(),
        AgentCommand::Learn(cmd) => progress::run(cmd),
    };

    // If there's an error, format it as JSON
    if let Err(ref e) = result {
        ErrorResponse::new(e).print();
    }

    result
}
