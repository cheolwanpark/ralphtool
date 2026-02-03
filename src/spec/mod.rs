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
mod types;

pub use types::*;

use std::time::Duration;

use crate::error::Result;

/// Unified trait for spec system adapters.
///
/// Provides read-only operations needed by the agent layer:
/// - Reading stories and scenarios
/// - Getting context for a story
/// - Getting verification commands
/// - Providing spec-tool-specific usage instructions
///
/// Note: Agents mark tasks complete by directly editing tasks.md files,
/// so no mark_done method is needed here.
pub trait SpecAdapter {
    /// Returns all stories with their tasks.
    fn stories(&self) -> Result<Vec<Story>>;

    /// Returns all verification scenarios.
    fn scenarios(&self) -> Result<Vec<Scenario>>;

    /// Returns complete context for working on a story.
    fn context(&self, story_id: &str) -> Result<Context>;

    /// Returns verification commands (checks and tests) for the project.
    #[allow(dead_code)]
    fn verify_commands(&self) -> Result<VerifyCommands>;

    /// Returns spec-tool-specific usage instructions for the agent.
    ///
    /// This provides instructions on how to use the spec tool, including:
    /// - File locations (proposal.md, design.md, tasks.md, specs/)
    /// - How to mark tasks complete
    /// - Verification commands
    fn tool_prompt(&self) -> String;
}

/// Creates a spec adapter for the given change.
///
/// Currently only supports OpenSpec changes. When SpecKit support is added,
/// this factory can detect the spec system type and return the appropriate adapter.
#[allow(dead_code)]
pub fn create_adapter(change_name: &str) -> Result<Box<dyn SpecAdapter>> {
    let adapter = openspec::OpenSpecAdapter::new(change_name)?;
    Ok(Box::new(adapter))
}

/// Creates a spec adapter for the given change (async version).
///
/// Uses async command execution to avoid blocking tokio worker threads.
#[allow(dead_code)]
pub async fn create_adapter_async(change_name: &str) -> Result<Box<dyn SpecAdapter>> {
    let adapter = openspec::OpenSpecAdapter::new_async(change_name).await?;
    Ok(Box::new(adapter))
}

/// Creates a spec adapter for the given change with configurable timeout (async version).
///
/// Uses async command execution to avoid blocking tokio worker threads.
pub async fn create_adapter_async_with_timeout(
    change_name: &str,
    timeout: Duration,
) -> Result<Box<dyn SpecAdapter>> {
    let adapter = openspec::OpenSpecAdapter::new_async_with_timeout(change_name, timeout).await?;
    Ok(Box::new(adapter))
}
