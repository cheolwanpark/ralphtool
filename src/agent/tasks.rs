//! Task completion management within the current story.
//!
//! Provides commands to mark tasks as complete and check status.
//! All operations are scoped to the current story (RALPH_STORY).

use anyhow::{anyhow, Result};
use serde::Serialize;

use crate::agent::cli::TaskCommand;
use crate::agent::session::{get_session_id, get_story_id, load_session, save_session};
use crate::spec::openspec::OpenSpecAdapter;
use crate::spec::{ContextProvider, TaskSource};

/// Response from the task done command.
#[derive(Debug, Serialize)]
pub struct TaskDoneResponse {
    pub success: bool,
    pub task_id: String,
    pub remaining: Vec<String>,
    pub story_complete: bool,
}

/// Response from the status command.
#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub story: StoryStatus,
    pub change: ChangeStatus,
    pub story_complete: bool,
    pub change_complete: bool,
}

/// Story-level status.
#[derive(Debug, Serialize)]
pub struct StoryStatus {
    pub id: String,
    pub tasks_done: usize,
    pub tasks_total: usize,
}

/// Change-level status.
#[derive(Debug, Serialize)]
pub struct ChangeStatus {
    pub stories_done: usize,
    pub stories_total: usize,
}

/// Runs a task subcommand.
pub fn run(command: TaskCommand) -> Result<()> {
    match command {
        TaskCommand::Done(args) => run_done(&args.task_id),
    }
}

fn run_done(task_id: &str) -> Result<()> {
    let session_id = get_session_id()?;
    let story_id = get_story_id()?;
    let mut session = load_session(&session_id)?;

    // Validate the task belongs to the current story
    if !task_id.starts_with(&format!("{}.", story_id)) {
        return Err(anyhow!(
            "Task '{}' is not in the current story scope (Story {}).\n\
             You can only mark tasks from the current story as complete.",
            task_id,
            story_id
        ));
    }

    // Mark task complete via adapter (persists to tasks.md)
    let mut adapter = OpenSpecAdapter::new(&session.change_name)?;
    adapter.mark_complete(task_id)?;

    // Track completion in session
    if !session.completed_tasks.contains(&task_id.to_string()) {
        session.completed_tasks.push(task_id.to_string());
        save_session(&session)?;
    }

    // Get remaining tasks for this story by re-loading adapter
    let adapter = OpenSpecAdapter::new(&session.change_name)?;
    let stories = adapter.list_tasks()?;
    let current_story = stories
        .iter()
        .find(|s| s.id == story_id)
        .ok_or_else(|| anyhow!("Story '{}' not found", story_id))?;

    let remaining: Vec<String> = current_story
        .tasks
        .iter()
        .filter(|t| !t.complete)
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

/// Runs the status command.
pub fn run_status() -> Result<()> {
    let session_id = get_session_id()?;
    let session = load_session(&session_id)?;
    let story_id = std::env::var("RALPH_STORY").ok();

    let adapter = OpenSpecAdapter::new(&session.change_name)?;

    // Get status from adapter
    let work_status = adapter.get_status()?;

    // Build story status
    let (story_status, story_complete) = if let Some(ref sid) = story_id {
        // If we have a story ID, find its specific status
        let stories = adapter.list_tasks()?;
        if let Some(story) = stories.iter().find(|s| &s.id == sid) {
            let tasks_done = story.tasks.iter().filter(|t| t.complete).count();
            let tasks_total = story.tasks.len();
            let complete = tasks_done == tasks_total && tasks_total > 0;
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
    } else if !work_status.story_id.is_empty() {
        // Use status from adapter
        let story_complete =
            work_status.story_tasks_done == work_status.story_tasks_total && work_status.story_tasks_total > 0;
        (
            StoryStatus {
                id: work_status.story_id,
                tasks_done: work_status.story_tasks_done,
                tasks_total: work_status.story_tasks_total,
            },
            story_complete,
        )
    } else {
        (
            StoryStatus {
                id: String::new(),
                tasks_done: 0,
                tasks_total: 0,
            },
            false,
        )
    };

    let change_complete = work_status.change_stories_done == work_status.change_stories_total
        && work_status.change_stories_total > 0;

    let response = StatusResponse {
        story: story_status,
        change: ChangeStatus {
            stories_done: work_status.change_stories_done,
            stories_total: work_status.change_stories_total,
        },
        story_complete,
        change_complete,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}
