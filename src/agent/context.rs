//! Context retrieval for the current story in a Ralph Loop iteration.
//!
//! The context command returns complete information needed by a coding agent
//! to work on the current story, including tasks, proposal, design, scenarios,
//! and learnings from previous iterations.

use anyhow::Result;
use serde::Serialize;

use crate::agent::session::{get_session_id, get_story_id, load_session};
use crate::spec::openspec::OpenSpecAdapter;
use crate::spec::ContextProvider;

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

    /// Learnings from previous iterations (from session state).
    pub learnings: Vec<LearningContext>,

    /// Codebase patterns (from session state).
    pub patterns: Vec<PatternContext>,

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

/// Pattern context information.
#[derive(Debug, Serialize)]
pub struct PatternContext {
    pub name: String,
    pub description: String,
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

    // Get context from adapter
    let work_context = adapter.get_context(&story_id)?;

    // Build story context
    let story_context = StoryContext {
        id: work_context.story.id.clone(),
        title: work_context.story.title.clone(),
        description: String::new(), // Stories from tasks.md don't have descriptions
    };

    // Build task contexts
    let tasks: Vec<TaskContext> = work_context
        .tasks
        .iter()
        .map(|t| TaskContext {
            id: t.id.clone(),
            description: t.description.clone(),
            complete: t.complete,
        })
        .collect();

    // Build scenario contexts
    let scenarios: Vec<ScenarioContext> = work_context
        .scenarios
        .iter()
        .map(|s| ScenarioContext {
            name: s.name.clone(),
            given: s.given.clone(),
            when: s.when.clone(),
            then: s.then.clone(),
        })
        .collect();

    // Get learnings from session state (not from adapter)
    let learnings: Vec<LearningContext> = session
        .learnings
        .iter()
        .map(|l| LearningContext {
            description: l.description.clone(),
            task_id: l.task_id.clone(),
            timestamp: l.timestamp.clone(),
        })
        .collect();

    // Get patterns from session state
    let patterns: Vec<PatternContext> = session
        .patterns
        .iter()
        .map(|p| PatternContext {
            name: p.name.clone(),
            description: p.description.clone(),
        })
        .collect();

    // Build verify commands
    let verify = VerifyCommands {
        checks: work_context.verify.checks,
        tests: work_context.verify.tests,
    };

    let response = ContextResponse {
        story: story_context,
        tasks,
        proposal: work_context.proposal,
        design: work_context.design,
        scenarios,
        learnings,
        patterns,
        verify,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
