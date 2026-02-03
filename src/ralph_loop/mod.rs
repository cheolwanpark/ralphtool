//! Loop orchestration module for the Ralph Loop.
//!
//! This module provides the orchestration logic for autonomous AI development.
//! The simplified orchestrator spawns a single agent with a self-contained prompt.
//! The agent reads files directly and marks tasks complete by editing tasks.md.

pub mod learnings;
mod orchestrator;

pub use orchestrator::{Orchestrator, DEFAULT_MAX_RETRIES};

// Re-export CompletionOption from checkpoint module for TUI use
pub use crate::checkpoint::CompletionOption;

/// Default timeout in seconds for external commands (git, openspec).
pub const DEFAULT_COMMAND_TIMEOUT_SECS: u64 = 30;

use crate::agent::StreamEvent;
use tokio::sync::{mpsc, oneshot};

/// Events emitted during loop execution.
///
/// Includes story progress tracking and agent output for TUI display.
#[derive(Debug)]
pub enum LoopEvent {
    /// Progress on a story (emitted when starting each story).
    StoryProgress {
        /// ID of the current story.
        story_id: String,
        /// Title of the current story.
        #[allow(dead_code)] // Used in Story 5 UI rendering
        story_title: String,
        /// Current story number (1-indexed).
        #[allow(dead_code)] // Used in Story 5 UI rendering
        current: usize,
        /// Total number of stories.
        total: usize,
        /// Number of completed stories.
        #[allow(dead_code)] // Used in Story 3 app event handler
        completed: usize,
    },

    /// Agent event with story context (for streaming display).
    StoryEvent {
        /// ID of the story this event belongs to.
        story_id: String,
        /// The stream event from the agent.
        event: StreamEvent,
    },

    /// An error occurred during loop execution.
    Error {
        #[allow(dead_code)] // Used in Story 5 UI rendering
        message: String,
    },

    /// Max retries exceeded for a story.
    MaxRetriesExceeded {
        /// ID of the story that exceeded max retries.
        story_id: String,
    },

    /// Orchestrator is awaiting user choice for completion action.
    /// TUI should show completion screen and send choice via the oneshot sender.
    AwaitingUserChoice {
        /// Sender to communicate user's completion choice back to orchestrator.
        choice_tx: oneshot::Sender<CompletionOption>,
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

    /// IDs of stories that have been started, in order.
    pub started_story_ids: Vec<String>,
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
            started_story_ids: Vec::new(),
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
