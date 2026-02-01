//! Trait definitions for spec abstraction concepts.
//!
//! Adapters implement these traits to provide task, story, context, and persistence
//! capabilities from their respective spec systems.

use crate::spec::types::{Scenario, Story, Task, UserStory};

// ============================================================================
// TaskSource Trait
// ============================================================================

/// Provides task data from a spec system backend.
///
/// Adapters implement this trait to expose tasks in the Ralph-compatible
/// hierarchical format (Story > Task).
pub trait TaskSource {
    /// The error type returned by this adapter.
    type Error;

    /// Returns all tasks in hierarchical form (Story > Task).
    fn list_tasks(&self) -> Result<Vec<Story>, Self::Error>;

    /// Returns the next incomplete task in priority order.
    ///
    /// Returns `None` if all tasks are complete.
    fn next_task(&self) -> Result<Option<Task>, Self::Error>;

    /// Marks a task as complete in the source system.
    fn mark_complete(&mut self, task_id: &str) -> Result<(), Self::Error>;
}

// ============================================================================
// SpecWriter Trait
// ============================================================================

/// A learning captured during task execution.
///
/// Used for passing learnings to the SpecWriter trait for persistence.
#[derive(Debug, Clone)]
pub struct Learning {
    /// Description of what was learned.
    pub description: String,
    /// Optional reference to the task this learning relates to.
    pub task_id: Option<String>,
    /// Story ID when this learning was recorded.
    pub story_id: Option<String>,
}

/// A reusable pattern discovered in the codebase.
///
/// Used for passing patterns to the SpecWriter trait for persistence.
#[derive(Debug, Clone)]
pub struct Pattern {
    /// Name identifying the pattern.
    pub name: String,
    /// Description of the pattern and how to use it.
    pub description: String,
}

/// Writes learnings and patterns to the spec system on session flush.
///
/// The agent layer owns learning/pattern accumulation in session state.
/// This trait is called when the session ends to persist the data.
pub trait SpecWriter {
    /// The error type returned by this adapter.
    type Error;

    /// Writes all learnings to the spec system's native format.
    ///
    /// For OpenSpec, this appends to design.md under a `## Learnings` section.
    fn write_learnings(&mut self, learnings: &[Learning]) -> Result<(), Self::Error>;

    /// Writes all patterns to the spec system's native format.
    ///
    /// For OpenSpec, this appends to design.md under a patterns section.
    fn write_patterns(&mut self, patterns: &[Pattern]) -> Result<(), Self::Error>;
}

// ============================================================================
// StorySource Trait
// ============================================================================

/// Provides user story data from a spec system backend.
///
/// Adapters implement this trait to expose user stories with their
/// acceptance criteria and priority information.
pub trait StorySource {
    /// The error type returned by this adapter.
    type Error;

    /// Returns all user stories with their metadata.
    fn list_stories(&self) -> Result<Vec<UserStory>, Self::Error>;

    /// Returns the highest-priority story that hasn't passed verification.
    ///
    /// Returns `None` if all stories have passed.
    fn next_story(&self) -> Result<Option<UserStory>, Self::Error>;

    /// Marks a story as passed in the source system.
    fn mark_passed(&mut self, story_id: &str) -> Result<(), Self::Error>;
}

// ============================================================================
// ScenarioSource Trait
// ============================================================================

/// Provides verification scenarios from a spec system backend.
///
/// Adapters implement this trait to expose Given/When/Then scenarios
/// for story verification.
pub trait ScenarioSource {
    /// The error type returned by this adapter.
    type Error;

    /// Returns all verification scenarios associated with a story.
    fn scenarios_for(&self, story_id: &str) -> Result<Vec<Scenario>, Self::Error>;

    /// Returns all verification scenarios from the source system.
    fn list_scenarios(&self) -> Result<Vec<Scenario>, Self::Error>;
}

// ============================================================================
// ContextProvider Trait
// ============================================================================

/// Verification commands for a project.
#[derive(Debug, Clone)]
pub struct VerifyCommands {
    /// Static check commands (e.g., cargo check, cargo clippy).
    pub checks: Vec<String>,
    /// Test command pattern.
    pub tests: String,
}

/// Complete context needed by an agent to work on a story.
#[derive(Debug, Clone)]
pub struct WorkContext {
    /// Current story information.
    pub story: Story,
    /// All tasks in the current story.
    pub tasks: Vec<Task>,
    /// Proposal content.
    pub proposal: String,
    /// Design content.
    pub design: String,
    /// Scenarios relevant to this story.
    pub scenarios: Vec<Scenario>,
    /// Verification commands.
    pub verify: VerifyCommands,
}

/// Status of work in progress.
#[derive(Debug, Clone)]
pub struct WorkStatus {
    /// Current story ID.
    pub story_id: String,
    /// Tasks done in current story.
    pub story_tasks_done: usize,
    /// Total tasks in current story.
    pub story_tasks_total: usize,
    /// Stories done in current change.
    pub change_stories_done: usize,
    /// Total stories in current change.
    pub change_stories_total: usize,
}

/// Provides unified context for agent work.
///
/// Replaces direct file access in agent layer with abstraction.
pub trait ContextProvider {
    /// The error type returned by this adapter.
    type Error;

    /// Returns complete context for working on a story.
    fn get_context(&self, story_id: &str) -> Result<WorkContext, Self::Error>;

    /// Returns current work status.
    fn get_status(&self) -> Result<WorkStatus, Self::Error>;
}
