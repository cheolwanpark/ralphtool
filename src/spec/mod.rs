//! Spec abstraction layer for spec system adapters.
//!
//! This module defines the `SpecAdapter` trait and domain types that abstract
//! spec system operations. Adapters implement this trait to provide task, story,
//! context, and persistence capabilities from their respective spec systems
//! (OpenSpec, SpecKit, etc.).

pub mod openspec;
mod types;

pub use types::*;

use crate::error::Result;

/// Unified trait for spec system adapters.
///
/// Provides all operations needed by the agent layer:
/// - Reading stories and scenarios
/// - Getting context for a story
/// - Marking tasks as done
/// - Appending learnings
pub trait SpecAdapter {
    /// Returns all stories with their tasks.
    fn stories(&self) -> Result<Vec<Story>>;

    /// Returns all verification scenarios.
    fn scenarios(&self) -> Result<Vec<Scenario>>;

    /// Returns complete context for working on a story.
    fn context(&self, story_id: &str) -> Result<Context>;

    /// Marks a task as done in the source system.
    fn mark_done(&mut self, task_id: &str) -> Result<()>;

    /// Appends learnings to the spec system's native format.
    fn append_learnings(&mut self, learnings: &[String]) -> Result<()>;
}

/// Creates a spec adapter for the given change.
///
/// Currently only supports OpenSpec changes. When SpecKit support is added,
/// this factory can detect the spec system type and return the appropriate adapter.
pub fn create_adapter(change_name: &str) -> Result<Box<dyn SpecAdapter>> {
    let adapter = openspec::OpenSpecAdapter::new(change_name)?;
    Ok(Box::new(adapter))
}
