//! Progress tracking for learnings and patterns during Ralph Loop iterations.
//!
//! Learnings and patterns are accumulated in session state and flushed to design.md
//! when the session ends. This ensures:
//! - No partial writes on agent crash
//! - Data immediately available in context (from session state)
//! - Atomic batch write at end

use anyhow::Result;
use chrono::Utc;
use serde::Serialize;

use crate::agent::cli::{LearnArgs, PatternArgs};
use crate::agent::session::{get_session_id, load_session, save_session, SessionLearning, SessionPattern};

/// Response from the learn command.
#[derive(Debug, Serialize)]
pub struct LearnResponse {
    pub success: bool,
    pub learning: String,
    pub task_id: Option<String>,
    pub total_learnings: usize,
}

/// Response from the pattern command.
#[derive(Debug, Serialize)]
pub struct PatternResponse {
    pub success: bool,
    pub name: String,
    pub description: String,
    pub total_patterns: usize,
}

/// Runs the learn command.
pub fn run(args: LearnArgs) -> Result<()> {
    let session_id = get_session_id()?;
    let mut session = load_session(&session_id)?;

    // Get current story ID if set
    let story_id = std::env::var("RALPH_STORY").ok();

    // Create learning entry
    let learning = SessionLearning {
        description: args.description.clone(),
        task_id: args.task.clone(),
        story_id,
        timestamp: Utc::now().to_rfc3339(),
    };

    // Add to session
    session.learnings.push(learning);
    let total = session.learnings.len();

    // Save session
    save_session(&session)?;

    let response = LearnResponse {
        success: true,
        learning: args.description,
        task_id: args.task,
        total_learnings: total,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

/// Runs the pattern command.
pub fn run_pattern(args: PatternArgs) -> Result<()> {
    let session_id = get_session_id()?;
    let mut session = load_session(&session_id)?;

    // Create pattern entry
    let pattern = SessionPattern {
        name: args.name.clone(),
        description: args.description.clone(),
        timestamp: Utc::now().to_rfc3339(),
    };

    // Add to session
    session.patterns.push(pattern);
    let total = session.patterns.len();

    // Save session
    save_session(&session)?;

    let response = PatternResponse {
        success: true,
        name: args.name,
        description: args.description,
        total_patterns: total,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
