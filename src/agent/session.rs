//! Session lifecycle management for the Ralph Loop orchestrator.
//!
//! Sessions provide isolated state for each Ralph Loop run:
//! - Created by orchestrator via `session init`
//! - Contains current story scope and accumulated learnings
//! - Persisted to temp directory (`temp_dir()/ralph/sessions/<id>.json`)
//! - Cleaned up via `session flush` at end of run

use std::fs::{self, File};
use std::path::PathBuf;

use fs2::FileExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::agent::cli::{SessionCommand, SessionInitArgs};
use crate::error::{Error, Result};
use crate::spec;

/// Session state stored in temp directory.
///
/// File format: `temp_dir()/ralph/sessions/<session_id>.json`
///
/// Contains all state accumulated during a Ralph Loop run that needs
/// to persist across agent spawns but shouldn't be committed until flush.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique identifier for this session (UUID v4).
    pub id: String,

    /// Name of the change being worked on.
    pub change: String,

    /// Current story being worked on (set by `session next-story`).
    pub story_id: Option<String>,

    /// Learnings accumulated during this session (not yet written to design.md).
    pub learnings: Vec<String>,
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
    let adapter = spec::create_adapter(change_name)?;

    // Acquire exclusive lock on the change
    let lock_path = lock_file_path(change_name);
    if let Some(parent) = lock_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let lock_file = File::create(&lock_path)?;

    // Try to acquire exclusive lock (non-blocking)
    lock_file
        .try_lock_exclusive()
        .map_err(|_| Error::ChangeLocked(change_name.to_string()))?;

    // Create session state
    let session_id = Uuid::new_v4().to_string();

    let session = Session {
        id: session_id.clone(),
        change: change_name.to_string(),
        story_id: None,
        learnings: Vec::new(),
    };

    // Save session state
    save(&session)?;

    // Build story info from adapter
    let stories = adapter.stories()?;
    let story_infos: Vec<StoryInfo> = stories
        .iter()
        .map(|s| {
            let tasks_done = s.tasks.iter().filter(|t| t.done).count();
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
    let mut session = load(&session_id)?;

    // Load the adapter to get story information
    let adapter = spec::create_adapter(&session.change)?;
    let stories = adapter.stories()?;

    // Find the next incomplete story
    let next_story = stories.iter().find(|story| !story.is_complete());

    let response = match next_story {
        Some(story) => {
            // Update session state with current story
            session.story_id = Some(story.id.clone());
            save(&session)?;

            NextStoryResponse {
                complete: false,
                story_id: Some(story.id.clone()),
                story_title: Some(story.title.clone()),
            }
        }
        None => {
            session.story_id = None;
            save(&session)?;

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
    let session = load(&session_id)?;

    let learnings_count = session.learnings.len();

    // Load adapter for writing
    let mut adapter = spec::create_adapter(&session.change)?;

    // Write learnings via adapter
    if !session.learnings.is_empty() {
        adapter.append_learnings(&session.learnings)?;
    }

    // Release lock by removing lock file
    let lock_path = lock_file_path(&session.change);
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

/// Gets the session ID from RALPH_SESSION environment variable.
pub fn get_session_id() -> Result<String> {
    std::env::var("RALPH_SESSION").map_err(|_| Error::SessionRequired)
}

/// Gets the current story ID from RALPH_STORY environment variable.
pub fn get_story_id() -> Result<String> {
    std::env::var("RALPH_STORY").map_err(|_| Error::StoryRequired)
}

/// Returns the path to the session state file.
pub fn session_file_path(session_id: &str) -> PathBuf {
    let temp_dir = std::env::temp_dir();
    temp_dir
        .join("ralph")
        .join("sessions")
        .join(format!("{}.json", session_id))
}

/// Returns the path to the lock file for a change.
pub fn lock_file_path(change_name: &str) -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_default();
    cwd.join(".ralph")
        .join("locks")
        .join(format!("{}.lock", change_name))
}

/// Loads session state from file.
pub fn load(session_id: &str) -> Result<Session> {
    let path = session_file_path(session_id);
    let content = fs::read_to_string(&path)?;
    let session: Session = serde_json::from_str(&content)?;
    Ok(session)
}

/// Saves session state to file.
pub fn save(session: &Session) -> Result<()> {
    let path = session_file_path(&session.id);

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(session)?;
    fs::write(&path, content)?;
    Ok(())
}
