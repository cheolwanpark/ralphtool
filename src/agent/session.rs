//! Session lifecycle management for the Ralph Loop orchestrator.
//!
//! Sessions provide isolated state for each Ralph Loop run:
//! - Created by orchestrator via `session init`
//! - Contains current story scope and accumulated learnings
//! - Persisted to temp directory (`temp_dir()/ralph/sessions/<id>.json`)
//! - Cleaned up via `session flush` at end of run

use std::fs::{self, File};

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::agent::cli::{SessionCommand, SessionInitArgs};
use crate::ralph::openspec::OpenSpecAdapter;

/// Session state stored in temp directory.
///
/// File format: `temp_dir()/ralph/sessions/<session_id>.json`
///
/// Contains all state accumulated during a Ralph Loop run that needs
/// to persist across agent spawns but shouldn't be committed until flush.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    /// Unique identifier for this session (UUID v4).
    pub session_id: String,

    /// Name of the change being worked on.
    pub change_name: String,

    /// Current story being worked on (set by `session next-story`).
    pub current_story_id: Option<String>,

    /// ISO 8601 timestamp when session was created.
    pub created_at: String,

    /// Learnings accumulated during this session (not yet written to design.md).
    pub learnings: Vec<SessionLearning>,

    /// Task IDs completed during this session.
    pub completed_tasks: Vec<String>,
}

/// A learning recorded during the session.
///
/// Learnings are buffered in session state and flushed to design.md
/// when the session ends.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLearning {
    /// Description of what was learned.
    pub description: String,

    /// Optional task ID this learning relates to.
    pub task_id: Option<String>,

    /// Story ID when this learning was recorded.
    pub story_id: Option<String>,

    /// ISO 8601 timestamp when learning was recorded.
    pub timestamp: String,
}

/// Response from `session init` command.
#[derive(Debug, Serialize)]
pub struct InitResponse {
    pub session_id: String,
    pub change_name: String,
    pub stories: Vec<StoryInfo>,
}

/// Story information returned in init response.
#[derive(Debug, Serialize)]
pub struct StoryInfo {
    pub id: String,
    pub title: String,
    pub tasks_total: usize,
    pub tasks_done: usize,
}

/// Response from `session next-story` command.
#[derive(Debug, Serialize)]
pub struct NextStoryResponse {
    /// True if all stories are complete.
    pub complete: bool,

    /// Story ID if one is available.
    pub story_id: Option<String>,

    /// Story title if one is available.
    pub story_title: Option<String>,
}

/// Response from `session flush` command.
#[derive(Debug, Serialize)]
pub struct FlushResponse {
    pub success: bool,
    pub learnings_written: usize,
}

/// Runs a session subcommand.
pub fn run(command: SessionCommand) -> Result<()> {
    match command {
        SessionCommand::Init(args) => run_init(args),
        SessionCommand::NextStory => run_next_story(),
        SessionCommand::Flush => run_flush(),
    }
}

fn run_init(args: SessionInitArgs) -> Result<()> {
    let change_name = &args.change;

    // Verify the change exists by loading the adapter
    let adapter = OpenSpecAdapter::new(change_name)
        .with_context(|| format!("Change '{}' not found", change_name))?;

    // Acquire exclusive lock on the change
    let lock_path = lock_file_path(change_name);
    if let Some(parent) = lock_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let lock_file = File::create(&lock_path)
        .with_context(|| format!("Failed to create lock file: {}", lock_path.display()))?;

    // Try to acquire exclusive lock (non-blocking)
    lock_file.try_lock_exclusive().map_err(|_| {
        anyhow!(
            "Change '{}' is locked by another session.\n\
             Another orchestrator may be running. Wait for it to complete or manually remove the lock file at:\n\
             {}",
            change_name,
            lock_path.display()
        )
    })?;

    // Create session state
    let session_id = Uuid::new_v4().to_string();
    let now: DateTime<Utc> = Utc::now();

    let state = SessionState {
        session_id: session_id.clone(),
        change_name: change_name.to_string(),
        current_story_id: None,
        created_at: now.to_rfc3339(),
        learnings: Vec::new(),
        completed_tasks: Vec::new(),
    };

    // Save session state
    save_session(&state)?;

    // Build story info from adapter
    use crate::ralph::TaskSource;
    let stories = adapter.list_tasks()?;
    let story_infos: Vec<StoryInfo> = stories
        .iter()
        .map(|s| {
            let tasks_done = s.tasks.iter().filter(|t| t.complete).count();
            StoryInfo {
                id: s.id.clone(),
                title: s.title.clone(),
                tasks_total: s.tasks.len(),
                tasks_done,
            }
        })
        .collect();

    // Output response
    let response = InitResponse {
        session_id,
        change_name: change_name.to_string(),
        stories: story_infos,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

fn run_next_story() -> Result<()> {
    let session_id = get_session_id()?;
    let mut state = load_session(&session_id)?;

    // Load the adapter to get story information
    let adapter = OpenSpecAdapter::new(&state.change_name)?;

    use crate::ralph::TaskSource;
    let stories = adapter.list_tasks()?;

    // Find the next incomplete story
    let next_story = stories.iter().find(|story| {
        // A story is incomplete if any of its tasks are incomplete
        story.tasks.iter().any(|task| !task.complete)
    });

    let response = match next_story {
        Some(story) => {
            // Update session state with current story
            state.current_story_id = Some(story.id.clone());
            save_session(&state)?;

            NextStoryResponse {
                complete: false,
                story_id: Some(story.id.clone()),
                story_title: Some(story.title.clone()),
            }
        }
        None => {
            state.current_story_id = None;
            save_session(&state)?;

            NextStoryResponse {
                complete: true,
                story_id: None,
                story_title: None,
            }
        }
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

fn run_flush() -> Result<()> {
    let session_id = get_session_id()?;
    let state = load_session(&session_id)?;

    let learnings_count = state.learnings.len();

    // Write learnings to design.md if there are any
    if !state.learnings.is_empty() {
        write_learnings_to_design(&state)?;
    }

    // Release lock by removing lock file
    let lock_path = lock_file_path(&state.change_name);
    if lock_path.exists() {
        fs::remove_file(&lock_path).ok(); // Ignore errors on cleanup
    }

    // Remove session file
    let session_path = session_file_path(&session_id);
    if session_path.exists() {
        fs::remove_file(&session_path).ok(); // Ignore errors on cleanup
    }

    let response = FlushResponse {
        success: true,
        learnings_written: learnings_count,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
    Ok(())
}

/// Writes accumulated learnings to design.md.
fn write_learnings_to_design(state: &SessionState) -> Result<()> {
    // Find design.md in the change directory
    let cwd = std::env::current_dir()?;
    let design_path = cwd
        .join("openspec")
        .join("changes")
        .join(&state.change_name)
        .join("design.md");

    if !design_path.exists() {
        return Err(anyhow!("design.md not found at: {}", design_path.display()));
    }

    let mut content = fs::read_to_string(&design_path)?;

    // Check if Learnings section exists
    let learnings_section = "\n## Learnings\n";
    if !content.contains("## Learnings") {
        content.push_str(learnings_section);
    }

    // Group learnings by date and story
    use std::collections::BTreeMap;
    let mut by_date_story: BTreeMap<(String, String), Vec<&SessionLearning>> = BTreeMap::new();

    for learning in &state.learnings {
        // Parse date from timestamp
        let date = learning.timestamp.split('T').next().unwrap_or("Unknown");
        let story_id = learning.story_id.clone().unwrap_or_else(|| "General".to_string());
        let key = (date.to_string(), story_id);
        by_date_story.entry(key).or_default().push(learning);
    }

    // Format learnings
    let mut learnings_text = String::new();
    for ((date, story_id), learnings) in by_date_story {
        learnings_text.push_str(&format!("\n### {} - Story {}\n", date, story_id));
        for learning in learnings {
            let task_ref = learning
                .task_id
                .as_ref()
                .map(|id| format!(" (Task {})", id))
                .unwrap_or_default();
            learnings_text.push_str(&format!("- {}{}\n", learning.description, task_ref));
        }
    }

    // Append learnings
    content.push_str(&learnings_text);

    fs::write(&design_path, content)?;
    Ok(())
}

/// Gets the session ID from RALPH_SESSION environment variable.
pub fn get_session_id() -> Result<String> {
    std::env::var("RALPH_SESSION").map_err(|_| {
        anyhow!(
            "RALPH_SESSION environment variable not set.\n\
             This command requires a valid session.\n\
             Use the orchestrator to manage sessions properly."
        )
    })
}

/// Gets the current story ID from RALPH_STORY environment variable.
pub fn get_story_id() -> Result<String> {
    std::env::var("RALPH_STORY").map_err(|_| {
        anyhow!(
            "RALPH_STORY environment variable not set.\n\
             This command requires a story scope.\n\
             Use `session next-story` to set the current story."
        )
    })
}

/// Returns the path to the session state file.
pub fn session_file_path(session_id: &str) -> std::path::PathBuf {
    let temp_dir = std::env::temp_dir();
    temp_dir
        .join("ralph")
        .join("sessions")
        .join(format!("{}.json", session_id))
}

/// Returns the path to the lock file for a change.
pub fn lock_file_path(change_name: &str) -> std::path::PathBuf {
    let cwd = std::env::current_dir().unwrap_or_default();
    cwd.join(".ralph")
        .join("locks")
        .join(format!("{}.lock", change_name))
}

/// Loads session state from file.
pub fn load_session(session_id: &str) -> Result<SessionState> {
    let path = session_file_path(session_id);
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read session file: {}", path.display()))?;
    let state: SessionState = serde_json::from_str(&content)
        .with_context(|| "Failed to parse session state")?;
    Ok(state)
}

/// Saves session state to file.
pub fn save_session(state: &SessionState) -> Result<()> {
    let path = session_file_path(&state.session_id);

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(state)?;
    std::fs::write(&path, content)?;
    Ok(())
}
