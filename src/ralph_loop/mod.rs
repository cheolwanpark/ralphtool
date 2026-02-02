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
/// Includes story progress tracking and agent output for TUI display.
#[derive(Debug, Clone)]
pub enum LoopEvent {
    /// Progress on a story (emitted when starting each story).
    StoryProgress {
        /// ID of the current story.
        story_id: String,
        /// Title of the current story.
        story_title: String,
        /// Current story number (1-indexed).
        current: usize,
        /// Total number of stories.
        total: usize,
    },

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
/// Tracks current story being worked on and overall progress.
#[derive(Debug, Clone)]
pub struct LoopState {
    /// Name of the change being processed.
    pub change_name: String,

    /// Whether the loop is running.
    pub running: bool,

    /// ID of the current story being worked on.
    pub current_story_id: Option<String>,

    /// Total number of stories.
    pub total_stories: usize,

    /// Number of completed stories.
    pub completed_stories: usize,
}

impl LoopState {
    /// Create a new loop state for a change.
    pub fn new(change_name: &str) -> Self {
        Self {
            change_name: change_name.to_string(),
            running: false,
            current_story_id: None,
            total_stories: 0,
            completed_stories: 0,
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
