//! Checkpoint module for branch-based state preservation.
//!
//! Provides checkpoint/revert functionality to preserve working directory state
//! before agent execution and restore it on failure.
//!
//! Uses a dedicated `ralph/{change_name}` branch with commits as checkpoints
//! instead of git stash. This avoids the "save and clean" behavior of stash
//! that was causing issues with completed story changes being lost.
//!
//! All operations are async-safe, using `async_cmd` to avoid blocking tokio
//! worker threads.

use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use crate::async_cmd;
use crate::error::{Error, Result};

/// Option for handling completion when the loop finishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionOption {
    /// Cleanup: return to original branch with uncommitted changes.
    Cleanup,
    /// Keep: stay on the ralph branch with checkpoint commits.
    Keep,
}

/// Checkpoint manager using branch + commit for state preservation.
///
/// Creates a `ralph/{change_name}` branch at loop start and uses commits
/// as checkpoints. On failure, uses `git reset --hard HEAD` to restore.
#[derive(Debug, Clone)]
pub struct Checkpoint {
    /// The name of the change being worked on.
    change_name: String,
    /// Optional working directory for git commands (for testing).
    work_dir: Option<PathBuf>,
    /// Timeout for git commands.
    timeout: Duration,
    /// The original branch name before switching to ralph branch.
    original_branch: Option<String>,
}

impl Checkpoint {
    /// Creates a new Checkpoint with the default timeout.
    pub fn new(change_name: impl Into<String>) -> Self {
        Self::with_timeout(change_name, async_cmd::DEFAULT_TIMEOUT)
    }

    /// Creates a new Checkpoint with a custom timeout.
    pub fn with_timeout(change_name: impl Into<String>, timeout: Duration) -> Self {
        Self {
            change_name: change_name.into(),
            work_dir: None,
            timeout,
            original_branch: None,
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
            original_branch: None,
        }
    }

    /// Returns the name of the ralph branch for this change.
    fn branch_name(&self) -> String {
        format!("ralph/{}", self.change_name)
    }

    /// Initializes the checkpoint system by creating a ralph branch.
    ///
    /// Stores the current branch name, creates/switches to `ralph/{change_name}`,
    /// and creates an "initial state" commit with `--allow-empty`.
    pub async fn init(&mut self) -> Result<()> {
        // Get current branch name
        let output = self.run_git(&["rev-parse", "--abbrev-ref", "HEAD"]).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: "git rev-parse --abbrev-ref HEAD".to_string(),
                stderr,
            });
        }
        let current_branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        self.original_branch = Some(current_branch);

        // Create/switch to ralph branch
        let branch_name = self.branch_name();
        let output = self.run_git(&["checkout", "-B", &branch_name]).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: format!("git checkout -B {}", branch_name),
                stderr,
            });
        }

        // Stage all changes and create initial commit
        let output = self.run_git(&["add", "-A"]).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: "git add -A".to_string(),
                stderr,
            });
        }

        let output = self
            .run_git(&["commit", "--allow-empty", "-m", "initial state"])
            .await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: "git commit --allow-empty -m 'initial state'".to_string(),
                stderr,
            });
        }

        Ok(())
    }

    /// Creates a checkpoint commit after a story completes successfully.
    ///
    /// Stages all changes and creates a commit with message "checkpoint: {story_id}".
    pub async fn commit_checkpoint(&self, story_id: &str) -> Result<()> {
        // Stage all changes
        let output = self.run_git(&["add", "-A"]).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: "git add -A".to_string(),
                stderr,
            });
        }

        // Create checkpoint commit
        let message = format!("checkpoint: {}", story_id);
        let output = self.run_git(&["commit", "--allow-empty", "-m", &message]).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: format!("git commit -m '{}'", message),
                stderr,
            });
        }

        Ok(())
    }

    /// Reverts to the last checkpoint by resetting to HEAD.
    ///
    /// Uses `git reset --hard HEAD` to discard tracked changes and
    /// `git clean -fd` to remove untracked files and directories.
    pub async fn revert(&self) -> Result<()> {
        // Reset tracked files
        let output = self.run_git(&["reset", "--hard", "HEAD"]).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: "git reset --hard HEAD".to_string(),
                stderr,
            });
        }

        // Clean untracked files and directories
        let output = self.run_git(&["clean", "-fd"]).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: "git clean -fd".to_string(),
                stderr,
            });
        }

        Ok(())
    }

    /// Handles completion based on the user's choice.
    ///
    /// - `Cleanup`: Returns to original branch with uncommitted changes
    /// - `Keep`: Stays on ralph branch with checkpoint commits
    pub async fn cleanup(&self, option: CompletionOption) -> Result<()> {
        match option {
            CompletionOption::Cleanup => self.do_cleanup().await,
            CompletionOption::Keep => Ok(()), // No-op for keep
        }
    }

    /// Performs cleanup: checkout original branch, merge --squash, reset HEAD, delete branch.
    async fn do_cleanup(&self) -> Result<()> {
        let original_branch = self.original_branch.as_ref().ok_or_else(|| Error::Command {
            cmd: "cleanup".to_string(),
            stderr: "No original branch stored - was init() called?".to_string(),
        })?;

        let branch_name = self.branch_name();

        // Checkout original branch
        let output = self.run_git(&["checkout", original_branch]).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: format!("git checkout {}", original_branch),
                stderr,
            });
        }

        // Merge --squash to bring all changes as staged
        let output = self.run_git(&["merge", "--squash", &branch_name]).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: format!("git merge --squash {}", branch_name),
                stderr,
            });
        }

        // Reset HEAD to make changes unstaged
        let output = self.run_git(&["reset", "HEAD"]).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: "git reset HEAD".to_string(),
                stderr,
            });
        }

        // Delete the ralph branch
        let output = self.run_git(&["branch", "-D", &branch_name]).await?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(Error::Command {
                cmd: format!("git branch -D {}", branch_name),
                stderr,
            });
        }

        Ok(())
    }

    /// Returns the original branch name, if available.
    pub fn original_branch(&self) -> Option<&str> {
        self.original_branch.as_deref()
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
    fn checkpoint_with_timeout_sets_change_name() {
        let checkpoint = Checkpoint::with_timeout("test-change", async_cmd::DEFAULT_TIMEOUT);
        assert_eq!(checkpoint.change_name, "test-change");
    }

    #[test]
    fn branch_name_format() {
        let checkpoint = Checkpoint::with_timeout("my-feature", async_cmd::DEFAULT_TIMEOUT);
        assert_eq!(checkpoint.branch_name(), "ralph/my-feature");
    }

    #[test]
    fn completion_option_enum() {
        // Verify both options exist and are distinct
        assert_ne!(CompletionOption::Cleanup, CompletionOption::Keep);
        assert_eq!(CompletionOption::Cleanup, CompletionOption::Cleanup);
        assert_eq!(CompletionOption::Keep, CompletionOption::Keep);
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

        // Create an initial commit (required for branch operations)
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
    /// Returns a cloned PathBuf to avoid borrowing issues.
    fn repo_path(checkpoint: &Checkpoint) -> PathBuf {
        checkpoint.work_dir.as_ref().expect("Checkpoint should have work_dir for tests").clone()
    }

    /// Helper to get current branch name
    fn get_current_branch(repo_path: impl AsRef<std::path::Path>) -> String {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to get current branch");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    /// Helper to get commit count
    fn get_commit_count(repo_path: impl AsRef<std::path::Path>) -> usize {
        let output = Command::new("git")
            .args(["rev-list", "--count", "HEAD"])
            .current_dir(repo_path)
            .output()
            .expect("Failed to count commits");
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse()
            .unwrap_or(0)
    }

    // ==================== init() tests ====================

    #[tokio::test]
    async fn init_stores_original_branch() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        // Get original branch name before init
        let original_branch = get_current_branch(&path);

        checkpoint.init().await.expect("init should succeed");

        // Verify original branch was stored
        assert_eq!(
            checkpoint.original_branch(),
            Some(original_branch.as_str())
        );
    }

    #[tokio::test]
    async fn init_creates_ralph_branch() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        checkpoint.init().await.expect("init should succeed");

        // Verify we're on the ralph branch
        let current_branch = get_current_branch(&path);
        assert_eq!(current_branch, "ralph/my-change");
    }

    #[tokio::test]
    async fn init_creates_initial_commit() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        let commits_before = get_commit_count(&path);
        checkpoint.init().await.expect("init should succeed");
        let commits_after = get_commit_count(&path);

        // Should have created one new commit
        assert_eq!(commits_after, commits_before + 1);

        // Verify commit message
        let output = Command::new("git")
            .args(["log", "-1", "--format=%s"])
            .current_dir(&path)
            .output()
            .expect("Failed to get commit message");
        let message = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert_eq!(message, "initial state");
    }

    #[tokio::test]
    async fn init_includes_uncommitted_changes() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        // Create uncommitted changes before init
        let new_file = path.join("uncommitted.txt");
        fs::write(&new_file, "uncommitted content").expect("Failed to write file");

        checkpoint.init().await.expect("init should succeed");

        // Verify file is committed in initial state
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&path)
            .output()
            .expect("Failed to get git status");
        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert!(status.is_empty(), "Working directory should be clean after init");
    }

    #[tokio::test]
    async fn init_force_creates_branch_if_exists() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        // First init
        checkpoint.init().await.expect("first init should succeed");

        // Go back to original branch manually
        Command::new("git")
            .args(["checkout", "-"])
            .current_dir(&path)
            .output()
            .expect("Failed to checkout");

        // Modify the checkpoint
        checkpoint.original_branch = None;

        // Second init should work (force recreates branch)
        checkpoint.init().await.expect("second init should succeed");

        let current_branch = get_current_branch(&path);
        assert_eq!(current_branch, "ralph/my-change");
    }

    // ==================== commit_checkpoint() tests ====================

    #[tokio::test]
    async fn commit_checkpoint_creates_commit_with_story_id() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        checkpoint.init().await.expect("init should succeed");

        // Make changes
        let test_file = path.join("feature.txt");
        fs::write(&test_file, "feature code").expect("Failed to write file");

        checkpoint.commit_checkpoint("story-1").await.expect("commit_checkpoint should succeed");

        // Verify commit message
        let output = Command::new("git")
            .args(["log", "-1", "--format=%s"])
            .current_dir(path)
            .output()
            .expect("Failed to get commit message");
        let message = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert_eq!(message, "checkpoint: story-1");
    }

    #[tokio::test]
    async fn commit_checkpoint_includes_all_changes() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        checkpoint.init().await.expect("init should succeed");

        // Create tracked and untracked files
        let tracked_file = path.join("tracked.txt");
        fs::write(&tracked_file, "tracked content").expect("Failed to write file");
        let untracked_file = path.join("untracked.txt");
        fs::write(&untracked_file, "untracked content").expect("Failed to write file");

        checkpoint.commit_checkpoint("story-1").await.expect("commit_checkpoint should succeed");

        // Verify working directory is clean
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(path)
            .output()
            .expect("Failed to get git status");
        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert!(status.is_empty(), "Working directory should be clean after checkpoint");
    }

    // ==================== revert() tests ====================

    #[tokio::test]
    async fn revert_discards_uncommitted_changes() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        checkpoint.init().await.expect("init should succeed");

        // Make changes after init
        let agent_file = path.join("agent_created.txt");
        fs::write(&agent_file, "agent output").expect("Failed to write file");

        // Modify existing file
        let initial_file = path.join("initial.txt");
        fs::write(&initial_file, "modified content").expect("Failed to write file");

        checkpoint.revert().await.expect("revert should succeed");

        // Verify agent file is gone
        assert!(!agent_file.exists(), "Agent-created file should be removed");

        // Verify existing file is restored
        let content = fs::read_to_string(&initial_file).expect("Failed to read file");
        assert_eq!(content, "initial content");
    }

    #[tokio::test]
    async fn revert_preserves_committed_changes() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        checkpoint.init().await.expect("init should succeed");

        // Story 1 completes successfully
        let story1_file = path.join("story1.txt");
        fs::write(&story1_file, "story 1 code").expect("Failed to write file");
        checkpoint.commit_checkpoint("story-1").await.expect("commit should succeed");

        // Story 2 starts and fails
        let story2_file = path.join("story2.txt");
        fs::write(&story2_file, "story 2 failed attempt").expect("Failed to write file");

        checkpoint.revert().await.expect("revert should succeed");

        // Story 1 file should still exist (committed)
        assert!(story1_file.exists(), "Story 1 file should remain after revert");

        // Story 2 file should be gone (uncommitted)
        assert!(!story2_file.exists(), "Story 2 file should be removed after revert");
    }

    // ==================== cleanup() tests ====================

    #[tokio::test]
    async fn cleanup_with_cleanup_option_returns_to_original_branch() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        let original_branch = get_current_branch(&path);
        checkpoint.init().await.expect("init should succeed");

        // Make and commit changes
        let test_file = path.join("feature.txt");
        fs::write(&test_file, "feature code").expect("Failed to write file");
        checkpoint.commit_checkpoint("story-1").await.expect("commit should succeed");

        checkpoint.cleanup(CompletionOption::Cleanup).await.expect("cleanup should succeed");

        // Verify we're back on original branch
        let current_branch = get_current_branch(&path);
        assert_eq!(current_branch, original_branch);
    }

    #[tokio::test]
    async fn cleanup_with_cleanup_option_brings_uncommitted_changes() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        checkpoint.init().await.expect("init should succeed");

        // Make and commit changes
        let test_file = path.join("feature.txt");
        fs::write(&test_file, "feature code").expect("Failed to write file");
        checkpoint.commit_checkpoint("story-1").await.expect("commit should succeed");

        checkpoint.cleanup(CompletionOption::Cleanup).await.expect("cleanup should succeed");

        // Verify changes are present but uncommitted
        assert!(test_file.exists(), "Feature file should exist");

        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&path)
            .output()
            .expect("Failed to get git status");
        let status = String::from_utf8_lossy(&output.stdout);
        assert!(!status.is_empty(), "Should have uncommitted changes after cleanup");
    }

    #[tokio::test]
    async fn cleanup_with_cleanup_option_deletes_ralph_branch() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        checkpoint.init().await.expect("init should succeed");
        checkpoint.commit_checkpoint("story-1").await.expect("commit should succeed");
        checkpoint.cleanup(CompletionOption::Cleanup).await.expect("cleanup should succeed");

        // Verify ralph branch is deleted
        let output = Command::new("git")
            .args(["branch", "--list", "ralph/my-change"])
            .current_dir(&path)
            .output()
            .expect("Failed to list branches");
        let branches = String::from_utf8_lossy(&output.stdout).trim().to_string();
        assert!(branches.is_empty(), "Ralph branch should be deleted after cleanup");
    }

    #[tokio::test]
    async fn cleanup_with_keep_option_stays_on_ralph_branch() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        checkpoint.init().await.expect("init should succeed");
        checkpoint.commit_checkpoint("story-1").await.expect("commit should succeed");
        checkpoint.cleanup(CompletionOption::Keep).await.expect("keep should succeed");

        // Verify we're still on ralph branch
        let current_branch = get_current_branch(&path);
        assert_eq!(current_branch, "ralph/my-change");
    }

    #[tokio::test]
    async fn cleanup_with_keep_option_preserves_commits() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("my-change");
        let path = repo_path(&checkpoint);

        checkpoint.init().await.expect("init should succeed");

        // Make multiple commits
        let file1 = path.join("story1.txt");
        fs::write(&file1, "story 1").expect("Failed to write file");
        checkpoint.commit_checkpoint("story-1").await.expect("commit should succeed");

        let file2 = path.join("story2.txt");
        fs::write(&file2, "story 2").expect("Failed to write file");
        checkpoint.commit_checkpoint("story-2").await.expect("commit should succeed");

        let commits_before = get_commit_count(&path);
        checkpoint.cleanup(CompletionOption::Keep).await.expect("keep should succeed");
        let commits_after = get_commit_count(&path);

        // Commits should be preserved
        assert_eq!(commits_after, commits_before);
    }

    // ==================== Integration tests ====================

    #[tokio::test]
    async fn integration_full_checkpoint_cycle_with_retries() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("test-feature");
        let path = repo_path(&checkpoint).clone();

        // Initialize checkpoint system
        checkpoint.init().await.expect("init should succeed");

        // Story 1: Agent fails twice, then succeeds
        let story1_file = path.join("story1.txt");

        // Attempt 1: fails
        fs::write(&story1_file, "attempt 1 - wrong approach").expect("write failed");
        checkpoint.revert().await.expect("revert 1 should succeed");
        assert!(!story1_file.exists(), "Failed attempt should be reverted");

        // Attempt 2: fails
        fs::write(&story1_file, "attempt 2 - still wrong").expect("write failed");
        checkpoint.revert().await.expect("revert 2 should succeed");
        assert!(!story1_file.exists(), "Failed attempt should be reverted");

        // Attempt 3: succeeds
        fs::write(&story1_file, "attempt 3 - success!").expect("write failed");
        checkpoint.commit_checkpoint("story-1").await.expect("commit should succeed");

        // Story 2: Succeeds on first try
        let story2_file = path.join("story2.txt");
        fs::write(&story2_file, "story 2 success").expect("write failed");
        checkpoint.commit_checkpoint("story-2").await.expect("commit should succeed");

        // Cleanup with cleanup option
        checkpoint.cleanup(CompletionOption::Cleanup).await.expect("cleanup should succeed");

        // Verify both story files exist as uncommitted changes
        assert!(story1_file.exists(), "Story 1 file should exist");
        assert!(story2_file.exists(), "Story 2 file should exist");

        // Verify we're on original branch
        let branch = get_current_branch(&path);
        assert_ne!(branch, "ralph/test-feature", "Should be back on original branch");
    }

    #[tokio::test]
    async fn integration_max_retries_with_partial_work() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("partial-work");
        let path = repo_path(&checkpoint).clone();

        checkpoint.init().await.expect("init should succeed");

        // Story 1 completes
        let story1_file = path.join("story1.txt");
        fs::write(&story1_file, "story 1 complete").expect("write failed");
        checkpoint.commit_checkpoint("story-1").await.expect("commit should succeed");

        // Story 2 fails repeatedly (max retries exceeded)
        for i in 1..=3 {
            let story2_file = path.join("story2.txt");
            fs::write(&story2_file, format!("attempt {}", i)).expect("write failed");
            checkpoint.revert().await.expect("revert should succeed");
        }

        // User chooses to cleanup - partial work (story 1) should be preserved
        checkpoint.cleanup(CompletionOption::Cleanup).await.expect("cleanup should succeed");

        // Story 1 work should exist as uncommitted changes
        assert!(story1_file.exists(), "Story 1 work should be preserved");
        let content = fs::read_to_string(&story1_file).expect("read failed");
        assert_eq!(content, "story 1 complete");
    }

    #[tokio::test]
    async fn integration_keep_option_preserves_history() {
        let (_temp_dir, mut checkpoint) = setup_temp_repo_with_checkpoint("keep-history");
        let path = repo_path(&checkpoint).clone();

        checkpoint.init().await.expect("init should succeed");

        // Complete multiple stories
        for i in 1..=3 {
            let file = path.join(format!("story{}.txt", i));
            fs::write(&file, format!("story {} content", i)).expect("write failed");
            checkpoint.commit_checkpoint(&format!("story-{}", i)).await.expect("commit should succeed");
        }

        // User chooses to keep
        checkpoint.cleanup(CompletionOption::Keep).await.expect("keep should succeed");

        // Verify commit history is intact
        let output = Command::new("git")
            .args(["log", "--oneline"])
            .current_dir(&path)
            .output()
            .expect("Failed to get log");
        let log = String::from_utf8_lossy(&output.stdout);

        assert!(log.contains("checkpoint: story-1"), "Story 1 commit should exist");
        assert!(log.contains("checkpoint: story-2"), "Story 2 commit should exist");
        assert!(log.contains("checkpoint: story-3"), "Story 3 commit should exist");
        assert!(log.contains("initial state"), "Initial commit should exist");
    }
}
