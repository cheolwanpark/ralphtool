//! Task completion management within the current story.
//!
//! Provides commands to mark tasks as complete and check status.
//! All operations are scoped to the current story (RALPH_STORY).

use std::fs::{self, File};
use std::io::Read;

use anyhow::{anyhow, Context, Result};
use fs2::FileExt;
use serde::Serialize;

use crate::agent::cli::TaskCommand;
use crate::agent::session::{get_session_id, get_story_id, load_session, save_session};
use crate::ralph::openspec::OpenSpecAdapter;
use crate::ralph::TaskSource;

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

    // Update tasks.md atomically
    let tasks_path = get_tasks_path(&session.change_name)?;
    update_task_in_file(&tasks_path, task_id)?;

    // Track completion in session
    if !session.completed_tasks.contains(&task_id.to_string()) {
        session.completed_tasks.push(task_id.to_string());
        save_session(&session)?;
    }

    // Get remaining tasks for this story
    let adapter = OpenSpecAdapter::new(&session.change_name)?;
    let stories = adapter.list_tasks()?;
    let current_story = stories
        .iter()
        .find(|s| s.id == story_id)
        .ok_or_else(|| anyhow!("Story '{}' not found", story_id))?;

    // Re-read tasks to get updated completion status
    let tasks_content = fs::read_to_string(&tasks_path)?;
    let remaining: Vec<String> = current_story
        .tasks
        .iter()
        .filter(|t| {
            // Check if task is incomplete in the file
            let checkbox_unchecked = format!("- [ ] {}", t.id);
            tasks_content.contains(&checkbox_unchecked)
        })
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
    let stories = adapter.list_tasks()?;

    // Calculate change-level status
    let mut stories_done = 0;
    let stories_total = stories.len();

    for story in &stories {
        let all_complete = story.tasks.iter().all(|t| t.complete);
        if all_complete && !story.tasks.is_empty() {
            stories_done += 1;
        }
    }

    // Calculate story-level status if a story is set
    let (story_status, story_complete) = if let Some(ref sid) = story_id {
        let current_story = stories.iter().find(|s| &s.id == sid);
        if let Some(story) = current_story {
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

/// Gets the path to tasks.md for a change.
fn get_tasks_path(change_name: &str) -> Result<std::path::PathBuf> {
    let cwd = std::env::current_dir()?;
    let path = cwd
        .join("openspec")
        .join("changes")
        .join(change_name)
        .join("tasks.md");
    if !path.exists() {
        return Err(anyhow!("tasks.md not found at: {}", path.display()));
    }
    Ok(path)
}

/// Updates a task in tasks.md from unchecked to checked.
///
/// Uses file locking to ensure atomic updates.
fn update_task_in_file(path: &std::path::PathBuf, task_id: &str) -> Result<()> {
    // Open file for read+write with locking
    let file = File::options()
        .read(true)
        .write(true)
        .open(path)
        .with_context(|| format!("Failed to open {}", path.display()))?;

    // Acquire exclusive lock
    file.lock_exclusive()
        .with_context(|| "Failed to acquire file lock")?;

    // Read content
    let mut content = String::new();
    {
        let mut reader = std::io::BufReader::new(&file);
        reader.read_to_string(&mut content)?;
    }

    // Find and replace the task checkbox
    let unchecked = format!("- [ ] {}", task_id);
    let checked = format!("- [x] {}", task_id);

    if !content.contains(&unchecked) {
        // Task might already be complete (idempotent)
        if content.contains(&checked) {
            // Already complete, success
            file.unlock()?;
            return Ok(());
        }
        file.unlock()?;
        return Err(anyhow!("Task '{}' not found in tasks.md", task_id));
    }

    let new_content = content.replace(&unchecked, &checked);

    // Write back (truncate and rewrite)
    // Need to use std::fs::write after unlocking for simplicity
    file.unlock()?;
    fs::write(path, new_content)?;

    Ok(())
}
