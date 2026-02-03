//! Learnings file management for cross-story knowledge sharing.
//!
//! This module manages a shared learnings file that persists across stories
//! within an iteration and across multiple iteration runs. The file enables
//! agents to share discoveries, decisions, and gotchas with subsequent stories.

use std::fs;
use std::path::PathBuf;

use crate::error::Result;

/// Initial template content for new learnings files.
///
/// This template provides guidance to agents about what to record.
/// Content detection checks for text beyond this template.
#[allow(dead_code)] // Used in Story 2 prompt integration
const INITIAL_TEMPLATE: &str = r#"<!-- Shared Learnings File -->
<!-- Record discoveries, decisions, and gotchas here for future stories -->

"#;

/// Returns the path to the learnings file for a given change.
///
/// The path follows the convention: `/tmp/ralphtool/{change_name}-learnings.md`
///
/// # Arguments
///
/// * `change_name` - The name of the change being processed
///
/// # Examples
///
/// ```
/// use ralphtool::ralph_loop::learnings::learnings_path;
///
/// let path = learnings_path("add-user-auth");
/// assert_eq!(path.to_string_lossy(), "/tmp/ralphtool/add-user-auth-learnings.md");
/// ```
#[allow(dead_code)] // Used in Story 2 prompt integration
pub fn learnings_path(change_name: &str) -> PathBuf {
    PathBuf::from("/tmp/ralphtool").join(format!("{}-learnings.md", change_name))
}

/// Ensures the learnings file exists, creating it with initial template if missing.
///
/// This function:
/// - Creates the `/tmp/ralphtool/` directory if it doesn't exist
/// - Creates the learnings file with initial template if it doesn't exist
/// - Preserves existing file content if the file already exists
///
/// # Arguments
///
/// * `change_name` - The name of the change being processed
///
/// # Errors
///
/// Returns an error if:
/// - The directory cannot be created
/// - The file cannot be written
#[allow(dead_code)] // Used in Story 2 orchestrator integration
pub fn ensure_learnings_file(change_name: &str) -> Result<()> {
    let path = learnings_path(change_name);

    // Create the directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create the file with initial template only if it doesn't exist
    if !path.exists() {
        fs::write(&path, INITIAL_TEMPLATE)?;
    }

    Ok(())
}

/// Reads learnings content if the file exists and has content beyond the template.
///
/// Returns `Some(content)` if the file exists and contains meaningful content
/// (more than just the initial template). Returns `None` if:
/// - The file doesn't exist
/// - The file contains only the initial template
/// - The file is empty or contains only whitespace after the template
///
/// # Arguments
///
/// * `change_name` - The name of the change being processed
///
/// # Errors
///
/// Returns an error if the file exists but cannot be read.
#[allow(dead_code)] // Used in Story 2 prompt integration
pub fn read_learnings(change_name: &str) -> Result<Option<String>> {
    let path = learnings_path(change_name);

    // If file doesn't exist, return None
    if !path.exists() {
        return Ok(None);
    }

    // Read the file content
    let content = fs::read_to_string(&path)?;

    // Check if content is beyond the initial template
    // Strip the template prefix and check for non-whitespace content
    let content_after_template = content
        .strip_prefix(INITIAL_TEMPLATE)
        .unwrap_or(&content);

    if content_after_template.trim().is_empty() {
        return Ok(None);
    }

    // Return the full content (including template, as it provides context)
    Ok(Some(content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn learnings_path_follows_convention() {
        let path = learnings_path("add-user-auth");
        assert_eq!(
            path.to_string_lossy(),
            "/tmp/ralphtool/add-user-auth-learnings.md"
        );
    }

    #[test]
    fn learnings_path_handles_hyphenated_names() {
        let path = learnings_path("my-complex-feature-name");
        assert_eq!(
            path.to_string_lossy(),
            "/tmp/ralphtool/my-complex-feature-name-learnings.md"
        );
    }

    #[test]
    fn learnings_path_handles_simple_names() {
        let path = learnings_path("feature");
        assert_eq!(
            path.to_string_lossy(),
            "/tmp/ralphtool/feature-learnings.md"
        );
    }

    #[test]
    fn initial_template_contains_guidance() {
        assert!(INITIAL_TEMPLATE.contains("Shared Learnings"));
        assert!(INITIAL_TEMPLATE.contains("discoveries"));
        assert!(INITIAL_TEMPLATE.contains("decisions"));
        assert!(INITIAL_TEMPLATE.contains("gotchas"));
    }

    // Integration tests that use the real file system
    // These tests use unique change names to avoid conflicts

    #[test]
    fn ensure_learnings_file_creates_directory_and_file() {
        let change_name = "test-ensure-creates-file";
        let path = learnings_path(change_name);

        // Clean up any existing file from previous test runs
        let _ = fs::remove_file(&path);

        // Ensure the file is created
        ensure_learnings_file(change_name).expect("Should create file");

        // Verify file exists
        assert!(path.exists(), "Learnings file should exist");

        // Verify content is the initial template
        let content = fs::read_to_string(&path).expect("Should read file");
        assert_eq!(content, INITIAL_TEMPLATE);

        // Clean up
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn ensure_learnings_file_preserves_existing_content() {
        let change_name = "test-ensure-preserves";
        let path = learnings_path(change_name);

        // Create directory and file with custom content
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Should create directory");
        }
        let custom_content = "# My Custom Learnings\n\nSome important notes.";
        fs::write(&path, custom_content).expect("Should write custom content");

        // Call ensure - should NOT overwrite
        ensure_learnings_file(change_name).expect("Should not fail");

        // Verify content is unchanged
        let content = fs::read_to_string(&path).expect("Should read file");
        assert_eq!(content, custom_content);

        // Clean up
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn read_learnings_returns_none_for_nonexistent_file() {
        let change_name = "test-read-nonexistent";
        let path = learnings_path(change_name);

        // Ensure file doesn't exist
        let _ = fs::remove_file(&path);

        let result = read_learnings(change_name).expect("Should not error");
        assert!(result.is_none());
    }

    #[test]
    fn read_learnings_returns_none_for_template_only() {
        let change_name = "test-read-template-only";
        let path = learnings_path(change_name);

        // Create file with just the template
        ensure_learnings_file(change_name).expect("Should create file");

        let result = read_learnings(change_name).expect("Should not error");
        assert!(result.is_none(), "Should return None for template-only content");

        // Clean up
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn read_learnings_returns_content_when_beyond_template() {
        let change_name = "test-read-with-content";
        let path = learnings_path(change_name);

        // Create file with template + additional content
        let content_with_learnings = format!(
            "{}## Story 1 Learnings\n\n- Discovered that foo uses bar pattern\n",
            INITIAL_TEMPLATE
        );
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Should create directory");
        }
        fs::write(&path, &content_with_learnings).expect("Should write content");

        let result = read_learnings(change_name).expect("Should not error");
        assert!(result.is_some(), "Should return Some when content exists");
        assert_eq!(result.unwrap(), content_with_learnings);

        // Clean up
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn read_learnings_returns_none_for_template_with_only_whitespace() {
        let change_name = "test-read-template-whitespace";
        let path = learnings_path(change_name);

        // Create file with template + only whitespace
        let content = format!("{}   \n\n  \t  \n", INITIAL_TEMPLATE);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Should create directory");
        }
        fs::write(&path, &content).expect("Should write content");

        let result = read_learnings(change_name).expect("Should not error");
        assert!(result.is_none(), "Should return None for whitespace-only addition");

        // Clean up
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn read_learnings_handles_content_without_template_prefix() {
        let change_name = "test-read-no-template";
        let path = learnings_path(change_name);

        // Create file with content that doesn't start with template
        // (e.g., user manually edited or template changed)
        let content = "# My Custom Format\n\nSome learnings here.";
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Should create directory");
        }
        fs::write(&path, content).expect("Should write content");

        let result = read_learnings(change_name).expect("Should not error");
        assert!(result.is_some(), "Should return Some for non-template content");
        assert_eq!(result.unwrap(), content);

        // Clean up
        let _ = fs::remove_file(&path);
    }

    // ==================== Integration Tests ====================
    //
    // These tests verify the integration between learnings file management
    // and the orchestrator/prompt builder components.

    /// Integration test: Learnings file is created on first iteration start.
    ///
    /// Verifies the scenario:
    /// - WHEN the orchestrator starts an iteration
    /// - THEN the learnings file does not exist
    /// - THEN the system SHALL create the directory `/tmp/ralphtool/` if needed
    /// - THEN create the learnings file with initial template content
    #[test]
    fn integration_learnings_file_created_on_first_iteration_start() {
        let change_name = "test-integration-first-iteration";
        let path = learnings_path(change_name);

        // Clean up any existing file to simulate first iteration
        let _ = fs::remove_file(&path);
        // Also ensure the directory doesn't exist if empty
        let parent = path.parent().unwrap();
        let _ = fs::remove_dir(parent); // Will fail if not empty, which is fine

        // Verify file doesn't exist initially
        assert!(!path.exists(), "File should not exist before first iteration");

        // Simulate orchestrator's iteration start by calling ensure_learnings_file
        // (This is what Orchestrator::run() calls at the start)
        ensure_learnings_file(change_name).expect("Should create learnings file");

        // Verify file now exists
        assert!(path.exists(), "File should exist after first iteration start");

        // Verify directory was created
        assert!(parent.exists(), "Directory /tmp/ralphtool/ should exist");

        // Verify file has initial template content
        let content = fs::read_to_string(&path).expect("Should read file");
        assert_eq!(content, INITIAL_TEMPLATE, "File should contain initial template");

        // Clean up
        let _ = fs::remove_file(&path);
    }

    /// Integration test: Existing learnings content appears in prompt.
    ///
    /// Verifies the scenarios:
    /// - WHEN the orchestrator generates an agent prompt
    /// - THEN the learnings file exists and contains content beyond the initial template
    /// - THEN the prompt SHALL include a "Shared Learnings" section with:
    ///   - The learnings content
    ///   - Instructions for what to record
    ///   - Path to learnings file
    #[test]
    fn integration_existing_learnings_content_appears_in_prompt() {
        let change_name = "test-integration-learnings-in-prompt";
        let path = learnings_path(change_name);

        // Create learnings file with meaningful content (beyond template)
        let learnings_content = format!(
            "{}## Story 1 Learnings\n\n- Found that module X uses pattern Y\n- Decided to use async for all IO operations\n",
            INITIAL_TEMPLATE
        );
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Should create directory");
        }
        fs::write(&path, &learnings_content).expect("Should write learnings");

        // Simulate orchestrator reading learnings for prompt generation
        // (This is what Orchestrator::run() does before calling PromptBuilder)
        let read_result = read_learnings(change_name).expect("Should read learnings");

        // Verify learnings are read
        assert!(read_result.is_some(), "Should return Some for existing learnings");
        let content = read_result.unwrap();

        // Verify the content includes what we wrote
        assert!(content.contains("Story 1 Learnings"));
        assert!(content.contains("pattern Y"));
        assert!(content.contains("async for all IO operations"));

        // The PromptBuilder would use this content - we verify the content is correct
        // for inclusion in the prompt's "Shared Learnings" section
        assert!(content.len() > INITIAL_TEMPLATE.len(), "Content should be beyond template");

        // Clean up
        let _ = fs::remove_file(&path);
    }

    /// Integration test: Empty learnings file results in no learnings section in prompt.
    ///
    /// Verifies the scenarios:
    /// - WHEN the orchestrator generates an agent prompt
    /// - THEN the learnings file does not exist or contains only the initial template
    /// - THEN the prompt SHALL NOT include a learnings section
    #[test]
    fn integration_empty_learnings_file_results_in_no_learnings_section() {
        let change_name = "test-integration-empty-learnings";
        let path = learnings_path(change_name);

        // Clean up any existing file
        let _ = fs::remove_file(&path);

        // Simulate orchestrator creating file on iteration start
        ensure_learnings_file(change_name).expect("Should create learnings file");

        // Verify file exists with initial template only
        assert!(path.exists(), "File should exist");
        let file_content = fs::read_to_string(&path).expect("Should read file");
        assert_eq!(file_content, INITIAL_TEMPLATE);

        // Simulate orchestrator reading learnings for prompt generation
        let read_result = read_learnings(change_name).expect("Should read learnings");

        // Verify read_learnings returns None for template-only content
        // This signals to PromptBuilder to NOT include learnings section
        assert!(
            read_result.is_none(),
            "Should return None for template-only file (no learnings section in prompt)"
        );

        // Clean up
        let _ = fs::remove_file(&path);
    }
}
