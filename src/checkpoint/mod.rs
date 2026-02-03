//! Checkpoint module for git stash-based state preservation.
//!
//! Provides checkpoint/revert functionality to preserve working directory state
//! before agent execution and restore it on failure.
//!
//! All operations are async-safe, using `async_cmd` to avoid blocking tokio
//! worker threads.

use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use crate::async_cmd;
use crate::error::{Error, Result};

/// Checkpoint manager using git stash for state preservation.
///
/// Creates named stashes with the pattern `ralph:{change_name}:{story_id}`
/// to allow targeted revert and cleanup operations.
#[derive(Debug, Clone)]
pub struct Checkpoint {
    /// The name of the change being worked on.
    change_name: String,
    /// Optional working directory for git commands (for testing).
    work_dir: Option<PathBuf>,
    /// Timeout for git commands.
    timeout: Duration,
}

impl Checkpoint {
    /// Creates a new Checkpoint with a custom timeout.
    pub fn with_timeout(change_name: impl Into<String>, timeout: Duration) -> Self {
        Self {
            change_name: change_name.into(),
            work_dir: None,
            timeout,
        }
    }

    /// Creates a new Checkpoint with a specific working directory.
    /// Used primarily for testing.
    #[cfg(test)]
    pub fn with_work_dir(change_name: impl Into<String>, work_dir: PathBuf) -> Self {
        Self {
            change_name: change_name.into(),
            work_dir: Some(work_dir),
            timeout: async_cmd::DEFAULT_TIMEOUT,
        }
    }

    /// Returns the stash message for a given story ID.
    fn stash_message(&self, story_id: &str) -> String {
        format!("ralph:{}:{}", self.change_name, story_id)
    }

    /// Creates a Command with the appropriate working directory set.
    /// Used for sync fallback in tests with work_dir.
    fn git_command(&self) -> Command {
        let mut cmd = Command::new("git");
        if let Some(ref work_dir) = self.work_dir {
            cmd.current_dir(work_dir);
        }
        cmd
    }

    /// Saves the current working directory state as a checkpoint.
    ///
    /// Uses `git stash push -u -m "ralph:{change}:{story}"` to capture both
    /// tracked and untracked files. Does not block tokio worker threads.
    pub async fn save(&self, story_id: &str) -> Result<()> {
        let message = self.stash_message(story_id);

        let args = vec!["stash", "push", "-u", "-m", &message];

        let output = self.run_git(&args).await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: "git stash push".to_string(),
                stderr,
            });
        }

        Ok(())
    }

    /// Finds the stash index for a given story ID.
    ///
    /// Parses `git stash list` output to find the stash with the matching
    /// message pattern `ralph:{change}:{story}`.
    ///
    /// Returns `None` if no matching stash is found.
    pub async fn find_stash(&self, story_id: &str) -> Result<Option<usize>> {
        let message = self.stash_message(story_id);

        let output = self.run_git(&["stash", "list"]).await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: "git stash list".to_string(),
                stderr,
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse stash list output format: "stash@{n}: On branch: message"
        for line in stdout.lines() {
            if line.contains(&message) {
                // Extract the index from "stash@{n}"
                if let Some(start) = line.find("stash@{") {
                    if let Some(end) = line[start..].find('}') {
                        let index_str = &line[start + 7..start + end];
                        if let Ok(index) = index_str.parse::<usize>() {
                            return Ok(Some(index));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// Reverts to the checkpoint for a given story ID.
    ///
    /// Uses `git stash apply stash@{n}` to restore state. The stash is
    /// preserved for potential further retries.
    ///
    /// Before applying, cleans the working directory to ensure a clean state.
    pub async fn revert(&self, story_id: &str) -> Result<()> {
        let index = self.find_stash(story_id).await?.ok_or_else(|| Error::Command {
            cmd: "git stash apply".to_string(),
            stderr: format!("No checkpoint found for story: {}", story_id),
        })?;

        // First, clean the working directory to discard agent changes
        // Reset tracked files
        let reset_output = self.run_git(&["checkout", "--", "."]).await?;

        if !reset_output.status.success() {
            let stderr = String::from_utf8_lossy(&reset_output.stderr).to_string();
            return Err(Error::Command {
                cmd: "git checkout".to_string(),
                stderr,
            });
        }

        // Remove untracked files
        let clean_output = self.run_git(&["clean", "-fd"]).await?;

        if !clean_output.status.success() {
            let stderr = String::from_utf8_lossy(&clean_output.stderr).to_string();
            return Err(Error::Command {
                cmd: "git clean".to_string(),
                stderr,
            });
        }

        // Apply the stash
        let stash_ref = format!("stash@{{{}}}", index);
        let output = self.run_git(&["stash", "apply", &stash_ref]).await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: format!("git stash apply {}", stash_ref),
                stderr,
            });
        }

        Ok(())
    }

    /// Drops the checkpoint for a given story ID.
    ///
    /// Uses `git stash drop stash@{n}` to remove the checkpoint after
    /// successful completion.
    pub async fn drop(&self, story_id: &str) -> Result<()> {
        let index = self.find_stash(story_id).await?.ok_or_else(|| Error::Command {
            cmd: "git stash drop".to_string(),
            stderr: format!("No checkpoint found for story: {}", story_id),
        })?;

        let stash_ref = format!("stash@{{{}}}", index);
        let output = self.run_git(&["stash", "drop", &stash_ref]).await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: format!("git stash drop {}", stash_ref),
                stderr,
            });
        }

        Ok(())
    }

    /// Cleans up all stashes matching the change name pattern.
    ///
    /// Drops all stashes with messages matching `ralph:{change_name}:*`.
    /// This should be called when the orchestrator finishes (success or failure).
    pub async fn cleanup(&self) -> Result<()> {
        let pattern = format!("ralph:{}:", self.change_name);

        // Keep dropping stashes until none match
        // We need to re-query each time because indices shift after each drop
        loop {
            let output = self.run_git(&["stash", "list"]).await?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                return Err(Error::Command {
                    cmd: "git stash list".to_string(),
                    stderr,
                });
            }

            let stdout = String::from_utf8_lossy(&output.stdout);

            // Find the first matching stash
            let mut found_index: Option<usize> = None;
            for line in stdout.lines() {
                if line.contains(&pattern) {
                    if let Some(start) = line.find("stash@{") {
                        if let Some(end) = line[start..].find('}') {
                            let index_str = &line[start + 7..start + end];
                            if let Ok(index) = index_str.parse::<usize>() {
                                found_index = Some(index);
                                break;
                            }
                        }
                    }
                }
            }

            match found_index {
                Some(index) => {
                    let stash_ref = format!("stash@{{{}}}", index);
                    let drop_output = self.run_git(&["stash", "drop", &stash_ref]).await?;

                    if !drop_output.status.success() {
                        let stderr = String::from_utf8_lossy(&drop_output.stderr).to_string();
                        return Err(Error::Command {
                            cmd: format!("git stash drop {}", stash_ref),
                            stderr,
                        });
                    }
                }
                None => break, // No more matching stashes
            }
        }

        Ok(())
    }

    /// Helper to run a git command asynchronously.
    ///
    /// For testing (when work_dir is set), falls back to sync execution.
    /// In production (work_dir is None), uses async_cmd for non-blocking execution.
    async fn run_git(&self, args: &[&str]) -> Result<std::process::Output> {
        if self.work_dir.is_some() {
            // Fall back to sync for testing with work_dir
            let output = self.git_command().args(args).output()?;
            Ok(output)
        } else {
            // Use async command execution
            async_cmd::run_unchecked_with_timeout("git", args, self.timeout).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn stash_message_format() {
        let checkpoint = Checkpoint::with_timeout("my-feature", async_cmd::DEFAULT_TIMEOUT);
        assert_eq!(
            checkpoint.stash_message("story-1"),
            "ralph:my-feature:story-1"
        );
    }

    #[test]
    fn checkpoint_with_timeout_sets_change_name() {
        let checkpoint = Checkpoint::with_timeout("test-change", async_cmd::DEFAULT_TIMEOUT);
        assert_eq!(checkpoint.change_name, "test-change");
    }

    /// Creates a temporary git repository for testing.
    /// Returns (TempDir, Checkpoint) - TempDir must stay in scope to keep the directory.
    fn setup_temp_repo_with_checkpoint(change_name: &str) -> (TempDir, Checkpoint) {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repo_path = temp_dir.path().to_path_buf();

        // Initialize git repo
        Command::new("git")
            .args(["init"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to init git repo");

        // Configure git user (required for commits)
        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to configure git email");

        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to configure git name");

        // Create an initial commit (required for stash to work)
        let test_file = repo_path.join("initial.txt");
        fs::write(&test_file, "initial content").expect("Failed to write initial file");

        Command::new("git")
            .args(["add", "."])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to git add");

        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to create initial commit");

        let checkpoint = Checkpoint::with_work_dir(change_name, repo_path);
        (temp_dir, checkpoint)
    }

    /// Gets the repo path from a checkpoint for file operations.
    fn repo_path(checkpoint: &Checkpoint) -> &PathBuf {
        checkpoint.work_dir.as_ref().expect("Checkpoint should have work_dir for tests")
    }

    #[tokio::test]
    async fn save_creates_stash_with_correct_message() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        // Create a modified file to stash
        let test_file = path.join("test.txt");
        fs::write(&test_file, "modified content").expect("Failed to write test file");

        checkpoint.save("story-1").await.expect("save should succeed");

        // Verify stash was created
        let output = Command::new("git")
            .args(["stash", "list"])
            .current_dir(path)
            .output()
            .expect("Failed to list stashes");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("ralph:my-change:story-1"),
            "Stash list should contain our checkpoint: {}",
            stdout
        );
    }

    #[tokio::test]
    async fn save_includes_untracked_files() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        // Create an untracked file
        let untracked_file = path.join("untracked.txt");
        fs::write(&untracked_file, "untracked content").expect("Failed to write untracked file");

        checkpoint.save("story-1").await.expect("save should succeed");

        // After stash, untracked file should be gone (stashed)
        assert!(
            !untracked_file.exists(),
            "Untracked file should be removed after stash"
        );

        // Verify we can get it back
        checkpoint.revert("story-1").await.expect("revert should succeed");

        assert!(
            untracked_file.exists(),
            "Untracked file should be restored after revert"
        );
    }

    #[tokio::test]
    async fn find_stash_returns_correct_index() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        // Create two stashes
        let file1 = path.join("file1.txt");
        fs::write(&file1, "content1").expect("Failed to write file1");

        checkpoint.save("story-1").await.expect("save story-1 should succeed");

        let file2 = path.join("file2.txt");
        fs::write(&file2, "content2").expect("Failed to write file2");

        checkpoint.save("story-2").await.expect("save story-2 should succeed");

        // story-2 should be at index 0 (most recent)
        // story-1 should be at index 1
        let index1 = checkpoint
            .find_stash("story-1")
            .await
            .expect("find_stash should succeed")
            .expect("story-1 should exist");
        assert_eq!(index1, 1, "story-1 should be at index 1");

        let index2 = checkpoint
            .find_stash("story-2")
            .await
            .expect("find_stash should succeed")
            .expect("story-2 should exist");
        assert_eq!(index2, 0, "story-2 should be at index 0");
    }

    #[tokio::test]
    async fn find_stash_returns_none_for_missing_stash() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("my-change");

        let index = checkpoint
            .find_stash("nonexistent")
            .await
            .expect("find_stash should succeed");
        assert!(index.is_none(), "Should return None for missing stash");
    }

    #[tokio::test]
    async fn revert_restores_stashed_state() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        // Create and stash a file
        let test_file = path.join("test.txt");
        fs::write(&test_file, "original content").expect("Failed to write test file");

        checkpoint.save("story-1").await.expect("save should succeed");

        // File should be gone after stash
        assert!(!test_file.exists(), "File should be removed after stash");

        // Revert should bring it back
        checkpoint.revert("story-1").await.expect("revert should succeed");

        assert!(test_file.exists(), "File should be restored after revert");
        let content = fs::read_to_string(&test_file).expect("Failed to read restored file");
        assert_eq!(content, "original content");
    }

    #[tokio::test]
    async fn revert_discards_agent_changes() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        // Create initial state
        let test_file = path.join("test.txt");
        fs::write(&test_file, "original content").expect("Failed to write test file");

        checkpoint.save("story-1").await.expect("save should succeed");

        // Simulate agent making changes (creating new file)
        let agent_file = path.join("agent_created.txt");
        fs::write(&agent_file, "agent output").expect("Failed to write agent file");

        // Revert should discard agent changes and restore original
        checkpoint.revert("story-1").await.expect("revert should succeed");

        assert!(
            !agent_file.exists(),
            "Agent-created file should be removed after revert"
        );
        assert!(
            test_file.exists(),
            "Original file should be restored after revert"
        );
    }

    #[tokio::test]
    async fn revert_fails_for_missing_stash() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("my-change");

        let result = checkpoint.revert("nonexistent").await;
        assert!(result.is_err(), "revert should fail for missing stash");
    }

    #[tokio::test]
    async fn drop_removes_stash() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        // Create and stash a file
        let test_file = path.join("test.txt");
        fs::write(&test_file, "content").expect("Failed to write test file");

        checkpoint.save("story-1").await.expect("save should succeed");

        // Verify stash exists
        let index_before = checkpoint
            .find_stash("story-1")
            .await
            .expect("find_stash should succeed");
        assert!(index_before.is_some(), "Stash should exist before drop");

        // Drop the stash
        checkpoint.drop("story-1").await.expect("drop should succeed");

        // Verify stash is gone
        let index_after = checkpoint
            .find_stash("story-1")
            .await
            .expect("find_stash should succeed");
        assert!(index_after.is_none(), "Stash should not exist after drop");
    }

    #[tokio::test]
    async fn drop_fails_for_missing_stash() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("my-change");

        let result = checkpoint.drop("nonexistent").await;
        assert!(result.is_err(), "drop should fail for missing stash");
    }

    #[tokio::test]
    async fn cleanup_removes_all_matching_stashes() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        // Create multiple stashes for the same change
        for i in 1..=3 {
            let file = path.join(format!("file{}.txt", i));
            fs::write(&file, format!("content{}", i)).expect("Failed to write file");

            checkpoint
                .save(&format!("story-{}", i))
                .await
                .expect("save should succeed");
        }

        // Verify all stashes exist
        for i in 1..=3 {
            let index = checkpoint
                .find_stash(&format!("story-{}", i))
                .await
                .expect("find_stash should succeed");
            assert!(index.is_some(), "story-{} should exist before cleanup", i);
        }

        // Cleanup
        checkpoint.cleanup().await.expect("cleanup should succeed");

        // Verify all stashes are gone
        for i in 1..=3 {
            let index = checkpoint
                .find_stash(&format!("story-{}", i))
                .await
                .expect("find_stash should succeed");
            assert!(
                index.is_none(),
                "story-{} should not exist after cleanup",
                i
            );
        }
    }

    #[tokio::test]
    async fn cleanup_only_removes_matching_change() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repo_path = temp_dir.path().to_path_buf();

        // Initialize git repo
        Command::new("git")
            .args(["init"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to init git repo");

        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to configure git email");

        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to configure git name");

        let test_file = repo_path.join("initial.txt");
        fs::write(&test_file, "initial content").expect("Failed to write initial file");

        Command::new("git")
            .args(["add", "."])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to git add");

        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to create initial commit");

        // Create two checkpoints for different changes, same repo
        let checkpoint1 = Checkpoint::with_work_dir("change-a", repo_path.clone());
        let checkpoint2 = Checkpoint::with_work_dir("change-b", repo_path.clone());

        // Create stash for change-a
        let file1 = repo_path.join("file1.txt");
        fs::write(&file1, "content1").expect("Failed to write file1");
        checkpoint1.save("story-1").await.expect("save should succeed");

        // Create stash for change-b
        let file2 = repo_path.join("file2.txt");
        fs::write(&file2, "content2").expect("Failed to write file2");
        checkpoint2.save("story-1").await.expect("save should succeed");

        // Cleanup only change-a
        checkpoint1.cleanup().await.expect("cleanup should succeed");

        // change-a stash should be gone
        let index_a = checkpoint1
            .find_stash("story-1")
            .await
            .expect("find_stash should succeed");
        assert!(
            index_a.is_none(),
            "change-a stash should not exist after cleanup"
        );

        // change-b stash should still exist
        let index_b = checkpoint2
            .find_stash("story-1")
            .await
            .expect("find_stash should succeed");
        assert!(
            index_b.is_some(),
            "change-b stash should still exist after cleanup of change-a"
        );
    }

    #[test]
    fn stash_naming_convention() {
        // Verify the naming convention matches the design
        let checkpoint = Checkpoint::with_timeout("my-feature", async_cmd::DEFAULT_TIMEOUT);
        assert_eq!(
            checkpoint.stash_message("story-1"),
            "ralph:my-feature:story-1"
        );

        // Verify pattern for cleanup
        let pattern = format!("ralph:{}:", checkpoint.change_name);
        assert_eq!(pattern, "ralph:my-feature:");
    }

    #[tokio::test]
    async fn revert_preserves_stash_for_retries() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        // Create and stash a file
        let test_file = path.join("test.txt");
        fs::write(&test_file, "content").expect("Failed to write test file");

        checkpoint.save("story-1").await.expect("save should succeed");

        // Revert (simulating first retry)
        checkpoint.revert("story-1").await.expect("revert should succeed");

        // Stash should still exist (for potential second retry)
        let index = checkpoint
            .find_stash("story-1")
            .await
            .expect("find_stash should succeed");
        assert!(
            index.is_some(),
            "Stash should still exist after revert (for retries)"
        );

        // Can revert again (simulating second retry)
        // First make some changes
        fs::write(&test_file, "modified by agent").expect("Failed to modify file");

        checkpoint
            .revert("story-1")
            .await
            .expect("second revert should succeed");

        // Stash should still exist
        let index = checkpoint
            .find_stash("story-1")
            .await
            .expect("find_stash should succeed");
        assert!(
            index.is_some(),
            "Stash should still exist after second revert"
        );
    }

    // ==================== Integration Tests ====================

    /// Integration test: Full checkpoint/revert/retry cycle
    ///
    /// This test simulates a complete orchestrator workflow:
    /// 1. Save checkpoint before agent spawn
    /// 2. Agent makes changes and fails (no COMPLETE signal)
    /// 3. Revert to checkpoint (retry attempt 1)
    /// 4. Agent makes changes and fails again
    /// 5. Revert to checkpoint (retry attempt 2)
    /// 6. Agent succeeds (outputs COMPLETE)
    /// 7. Drop checkpoint on success
    /// 8. Cleanup any remaining stashes
    #[tokio::test]
    async fn integration_full_checkpoint_revert_retry_cycle() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("test-feature");
        let path = repo_path(&checkpoint).clone();

        // Initial state: create a file that represents the "pre-story" state
        let initial_file = path.join("existing_code.rs");
        fs::write(&initial_file, "fn main() { println!(\"Hello\"); }").unwrap();

        // Commit the initial state so it's part of the repo
        Command::new("git")
            .args(["add", "."])
            .current_dir(&path)
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "Add existing code"])
            .current_dir(&path)
            .output()
            .unwrap();

        // Make a small change so there's something to checkpoint
        // (git stash requires changes to create a stash)
        let work_file = path.join("work_in_progress.txt");
        fs::write(&work_file, "starting work").unwrap();

        // ============ STEP 1: Save checkpoint before agent spawn ============
        checkpoint
            .save("story-1")
            .await
            .expect("save checkpoint should succeed");

        // Verify checkpoint exists
        let stash_index = checkpoint.find_stash("story-1").await.unwrap();
        assert!(stash_index.is_some(), "Checkpoint should exist after save");

        // ============ STEP 2: Agent attempt 1 - makes changes but fails ============
        fs::write(
            &initial_file,
            "fn main() { println!(\"Modified by agent\"); }",
        )
        .unwrap();

        let agent_file1 = path.join("new_feature.rs");
        fs::write(&agent_file1, "// New feature code").unwrap();

        // Verify agent's changes are present
        assert!(agent_file1.exists(), "Agent should have created new file");
        let content = fs::read_to_string(&initial_file).unwrap();
        assert!(
            content.contains("Modified by agent"),
            "Agent should have modified existing file"
        );

        // ============ STEP 3: Revert to checkpoint (retry attempt 1) ============
        checkpoint.revert("story-1").await.expect("revert should succeed");

        // Verify working directory is restored to pre-agent state
        assert!(
            !agent_file1.exists(),
            "Agent's new file should be gone after revert"
        );
        let content = fs::read_to_string(&initial_file).unwrap();
        assert!(
            content.contains("Hello"),
            "Existing file should be restored to original content"
        );

        // Verify stash is preserved for potential second retry
        let stash_index = checkpoint.find_stash("story-1").await.unwrap();
        assert!(
            stash_index.is_some(),
            "Stash should still exist after revert (for retries)"
        );

        // ============ STEP 4: Agent attempt 2 - makes different changes, still fails ============
        fs::write(&initial_file, "fn main() { println!(\"Second attempt\"); }").unwrap();
        let agent_file2 = path.join("different_approach.rs");
        fs::write(&agent_file2, "// Different approach").unwrap();

        // ============ STEP 5: Revert to checkpoint (retry attempt 2) ============
        checkpoint
            .revert("story-1")
            .await
            .expect("second revert should succeed");

        // Verify clean state again
        assert!(
            !agent_file2.exists(),
            "Second attempt's file should be gone"
        );
        let content = fs::read_to_string(&initial_file).unwrap();
        assert!(content.contains("Hello"), "File should be restored again");

        // ============ STEP 6: Agent attempt 3 - succeeds ============
        fs::write(&initial_file, "fn main() { println!(\"Success!\"); }").unwrap();
        let final_file = path.join("completed_feature.rs");
        fs::write(&final_file, "// Completed feature").unwrap();

        // ============ STEP 7: Drop checkpoint on success ============
        checkpoint
            .drop("story-1")
            .await
            .expect("drop should succeed on success");

        // Verify stash is gone
        let stash_index = checkpoint.find_stash("story-1").await.unwrap();
        assert!(
            stash_index.is_none(),
            "Stash should be dropped after successful completion"
        );

        // Verify agent's final changes are preserved (not reverted)
        assert!(
            final_file.exists(),
            "Successful agent's changes should remain"
        );
        let content = fs::read_to_string(&initial_file).unwrap();
        assert!(
            content.contains("Success"),
            "Final modifications should remain"
        );
    }

    /// Integration test: Multiple stories with checkpoint cleanup
    ///
    /// Simulates processing multiple stories:
    /// 1. Story 1 succeeds after 1 retry
    /// 2. Story 2 succeeds on first try
    /// 3. All checkpoints are cleaned up at the end
    #[tokio::test]
    async fn integration_multiple_stories_with_cleanup() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("multi-story-change");
        let path = repo_path(&checkpoint).clone();

        // ============ Story 1: Fails then succeeds ============
        let story1_file = path.join("story1.txt");
        fs::write(&story1_file, "story 1 initial").unwrap();

        checkpoint
            .save("story-1")
            .await
            .expect("save story-1 should succeed");

        // Agent fails
        fs::write(&story1_file, "story 1 failed attempt").unwrap();

        // Revert
        checkpoint
            .revert("story-1")
            .await
            .expect("revert story-1 should succeed");

        // Agent succeeds
        fs::write(&story1_file, "story 1 success").unwrap();
        checkpoint
            .drop("story-1")
            .await
            .expect("drop story-1 should succeed");

        // Commit story 1 changes
        Command::new("git")
            .args(["add", "."])
            .current_dir(&path)
            .output()
            .unwrap();
        Command::new("git")
            .args(["commit", "-m", "Complete story 1"])
            .current_dir(&path)
            .output()
            .unwrap();

        // ============ Story 2: Succeeds on first try ============
        let story2_file = path.join("story2.txt");
        fs::write(&story2_file, "story 2 initial").unwrap();

        checkpoint
            .save("story-2")
            .await
            .expect("save story-2 should succeed");

        // Agent succeeds immediately
        fs::write(&story2_file, "story 2 success").unwrap();
        checkpoint
            .drop("story-2")
            .await
            .expect("drop story-2 should succeed");

        // ============ Verify no orphaned stashes ============
        let stash1 = checkpoint.find_stash("story-1").await.unwrap();
        let stash2 = checkpoint.find_stash("story-2").await.unwrap();
        assert!(stash1.is_none(), "story-1 stash should be dropped");
        assert!(stash2.is_none(), "story-2 stash should be dropped");
    }

    /// Integration test: Max retries exceeded with cleanup
    ///
    /// Simulates a story that fails max_retries times:
    /// 1. Each attempt fails
    /// 2. After max retries, loop stops
    /// 3. Cleanup removes all orphaned stashes
    #[tokio::test]
    async fn integration_max_retries_exceeded_with_cleanup() {
        let (_temp_dir, checkpoint) = setup_temp_repo_with_checkpoint("failed-change");
        let path = repo_path(&checkpoint).clone();
        let max_retries = 3;

        // Create initial state
        let work_file = path.join("work.txt");
        fs::write(&work_file, "initial work").unwrap();

        // Save checkpoint
        checkpoint.save("story-1").await.expect("save should succeed");

        // Simulate max_retries failed attempts
        for attempt in 1..=max_retries {
            // Agent makes changes
            fs::write(&work_file, format!("attempt {} changes", attempt)).unwrap();

            // Agent fails
            if attempt < max_retries {
                // Revert for retry
                checkpoint.revert("story-1").await.expect("revert should succeed");
            }
        }

        // After max retries exceeded, stash should still exist
        let stash_before_cleanup = checkpoint.find_stash("story-1").await.unwrap();
        assert!(
            stash_before_cleanup.is_some(),
            "Stash should exist before cleanup (never dropped due to max retries)"
        );

        // ============ Cleanup on loop exit ============
        checkpoint.cleanup().await.expect("cleanup should succeed");

        // Verify all stashes are gone
        let stash_after_cleanup = checkpoint.find_stash("story-1").await.unwrap();
        assert!(
            stash_after_cleanup.is_none(),
            "Stash should be cleaned up after loop exit"
        );
    }

    /// Integration test: Stash isolation between different changes
    ///
    /// This tests the scenario where checkpoints for different changes
    /// exist simultaneously.
    #[tokio::test]
    async fn integration_stash_isolation_between_changes() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repo_path = temp_dir.path().to_path_buf();

        // Initialize git repo
        Command::new("git")
            .args(["init"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to init git repo");

        Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to configure git email");

        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to configure git name");

        let test_file = repo_path.join("initial.txt");
        fs::write(&test_file, "initial content").expect("Failed to write initial file");

        Command::new("git")
            .args(["add", "."])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to git add");

        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to create initial commit");

        // Create two checkpoints for different changes, same repo
        let checkpoint_a = Checkpoint::with_work_dir("feature-a", repo_path.clone());
        let checkpoint_b = Checkpoint::with_work_dir("feature-b", repo_path.clone());

        // Create stash for feature-a story-1
        let file_a = repo_path.join("feature_a.txt");
        fs::write(&file_a, "feature a work").unwrap();
        checkpoint_a.save("story-1").await.expect("save a should succeed");

        // Create stash for feature-b story-1
        let file_b = repo_path.join("feature_b.txt");
        fs::write(&file_b, "feature b work").unwrap();
        checkpoint_b.save("story-1").await.expect("save b should succeed");

        // Verify both stashes exist
        assert!(checkpoint_a.find_stash("story-1").await.unwrap().is_some());
        assert!(checkpoint_b.find_stash("story-1").await.unwrap().is_some());

        // Cleanup feature-a
        checkpoint_a.cleanup().await.expect("cleanup a should succeed");

        // Feature-a stash should be gone, feature-b should remain
        assert!(
            checkpoint_a.find_stash("story-1").await.unwrap().is_none(),
            "feature-a stash should be cleaned up"
        );
        assert!(
            checkpoint_b.find_stash("story-1").await.unwrap().is_some(),
            "feature-b stash should remain"
        );

        // Cleanup feature-b
        checkpoint_b.cleanup().await.expect("cleanup b should succeed");
        assert!(
            checkpoint_b.find_stash("story-1").await.unwrap().is_none(),
            "feature-b stash should be cleaned up"
        );
    }
}
