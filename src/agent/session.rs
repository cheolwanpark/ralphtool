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
use crate::spec::openspec::OpenSpecAdapter;

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

    /// Patterns accumulated during this session (not yet written to design.md).
    #[serde(default)]
    pub patterns: Vec<SessionPattern>,

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

/// A reusable pattern discovered in the codebase.
///
/// Patterns are buffered in session state and flushed to design.md
/// when the session ends.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPattern {
    /// Name identifying the pattern.
    pub name: String,

    /// Description of the pattern and how to use it.
    pub description: String,

    /// ISO 8601 timestamp when pattern was recorded.
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
    pub patterns_written: usize,
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
        patterns: Vec::new(),
        completed_tasks: Vec::new(),
    };

    // Save session state
    save_session(&state)?;

    // Build story info from adapter
    use crate::spec::TaskSource;
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

    use crate::spec::TaskSource;
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
    use crate::spec::{Learning, Pattern, SpecWriter};

    let session_id = get_session_id()?;
    let state = load_session(&session_id)?;

    let learnings_count = state.learnings.len();
    let patterns_count = state.patterns.len();

    // Load adapter for writing
    let mut adapter = OpenSpecAdapter::new(&state.change_name)?;

    // Write learnings via adapter
    if !state.learnings.is_empty() {
        let learnings: Vec<Learning> = state
            .learnings
            .iter()
            .map(|l| Learning {
                description: l.description.clone(),
                task_id: l.task_id.clone(),
                story_id: l.story_id.clone(),
            })
            .collect();
        adapter.write_learnings(&learnings)?;
    }

    // Write patterns via adapter
    if !state.patterns.is_empty() {
        let patterns: Vec<Pattern> = state
            .patterns
            .iter()
            .map(|p| Pattern {
                name: p.name.clone(),
                description: p.description.clone(),
            })
            .collect();
        adapter.write_patterns(&patterns)?;
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
        patterns_written: patterns_count,
    };

    println!("{}", serde_json::to_string_pretty(&response)?);
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
