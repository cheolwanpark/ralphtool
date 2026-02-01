//! Spec abstraction layer for spec system adapters.
//!
//! This module defines the `SpecAdapter` trait and domain types that abstract
//! spec system operations. Adapters implement this trait to provide read-only
//! access to stories, scenarios, and verification commands from their respective
//! spec systems (OpenSpec, SpecKit, etc.).
//!
//! Note: Task marking and learnings are handled directly by the agent via file
//! edits, so those operations are not part of this trait.

pub mod openspec;
mod prompt;
mod types;

pub use prompt::generate_prompt;
pub use types::*;

use crate::error::Result;

/// Unified trait for spec system adapters.
///
/// Provides read-only operations needed by the agent layer:
/// - Reading stories and scenarios
/// - Getting context for a story
/// - Getting verification commands
///
/// Note: Agents mark tasks complete by directly editing tasks.md files,
/// so no mark_done method is needed here.
#[allow(dead_code)]
pub trait SpecAdapter {
    /// Returns all stories with their tasks.
    fn stories(&self) -> Result<Vec<Story>>;

    /// Returns all verification scenarios.
    fn scenarios(&self) -> Result<Vec<Scenario>>;

    /// Returns complete context for working on a story.
    fn context(&self, story_id: &str) -> Result<Context>;

    /// Returns verification commands (checks and tests) for the project.
    fn verify_commands(&self) -> Result<VerifyCommands>;
}

/// Creates a spec adapter for the given change.
///
/// Currently only supports OpenSpec changes. When SpecKit support is added,
/// this factory can detect the spec system type and return the appropriate adapter.
pub fn create_adapter(change_name: &str) -> Result<Box<dyn SpecAdapter>> {
    let adapter = openspec::OpenSpecAdapter::new(change_name)?;
    Ok(Box::new(adapter))
}
