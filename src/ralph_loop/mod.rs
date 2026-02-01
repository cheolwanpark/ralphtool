//! Loop orchestration module for the Ralph Loop.
//!
//! This module provides the orchestration logic for autonomous AI development.
//! It drives the Ralph loop by iterating through stories, spawning agents,
//! and tracking progress via events.
//!
//! Note: The orchestrator is defined for future TUI integration. Currently,
//! only the LoopState and event types are used by the UI layer.

#[allow(dead_code)]
mod orchestrator;

#[allow(unused_imports)]
pub use orchestrator::Orchestrator;

use tokio::sync::mpsc;

/// Events emitted during loop execution.
///
/// These events are used by the TUI to display real-time progress.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LoopEvent {
    /// A story has started processing.
    StoryStarted {
        story_id: String,
        title: String,
    },

    /// A task has been completed.
    TaskCompleted {
        task_id: String,
    },

    /// A story has been completed (all tasks done).
    StoryCompleted {
        story_id: String,
    },

    /// Agent output line (for streaming display).
    AgentOutput {
        line: String,
    },

    /// An error occurred during loop execution.
    Error {
        message: String,
    },

    /// The loop has completed (all stories done).
    Complete,
}

/// State tracking for the loop execution.
#[derive(Debug, Clone)]
pub struct LoopState {
    /// Name of the change being processed.
    pub change_name: String,

    /// Current story being worked on (if any).
    pub current_story: Option<String>,

    /// Number of stories completed.
    pub stories_completed: usize,

    /// Total number of stories.
    pub stories_total: usize,

    /// Number of tasks completed in current story.
    pub tasks_completed: usize,

    /// Total number of tasks in current story.
    pub tasks_total: usize,

    /// Whether the loop is running.
    pub running: bool,
}

impl LoopState {
    /// Create a new loop state for a change.
    pub fn new(change_name: &str) -> Self {
        Self {
            change_name: change_name.to_string(),
            current_story: None,
            stories_completed: 0,
            stories_total: 0,
            tasks_completed: 0,
            tasks_total: 0,
            running: false,
        }
    }
}

/// Sender for loop events.
#[allow(dead_code)]
pub type LoopEventSender = mpsc::Sender<LoopEvent>;

/// Receiver for loop events.
#[allow(dead_code)]
pub type LoopEventReceiver = mpsc::Receiver<LoopEvent>;

/// Create an event channel for loop communication.
#[allow(dead_code)]
pub fn event_channel(buffer: usize) -> (LoopEventSender, LoopEventReceiver) {
    mpsc::channel(buffer)
}
