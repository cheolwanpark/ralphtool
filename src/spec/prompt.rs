//! Prompt generation for coding agents.
//!
//! This module generates a self-contained prompt for AI coding agents.
//! The prompt tells the agent how to work on a change autonomously by
//! reading files directly and marking tasks complete in tasks.md.

use std::path::PathBuf;

use crate::error::Result;
use super::SpecAdapter;

/// Generates a self-contained prompt for an agent to work on a change.
///
/// The prompt includes:
/// - Path to the change directory
/// - Workflow instructions (read files, implement, mark tasks done)
/// - Verification commands
/// - Instructions for story progression
///
/// The agent does not need environment variables or special CLI commands.
pub fn generate_prompt(change_name: &str, adapter: &dyn SpecAdapter) -> Result<String> {
    let change_dir = get_change_directory(change_name);
    let verify = adapter.verify_commands()?;

    let mut sections = Vec::new();

    // Header
    sections.push(format!("# Working on Change: {}\n", change_name));

    // Change location
    sections.push("## Change Location\n".to_string());
    sections.push(format!("The change files are located at: `{}`\n", change_dir.display()));

    // Files to read
    sections.push("## Files to Read\n".to_string());
    sections.push("Read these files to understand the change:\n".to_string());
    sections.push(format!("- `{}/proposal.md` - Motivation and scope", change_dir.display()));
    sections.push(format!("- `{}/design.md` - Technical decisions", change_dir.display()));
    sections.push(format!("- `{}/tasks.md` - Stories and tasks to implement", change_dir.display()));
    sections.push(format!("- `{}/specs/` - Detailed requirements\n", change_dir.display()));

    // Workflow instructions
    sections.push("## Workflow\n".to_string());
    sections.push("1. Read the proposal and design to understand the context".to_string());
    sections.push("2. Read tasks.md to see the stories and tasks".to_string());
    sections.push("3. Complete all tasks in Story 1 before moving to Story 2, etc.".to_string());
    sections.push("4. For each task:".to_string());
    sections.push("   - Implement the required changes".to_string());
    sections.push("   - Run verification commands".to_string());
    sections.push("   - Mark the task complete in tasks.md\n".to_string());

    // Task marking instructions
    sections.push("## Marking Tasks Complete\n".to_string());
    sections.push(format!(
        "Edit `{}/tasks.md` directly to mark tasks complete:",
        change_dir.display()
    ));
    sections.push("- Change `- [ ]` to `- [x]` for completed tasks".to_string());
    sections.push("- Example: `- [ ] 1.1 Task description` becomes `- [x] 1.1 Task description`\n".to_string());

    // Verification commands
    sections.push("## Verification\n".to_string());
    if !verify.checks.is_empty() {
        sections.push("Run these checks after implementing:".to_string());
        for check in &verify.checks {
            sections.push(format!("```bash\n{}\n```", check));
        }
    }
    if !verify.tests.is_empty() {
        sections.push(format!("\nRun tests:\n```bash\n{}\n```\n", verify.tests));
    }

    // Instructions
    sections.push("## Instructions\n".to_string());
    sections.push("Work through all stories in order. For each story:".to_string());
    sections.push("1. Complete all tasks in the story".to_string());
    sections.push("2. Verify with the commands above".to_string());
    sections.push("3. Mark tasks complete by editing tasks.md".to_string());
    sections.push("4. Move to the next story\n".to_string());
    sections.push("Stop when all tasks in all stories are marked complete.".to_string());

    Ok(sections.join("\n"))
}

/// Returns the path to a change directory.
fn get_change_directory(change_name: &str) -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_default();
    cwd.join("openspec").join("changes").join(change_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::{Context, Scenario, Story, VerifyCommands};

    struct MockAdapter {
        verify: VerifyCommands,
    }

    impl SpecAdapter for MockAdapter {
        fn stories(&self) -> Result<Vec<Story>> {
            Ok(vec![])
        }

        fn scenarios(&self) -> Result<Vec<Scenario>> {
            Ok(vec![])
        }

        fn context(&self, _story_id: &str) -> Result<Context> {
            unimplemented!()
        }

        fn verify_commands(&self) -> Result<VerifyCommands> {
            Ok(self.verify.clone())
        }
    }

    #[test]
    fn generates_prompt_with_change_location() {
        let adapter = MockAdapter {
            verify: VerifyCommands {
                checks: vec!["cargo check".to_string()],
                tests: "cargo test".to_string(),
            },
        };

        let prompt = generate_prompt("my-change", &adapter).unwrap();

        assert!(prompt.contains("my-change"));
        assert!(prompt.contains("openspec/changes/my-change"));
    }

    #[test]
    fn generates_prompt_with_workflow_instructions() {
        let adapter = MockAdapter {
            verify: VerifyCommands {
                checks: vec![],
                tests: String::new(),
            },
        };

        let prompt = generate_prompt("test", &adapter).unwrap();

        assert!(prompt.contains("proposal.md"));
        assert!(prompt.contains("design.md"));
        assert!(prompt.contains("tasks.md"));
        assert!(prompt.contains("specs/"));
    }

    #[test]
    fn generates_prompt_with_task_marking_instructions() {
        let adapter = MockAdapter {
            verify: VerifyCommands {
                checks: vec![],
                tests: String::new(),
            },
        };

        let prompt = generate_prompt("test", &adapter).unwrap();

        assert!(prompt.contains("- [ ]"));
        assert!(prompt.contains("- [x]"));
        assert!(prompt.contains("Edit"));
    }

    #[test]
    fn generates_prompt_with_verification_commands() {
        let adapter = MockAdapter {
            verify: VerifyCommands {
                checks: vec!["cargo check".to_string(), "cargo clippy".to_string()],
                tests: "cargo test".to_string(),
            },
        };

        let prompt = generate_prompt("test", &adapter).unwrap();

        assert!(prompt.contains("cargo check"));
        assert!(prompt.contains("cargo clippy"));
        assert!(prompt.contains("cargo test"));
    }

    #[test]
    fn generates_prompt_with_story_progression() {
        let adapter = MockAdapter {
            verify: VerifyCommands {
                checks: vec![],
                tests: String::new(),
            },
        };

        let prompt = generate_prompt("test", &adapter).unwrap();

        assert!(prompt.contains("Story 1 before"));
        assert!(prompt.contains("Story 2"));
    }

    struct FailingAdapter;

    impl SpecAdapter for FailingAdapter {
        fn stories(&self) -> Result<Vec<Story>> {
            Ok(vec![])
        }

        fn scenarios(&self) -> Result<Vec<Scenario>> {
            Ok(vec![])
        }

        fn context(&self, _story_id: &str) -> Result<Context> {
            unimplemented!()
        }

        fn verify_commands(&self) -> Result<VerifyCommands> {
            Err(crate::error::Error::Parse("test error".to_string()))
        }
    }

    #[test]
    fn propagates_error_from_verify_commands() {
        let adapter = FailingAdapter;

        let result = generate_prompt("test", &adapter);

        assert!(result.is_err());
    }
}
