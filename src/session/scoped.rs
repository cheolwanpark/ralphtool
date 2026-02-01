//! Scoped session for RAII-style session lifecycle management.
//!
//! `ScopedSession` provides an ergonomic wrapper around session management that:
//! - Initializes a session and acquires an exclusive lock on construction
//! - Provides environment variables for subprocess spawning
//! - Cleans up session state on drop (releases lock, removes session file)

use std::collections::HashMap;
use std::fs::{self, File};

use fs2::FileExt;
use uuid::Uuid;

use super::state::{lock_file_path, session_file_path, Session};
use crate::error::{Error, Result};
use crate::spec;

/// RAII wrapper for session lifecycle management.
///
/// Creates a session on construction and cleans up on drop.
/// Provides helpers for configuring subprocess environment.
pub struct ScopedSession {
    /// Unique session identifier.
    session_id: String,

    /// Name of the change being worked on.
    change: String,

    /// Current story being worked on.
    story_id: Option<String>,

    /// Lock file handle (kept open to maintain lock).
    #[allow(dead_code)]
    lock_file: File,

    /// Whether the session has been explicitly flushed.
    flushed: bool,
}

impl ScopedSession {
    /// Initialize a new scoped session for the given change.
    ///
    /// Creates a session, acquires an exclusive lock, and saves session state.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The change does not exist
    /// - The change is already locked by another session
    pub fn init(change: &str) -> Result<Self> {
        // Verify the change exists by loading the adapter
        let _adapter = spec::create_adapter(change)?;

        // Acquire exclusive lock on the change
        let lock_path = lock_file_path(change);
        if let Some(parent) = lock_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let lock_file = File::create(&lock_path)?;

        // Try to acquire exclusive lock (non-blocking)
        lock_file
            .try_lock_exclusive()
            .map_err(|_| Error::ChangeLocked(change.to_string()))?;

        // Create session state
        let session_id = Uuid::new_v4().to_string();

        let session = Session {
            id: session_id.clone(),
            change: change.to_string(),
            story_id: None,
            learnings: Vec::new(),
        };

        // Save session state
        super::state::save(&session)?;

        Ok(Self {
            session_id,
            change: change.to_string(),
            story_id: None,
            lock_file,
            flushed: false,
        })
    }

    /// Returns session environment variables for subprocess configuration.
    ///
    /// The returned map contains:
    /// - `RALPH_SESSION`: The session ID
    /// - `RALPH_STORY`: The current story ID (if set)
    pub fn env(&self) -> HashMap<String, String> {
        let mut env = HashMap::new();
        env.insert("RALPH_SESSION".to_string(), self.session_id.clone());

        if let Some(ref story_id) = self.story_id {
            env.insert("RALPH_STORY".to_string(), story_id.clone());
        }

        env
    }

    /// Advances to the next incomplete story.
    ///
    /// Returns `Ok(Some(story_id))` if there is an incomplete story,
    /// or `Ok(None)` if all stories are complete.
    pub fn next_story(&mut self) -> Result<Option<String>> {
        let adapter = spec::create_adapter(&self.change)?;
        let stories = adapter.stories()?;

        // Find the next incomplete story
        let next_story = stories.iter().find(|story| !story.is_complete());

        match next_story {
            Some(story) => {
                self.story_id = Some(story.id.clone());

                // Update persisted session state
                let mut session = super::state::load(&self.session_id)?;
                session.story_id = self.story_id.clone();
                super::state::save(&session)?;

                Ok(Some(story.id.clone()))
            }
            None => {
                self.story_id = None;

                // Update persisted session state
                let mut session = super::state::load(&self.session_id)?;
                session.story_id = None;
                super::state::save(&session)?;

                Ok(None)
            }
        }
    }

    /// Flushes learnings and cleans up the session.
    ///
    /// This method consumes the session, writing any accumulated learnings
    /// to the spec adapter and releasing resources.
    ///
    /// # Arguments
    ///
    /// * `learnings` - Learnings to persist before cleanup
    pub fn flush(mut self, learnings: &[String]) -> Result<()> {
        // Load adapter for writing learnings
        let mut adapter = spec::create_adapter(&self.change)?;

        // Write learnings via adapter
        if !learnings.is_empty() {
            adapter.append_learnings(learnings)?;
        }

        // Mark as flushed so Drop doesn't do cleanup again
        self.flushed = true;

        // Cleanup is handled in drop
        self.cleanup();

        Ok(())
    }

    /// Internal cleanup: release lock and remove session file.
    fn cleanup(&self) {
        // Release lock by removing lock file
        let lock_path = lock_file_path(&self.change);
        if lock_path.exists() {
            fs::remove_file(&lock_path).ok(); // Ignore errors on cleanup
        }

        // Remove session file
        let session_path = session_file_path(&self.session_id);
        if session_path.exists() {
            fs::remove_file(&session_path).ok(); // Ignore errors on cleanup
        }
    }
}

impl Drop for ScopedSession {
    fn drop(&mut self) {
        // Only cleanup if not explicitly flushed
        if !self.flushed {
            self.cleanup();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    /// Create a ScopedSession for testing without requiring a real change.
    fn create_test_session(story_id: Option<String>) -> ScopedSession {
        // Create a temporary lock file to satisfy the struct requirements
        let temp_file = NamedTempFile::new().unwrap();
        let lock_file = temp_file.reopen().unwrap();
        std::mem::forget(temp_file); // Keep the file alive

        ScopedSession {
            session_id: "test-session-123".to_string(),
            change: "test-change".to_string(),
            story_id,
            lock_file,
            flushed: true, // Prevent cleanup on drop for tests
        }
    }

    // =========================================================================
    // Task 3.1: Unit tests for env() output
    // =========================================================================

    #[test]
    fn env_returns_session_id() {
        let session = create_test_session(None);

        let env = session.env();

        assert_eq!(
            env.get("RALPH_SESSION"),
            Some(&"test-session-123".to_string())
        );
    }

    #[test]
    fn env_without_story_has_no_ralph_story() {
        let session = create_test_session(None);

        let env = session.env();

        assert!(!env.contains_key("RALPH_STORY"));
    }

    #[test]
    fn env_with_story_includes_ralph_story() {
        let session = create_test_session(Some("story-1".to_string()));

        let env = session.env();

        assert_eq!(env.get("RALPH_STORY"), Some(&"story-1".to_string()));
    }

    #[test]
    fn env_returns_both_vars_when_story_set() {
        let session = create_test_session(Some("story-42".to_string()));

        let env = session.env();

        assert_eq!(env.len(), 2);
        assert!(env.contains_key("RALPH_SESSION"));
        assert!(env.contains_key("RALPH_STORY"));
    }
}
