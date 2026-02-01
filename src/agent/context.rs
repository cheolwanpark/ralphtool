//! Context retrieval for the current story in a Ralph Loop iteration.
//!
//! The context command returns complete information needed by a coding agent
//! to work on the current story, including tasks, proposal, design, scenarios,
//! and learnings from previous iterations.

use std::fs;

use anyhow::Result;
use serde::Serialize;

use crate::agent::session::{get_session_id, get_story_id, load_session};
use crate::ralph::openspec::OpenSpecAdapter;
use crate::ralph::{TaskSource, VerificationSource};

/// Response from the context command.
#[derive(Debug, Serialize)]
pub struct ContextResponse {
    /// Current story information.
    pub story: StoryContext,

    /// All tasks in the current story.
    pub tasks: Vec<TaskContext>,

    /// Proposal content.
    pub proposal: String,

    /// Design content.
    pub design: String,

    /// Scenarios relevant to this story.
    pub scenarios: Vec<ScenarioContext>,

    /// Learnings from previous iterations.
    pub learnings: Vec<LearningContext>,

    /// Codebase patterns.
    pub patterns: Vec<String>,

    /// Verification commands.
    pub verify: VerifyCommands,
}

/// Story context information.
#[derive(Debug, Serialize)]
pub struct StoryContext {
    pub id: String,
    pub title: String,
    pub description: String,
}

/// Task context information.
#[derive(Debug, Serialize)]
pub struct TaskContext {
    pub id: String,
    pub description: String,
    pub complete: bool,
}

/// Scenario context information.
#[derive(Debug, Serialize)]
pub struct ScenarioContext {
    pub name: String,
    pub given: Vec<String>,
    pub when: String,
    pub then: Vec<String>,
}

/// Learning context information.
#[derive(Debug, Serialize)]
pub struct LearningContext {
    pub description: String,
    pub task_id: Option<String>,
    pub timestamp: String,
}

/// Verification commands.
#[derive(Debug, Serialize)]
pub struct VerifyCommands {
    /// Static check commands (e.g., cargo check, cargo clippy).
    pub checks: Vec<String>,

    /// Test command pattern.
    pub tests: String,
}

/// Runs the context command.
pub fn run() -> Result<()> {
    let session_id = get_session_id()?;
    let story_id = get_story_id()?;
    let session = load_session(&session_id)?;

    let adapter = OpenSpecAdapter::new(&session.change_name)?;

    // Get all stories and find the current one
    let stories = adapter.list_tasks()?;
    let current_story = stories
        .iter()
        .find(|s| s.id == story_id)
        .ok_or_else(|| anyhow::anyhow!("Story '{}' not found", story_id))?;

    // Build story context
    let story_context = StoryContext {
        id: current_story.id.clone(),
        title: current_story.title.clone(),
        description: String::new(), // Stories from tasks.md don't have descriptions
    };

    // Build task contexts
    let tasks: Vec<TaskContext> = current_story
        .tasks
        .iter()
        .map(|t| TaskContext {
            id: t.id.clone(),
            description: t.description.clone(),
            complete: t.complete,
        })
        .collect();

    // Read proposal and design
    let change_dir = get_change_dir(&session.change_name)?;
    let proposal = fs::read_to_string(change_dir.join("proposal.md")).unwrap_or_default();
    let design = fs::read_to_string(change_dir.join("design.md")).unwrap_or_default();

    // Get scenarios for this story
    let all_scenarios = adapter.list_scenarios()?;
    let scenarios: Vec<ScenarioContext> = all_scenarios
        .into_iter()
        .map(|s| ScenarioContext {
            name: s.name,
            given: s.given,
            when: s.when,
            then: s.then,
        })
        .collect();

    // Convert learnings from session
    let learnings: Vec<LearningContext> = session
        .learnings
        .iter()
        .map(|l| LearningContext {
            description: l.description.clone(),
            task_id: l.task_id.clone(),
            timestamp: l.timestamp.clone(),
        })
        .collect();

    // Infer verification commands from project type
    let verify = infer_verify_commands()?;

    let response = ContextResponse {
        story: story_context,
        tasks,
        proposal,
        design,
        scenarios,
        learnings,
        patterns: Vec::new(), // Patterns can be extracted from specs if needed
        verify,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

/// Gets the change directory path.
fn get_change_dir(change_name: &str) -> Result<std::path::PathBuf> {
    let cwd = std::env::current_dir()?;
    let change_dir = cwd.join("openspec").join("changes").join(change_name);
    if !change_dir.exists() {
        return Err(anyhow::anyhow!(
            "Change directory not found: {}",
            change_dir.display()
        ));
    }
    Ok(change_dir)
}

/// Infers verification commands from project type.
fn infer_verify_commands() -> Result<VerifyCommands> {
    let cwd = std::env::current_dir()?;

    // Check for Cargo.toml (Rust project)
    if cwd.join("Cargo.toml").exists() {
        return Ok(VerifyCommands {
            checks: vec![
                "cargo check".to_string(),
                "cargo clippy -- -D warnings".to_string(),
            ],
            tests: "cargo test".to_string(),
        });
    }

    // Check for package.json (Node.js project)
    if cwd.join("package.json").exists() {
        return Ok(VerifyCommands {
            checks: vec!["npm run lint".to_string()],
            tests: "npm test".to_string(),
        });
    }

    // Check for pyproject.toml or setup.py (Python project)
    if cwd.join("pyproject.toml").exists() || cwd.join("setup.py").exists() {
        return Ok(VerifyCommands {
            checks: vec!["python -m mypy .".to_string()],
            tests: "python -m pytest".to_string(),
        });
    }

    // Default/fallback
    Ok(VerifyCommands {
        checks: Vec::new(),
        tests: String::new(),
    })
}
