//! Trait definitions for Ralph workflow concepts.
//!
//! Adapters implement these traits to provide task, story, and progress data
//! from their respective spec systems.

use crate::ralph::types::{Learning, Pattern, Scenario, Story, Task, UserStory};

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
// ProgressTracker Trait
// ============================================================================

/// Records learnings and patterns back to the source system.
///
/// Adapters implement this trait to persist progress information
/// in their native format.
pub trait ProgressTracker {
    /// The error type returned by this adapter.
    type Error;

    /// Records a learning to the source system.
    fn record_learning(&mut self, learning: Learning) -> Result<(), Self::Error>;

    /// Records a pattern to the source system.
    fn record_pattern(&mut self, pattern: Pattern) -> Result<(), Self::Error>;

    /// Returns all recorded patterns from the source system.
    fn list_patterns(&self) -> Result<Vec<Pattern>, Self::Error>;
}

// ============================================================================
// StoryProvider Trait
// ============================================================================

/// Provides user story data from a spec system backend.
///
/// Adapters implement this trait to expose user stories with their
/// acceptance criteria and priority information.
pub trait StoryProvider {
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
// VerificationSource Trait
// ============================================================================

/// Provides verification scenarios from a spec system backend.
///
/// Adapters implement this trait to expose Given/When/Then scenarios
/// for story verification.
pub trait VerificationSource {
    /// The error type returned by this adapter.
    type Error;

    /// Returns all verification scenarios associated with a story.
    fn scenarios_for(&self, story_id: &str) -> Result<Vec<Scenario>, Self::Error>;

    /// Returns all verification scenarios from the source system.
    fn list_scenarios(&self) -> Result<Vec<Scenario>, Self::Error>;
}
