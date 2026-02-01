//! Loop orchestration module for the Ralph Loop.
//!
//! This module provides the orchestration logic for autonomous AI development.
//! The simplified orchestrator spawns a single agent with a self-contained prompt.
//! The agent reads files directly and marks tasks complete by editing tasks.md.

mod orchestrator;

pub use orchestrator::Orchestrator;

use tokio::sync::mpsc;

/// Events emitted during loop execution.
///
/// Simplified to only emit agent output and completion status.
/// Story and task tracking is done by the agent via file edits.
#[derive(Debug, Clone)]
pub enum LoopEvent {
    /// Agent output line (for streaming display).
    AgentOutput {
        line: String,
    },

    /// An error occurred during loop execution.
    Error {
        message: String,
    },

    /// The loop has completed.
    Complete,
}

/// State tracking for the loop execution.
///
/// Simplified to track only running status. The agent manages
/// its own progress by reading and editing tasks.md directly.
#[derive(Debug, Clone)]
pub struct LoopState {
    /// Name of the change being processed.
    pub change_name: String,

    /// Whether the loop is running.
    pub running: bool,
}

impl LoopState {
    /// Create a new loop state for a change.
    pub fn new(change_name: &str) -> Self {
        Self {
            change_name: change_name.to_string(),
            running: false,
        }
    }
}

/// Sender for loop events.
pub type LoopEventSender = mpsc::Sender<LoopEvent>;

/// Receiver for loop events.
#[allow(dead_code)]
pub type LoopEventReceiver = mpsc::Receiver<LoopEvent>;

/// Create an event channel for loop communication.
#[allow(dead_code)]
pub fn event_channel(buffer: usize) -> (LoopEventSender, LoopEventReceiver) {
    mpsc::channel(buffer)
}
