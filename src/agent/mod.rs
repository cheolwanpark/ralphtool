//! Coding agent abstraction module.
//!
//! This module provides a trait for different AI coding backends (Claude Code, Amp, etc.)
//! and implementations for spawning agents and capturing their output.
//!
//! Note: These types are defined for future integration with the TUI orchestrator.
//! The orchestrator will spawn agents when the loop execution screen is active.

#[allow(dead_code)]
pub mod claude;
mod prompt;

pub use prompt::PromptBuilder;

use crate::error::Result;

/// Prompt for a coding agent with separate system and user components.
#[derive(Debug, Clone, Default)]
pub struct Prompt {
    /// System prompt (appended to Claude's default system prompt).
    pub system: String,
    /// User prompt (the main instruction).
    pub user: String,
}

/// Response from a coding agent run with execution metadata.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Response {
    /// The result/response text from the agent.
    pub content: String,
    /// Number of turns taken by the agent.
    pub turns: u32,
    /// Total tokens used (input + output).
    pub tokens: u32,
    /// Total cost in USD.
    pub cost: f64,
}

/// Stream event from a coding agent.
#[derive(Debug, Clone)]
pub enum StreamEvent {
    /// Intermediate message from the agent.
    Message(String),
    /// Final result with execution metadata.
    Done(Response),
}

// Re-export ClaudeAgent and AgentStream for use
#[allow(unused_imports)]
pub use claude::{AgentStream, ClaudeAgent};

/// Trait for AI coding agent backends.
///
/// Implementations spawn an AI agent with a prompt,
/// then return a stream of events from the agent.
pub trait CodingAgent {
    /// Spawn agent with prompt, return a stream of events.
    fn run(&self, prompt: &Prompt) -> Result<AgentStream>;
}
