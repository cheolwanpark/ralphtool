//! Agent CLI module for machine-to-machine interaction.
//!
//! This module provides a structured CLI interface for coding agents (Claude, Amp)
//! to interact with Ralph state. All commands output JSON and require session management.
//!
//! **Note:** This interface is designed for machine use. Human users should use the TUI.

pub mod cli;
pub mod session;

use serde::Serialize;

use cli::AgentCommand;
use crate::error::{Error, Result};
use crate::spec;

/// Standard error response format for all agent commands.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

impl ErrorResponse {
    pub fn new(error: &Error) -> Self {
        Self {
            success: false,
            error: error.to_string(),
            code: error.code().to_string(),
        }
    }

    pub fn print(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            eprintln!("{}", json);
        }
    }
}

/// Runs the agent CLI with the given command.
///
/// On error, outputs a consistent JSON error response to stderr.
pub fn run(command: AgentCommand) -> anyhow::Result<()> {
    let result = match command {
        AgentCommand::Session(cmd) => session::run(cmd),
        AgentCommand::Context => run_context(),
        AgentCommand::Task(cmd) => run_task(cmd),
        AgentCommand::Status => run_status(),
        AgentCommand::Learn(args) => run_learn(args),
    };

    // If there's an error, format it as JSON
    if let Err(ref e) = result {
        ErrorResponse::new(e).print();
    }

    result.map_err(|e| anyhow::anyhow!("{}", e))
}

// ============================================================================
// Context Command
// ============================================================================

/// Response from the context command.
#[derive(Debug, Serialize)]
pub struct ContextResponse {
    pub story: StoryContext,
    pub tasks: Vec<TaskContext>,
    pub proposal: String,
    pub design: String,
    pub scenarios: Vec<ScenarioContext>,
    pub learnings: Vec<String>,
    pub verify: VerifyContext,
}

#[derive(Debug, Serialize)]
pub struct StoryContext {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct TaskContext {
    pub id: String,
    pub description: String,
    pub done: bool,
}

#[derive(Debug, Serialize)]
pub struct ScenarioContext {
    pub name: String,
    pub story_id: String,
    pub given: Vec<String>,
    pub when: String,
    pub then: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct VerifyContext {
    pub checks: Vec<String>,
    pub tests: String,
}

fn run_context() -> Result<()> {
    let session_id = session::get_session_id()?;
    let story_id = session::get_story_id()?;
    let session = session::load(&session_id)?;

    let adapter = spec::create_adapter(&session.change)?;
    let context = adapter.context(&story_id)?;

    let response = ContextResponse {
        story: StoryContext {
            id: context.story.id,
            title: context.story.title,
        },
        tasks: context
            .story
            .tasks
            .iter()
            .map(|t| TaskContext {
                id: t.id.clone(),
                description: t.description.clone(),
                done: t.done,
            })
            .collect(),
        proposal: context.proposal,
        design: context.design,
        scenarios: context
            .scenarios
            .iter()
            .map(|s| ScenarioContext {
                name: s.name.clone(),
                story_id: s.story_id.clone(),
                given: s.given.clone(),
                when: s.when.clone(),
                then: s.then.clone(),
            })
            .collect(),
        learnings: session.learnings,
        verify: VerifyContext {
            checks: context.verify.checks,
            tests: context.verify.tests,
        },
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

// ============================================================================
// Task Commands
// ============================================================================

use cli::TaskCommand;

/// Response from the task done command.
#[derive(Debug, Serialize)]
pub struct TaskDoneResponse {
    pub success: bool,
    pub task_id: String,
    pub remaining: Vec<String>,
    pub story_complete: bool,
}

fn run_task(command: TaskCommand) -> Result<()> {
    match command {
        TaskCommand::Done(args) => run_task_done(&args.task_id),
    }
}

fn run_task_done(task_id: &str) -> Result<()> {
    let session_id = session::get_session_id()?;
    let story_id = session::get_story_id()?;
    let session = session::load(&session_id)?;

    // Validate the task belongs to the current story
    if !task_id.starts_with(&format!("{}.", story_id)) {
        return Err(Error::TaskNotFound(format!(
            "Task '{}' is not in the current story scope (Story {})",
            task_id, story_id
        )));
    }

    // Mark task complete via adapter (persists to tasks.md)
    let mut adapter = spec::create_adapter(&session.change)?;
    adapter.mark_done(task_id)?;

    // Get remaining tasks for this story by re-loading adapter
    let adapter = spec::create_adapter(&session.change)?;
    let stories = adapter.stories()?;
    let current_story = stories
        .iter()
        .find(|s| s.id == story_id)
        .ok_or_else(|| Error::StoryNotFound(story_id.clone()))?;

    let remaining: Vec<String> = current_story
        .tasks
        .iter()
        .filter(|t| !t.done)
        .map(|t| t.id.clone())
        .collect();

    let story_complete = remaining.is_empty();

    let response = TaskDoneResponse {
        success: true,
        task_id: task_id.to_string(),
        remaining,
        story_complete,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

// ============================================================================
// Status Command
// ============================================================================

/// Response from the status command.
#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub story: StoryStatus,
    pub change: ChangeStatus,
    pub story_complete: bool,
    pub change_complete: bool,
}

#[derive(Debug, Serialize)]
pub struct StoryStatus {
    pub id: String,
    pub tasks_done: usize,
    pub tasks_total: usize,
}

#[derive(Debug, Serialize)]
pub struct ChangeStatus {
    pub stories_done: usize,
    pub stories_total: usize,
}

fn run_status() -> Result<()> {
    let session_id = session::get_session_id()?;
    let session = session::load(&session_id)?;
    let story_id = std::env::var("RALPH_STORY").ok();

    let adapter = spec::create_adapter(&session.change)?;
    let stories = adapter.stories()?;

    // Count completed stories
    let stories_done = stories.iter().filter(|s| s.is_complete()).count();
    let stories_total = stories.len();

    // Get story status
    let (story_status, story_complete) = if let Some(ref sid) = story_id {
        if let Some(story) = stories.iter().find(|s| &s.id == sid) {
            let tasks_done = story.tasks.iter().filter(|t| t.done).count();
            let tasks_total = story.tasks.len();
            let complete = story.is_complete();
            (
                StoryStatus {
                    id: story.id.clone(),
                    tasks_done,
                    tasks_total,
                },
                complete,
            )
        } else {
            (
                StoryStatus {
                    id: sid.clone(),
                    tasks_done: 0,
                    tasks_total: 0,
                },
                false,
            )
        }
    } else {
        // Find first incomplete story
        if let Some(story) = stories.iter().find(|s| !s.is_complete()) {
            let tasks_done = story.tasks.iter().filter(|t| t.done).count();
            (
                StoryStatus {
                    id: story.id.clone(),
                    tasks_done,
                    tasks_total: story.tasks.len(),
                },
                false,
            )
        } else {
            (
                StoryStatus {
                    id: String::new(),
                    tasks_done: 0,
                    tasks_total: 0,
                },
                true,
            )
        }
    };

    let change_complete = stories_done == stories_total && stories_total > 0;

    let response = StatusResponse {
        story: story_status,
        change: ChangeStatus {
            stories_done,
            stories_total,
        },
        story_complete,
        change_complete,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

// ============================================================================
// Learn Command
// ============================================================================

use cli::LearnArgs;

/// Response from the learn command.
#[derive(Debug, Serialize)]
pub struct LearnResponse {
    pub success: bool,
    pub learning: String,
    pub total_learnings: usize,
}

fn run_learn(args: LearnArgs) -> Result<()> {
    let session_id = session::get_session_id()?;
    let mut session = session::load(&session_id)?;

    // Add learning to session
    session.learnings.push(args.description.clone());
    let total = session.learnings.len();

    // Save session
    session::save(&session)?;

    let response = LearnResponse {
        success: true,
        learning: args.description,
        total_learnings: total,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
