//! Instruction generation for coding agents.
//!
//! This module generates markdown prompts from spec layer context
//! that are suitable for AI coding agents.

use crate::error::Result;
use crate::spec::{Context, SpecAdapter};

/// Generates AI instructions from spec layer context for a given story.
///
/// The generated markdown includes:
/// - Proposal summary
/// - Design decisions
/// - Current story and tasks
/// - Relevant scenarios
/// - Verification commands
pub fn generate_instructions(adapter: &dyn SpecAdapter, story_id: &str) -> Result<String> {
    let context = adapter.context(story_id)?;
    Ok(build_markdown(&context))
}

fn build_markdown(context: &Context) -> String {
    let mut sections = Vec::new();

    // Header
    sections.push(format!(
        "# Story {}: {}\n",
        context.story.id, context.story.title
    ));

    // Proposal section
    sections.push("## Proposal\n".to_string());
    sections.push(context.proposal.clone());
    sections.push(String::new());

    // Design section
    sections.push("## Design\n".to_string());
    sections.push(context.design.clone());
    sections.push(String::new());

    // Tasks section
    sections.push("## Tasks\n".to_string());
    for task in &context.story.tasks {
        let checkbox = if task.done { "[x]" } else { "[ ]" };
        sections.push(format!("- {} {} {}", checkbox, task.id, task.description));
    }
    sections.push(String::new());

    // Scenarios section (if any)
    if !context.scenarios.is_empty() {
        sections.push("## Scenarios\n".to_string());
        for scenario in &context.scenarios {
            sections.push(format!("### {}\n", scenario.name));
            if !scenario.given.is_empty() {
                sections.push("**Given:**".to_string());
                for given in &scenario.given {
                    sections.push(format!("- {}", given));
                }
            }
            sections.push(format!("**When:** {}", scenario.when));
            sections.push("**Then:**".to_string());
            for then_step in &scenario.then {
                sections.push(format!("- {}", then_step));
            }
            sections.push(String::new());
        }
    }

    // Verification section
    sections.push("## Verification\n".to_string());
    if !context.verify.checks.is_empty() {
        sections.push("**Checks:**".to_string());
        for check in &context.verify.checks {
            sections.push(format!("```bash\n{}\n```", check));
        }
    }
    if !context.verify.tests.is_empty() {
        sections.push(format!("**Tests:**\n```bash\n{}\n```", context.verify.tests));
    }
    sections.push(String::new());

    // Available commands section
    sections.push("## Available Commands\n".to_string());
    sections.push("Use these ralphtool commands to manage your progress:\n".to_string());
    sections.push("- `ralphtool agent context` - Get context for current story".to_string());
    sections.push("- `ralphtool agent task done <task_id>` - Mark a task as complete".to_string());
    sections.push("- `ralphtool agent status` - Check current progress".to_string());
    sections.push("- `ralphtool agent learn \"<description>\"` - Record a learning".to_string());
    sections.push(String::new());

    // Instructions
    sections.push("## Instructions\n".to_string());
    sections.push("Complete all tasks in this story. For each task:".to_string());
    sections.push("1. Implement the required changes".to_string());
    sections.push("2. Run verification checks".to_string());
    sections.push("3. Mark the task complete with `ralphtool agent task done <task_id>`".to_string());
    sections.push(String::new());

    sections.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::{Scenario, Story, Task, VerifyCommands};

    #[test]
    fn generates_markdown_with_all_sections() {
        let context = Context {
            story: Story {
                id: "1".to_string(),
                title: "Implement feature".to_string(),
                tasks: vec![
                    Task {
                        id: "1.1".to_string(),
                        description: "First task".to_string(),
                        done: false,
                    },
                    Task {
                        id: "1.2".to_string(),
                        description: "Second task".to_string(),
                        done: true,
                    },
                ],
            },
            proposal: "This is the proposal.".to_string(),
            design: "This is the design.".to_string(),
            scenarios: vec![Scenario {
                name: "Test scenario".to_string(),
                story_id: "1".to_string(),
                given: vec!["Precondition".to_string()],
                when: "Action happens".to_string(),
                then: vec!["Expected result".to_string()],
            }],
            verify: VerifyCommands {
                checks: vec!["cargo check".to_string()],
                tests: "cargo test".to_string(),
            },
        };

        let markdown = build_markdown(&context);

        assert!(markdown.contains("# Story 1: Implement feature"));
        assert!(markdown.contains("## Proposal"));
        assert!(markdown.contains("This is the proposal."));
        assert!(markdown.contains("## Design"));
        assert!(markdown.contains("This is the design."));
        assert!(markdown.contains("## Tasks"));
        assert!(markdown.contains("- [ ] 1.1 First task"));
        assert!(markdown.contains("- [x] 1.2 Second task"));
        assert!(markdown.contains("## Scenarios"));
        assert!(markdown.contains("### Test scenario"));
        assert!(markdown.contains("**Given:**"));
        assert!(markdown.contains("**When:** Action happens"));
        assert!(markdown.contains("**Then:**"));
        assert!(markdown.contains("## Verification"));
        assert!(markdown.contains("cargo check"));
        assert!(markdown.contains("cargo test"));
        assert!(markdown.contains("## Available Commands"));
        assert!(markdown.contains("ralphtool agent task done"));
    }
}
