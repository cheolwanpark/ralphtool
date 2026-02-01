//! Coding agent abstraction module.
//!
//! This module provides a trait for different AI coding backends (Claude Code, Amp, etc.)
//! and implementations for spawning agents and capturing their output.
//!
//! Note: These types are defined for future integration with the TUI orchestrator.
//! The orchestrator will spawn agents when the loop execution screen is active.

#[allow(dead_code)]
pub mod claude;

use std::time::Duration;

use crate::error::Result;

/// Trait for AI coding agent backends.
///
/// Implementations spawn an AI agent with a prompt and configuration,
/// then return the agent's output when complete.
#[allow(dead_code)]
pub trait CodingAgent {
    /// Spawn agent with prompt and configuration, return output when complete.
    fn run(&self, prompt: &str, config: &AgentConfig) -> Result<AgentOutput>;
}

/// Configuration for spawning a coding agent.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgentConfig {
    /// Maximum number of turns before the agent stops.
    pub max_turns: u32,

    /// Timeout for the agent execution.
    pub timeout: Duration,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_turns: 50,
            timeout: Duration::from_secs(600), // 10 minutes
        }
    }
}

/// Output from a coding agent run.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AgentOutput {
    /// The result/response text from the agent.
    pub result: String,

    /// Session ID for the agent run (if available).
    pub session_id: String,

    /// Token usage statistics.
    pub usage: TokenUsage,
}

/// Token usage statistics from an agent run.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TokenUsage {
    /// Number of input tokens consumed.
    pub input_tokens: u64,

    /// Number of output tokens generated.
    pub output_tokens: u64,
}

// Re-export ClaudeAgent for future use
#[allow(unused_imports)]
pub use claude::ClaudeAgent;
