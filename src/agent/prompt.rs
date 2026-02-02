//! Prompt generation for coding agents.
//!
//! This module generates story-specific prompts for AI coding agents.
//! The prompt tells the agent how to work on a single story of a change,
//! with relevant scenarios and completion signal instructions.

use crate::error::Result;
use crate::spec::{Scenario, SpecAdapter, Story};

/// Builder for generating story-specific agent prompts.
pub struct PromptBuilder<'a> {
    adapter: &'a dyn SpecAdapter,
    #[allow(dead_code)]
    change_name: String,
}

impl<'a> PromptBuilder<'a> {
    /// Create a new PromptBuilder for a change.
    pub fn new(adapter: &'a dyn SpecAdapter, change_name: &str) -> Self {
        Self {
            adapter,
            change_name: change_name.to_string(),
        }
    }

    /// Generate a prompt for working on a specific story.
    ///
    /// The prompt includes:
    /// - Story ID and title
    /// - Tasks belonging to this story
    /// - All scenarios with instruction to focus on relevant ones
    /// - Spec tool usage instructions from adapter
    /// - Completion signal instructions (`<promise>COMPLETE</promise>`)
    pub fn for_story(&self, story_id: &str) -> Result<String> {
        let context = self.adapter.context(story_id)?;
        let all_scenarios = self.adapter.scenarios()?;

        let mut sections = Vec::new();

        // Header
        sections.push(format!(
            "# Working on Story {}: {}\n",
            context.story.id, context.story.title
        ));

        // Story scope instruction
        sections.push("## Your Task\n".to_string());
        sections.push(format!(
            "Complete all tasks in **Story {} only**. Do not work on other stories.",
            context.story.id
        ));
        sections.push("The orchestrator will handle the next story after you complete this one.\n".to_string());

        // Tasks for this story
        sections.push("## Tasks to Complete\n".to_string());
        sections.push(self.format_tasks(&context.story));

        // Context files
        sections.push("## Context\n".to_string());
        sections.push(
            "Read the proposal and design to understand the change:\n- Proposal: motivation and scope\n- Design: technical decisions\n".to_string()
        );

        // Scenarios
        sections.push("## Verification Scenarios\n".to_string());
        sections.push(
            "Focus on scenarios relevant to this story's tasks. \
             You don't need to verify unrelated scenarios.\n"
                .to_string(),
        );
        sections.push(self.format_scenarios(&all_scenarios));

        // Spec tool usage instructions
        sections.push(self.adapter.tool_prompt());

        // Completion signal instructions
        sections.push("\n## Completion Signal\n".to_string());
        sections.push("After completing all tasks in this story:\n".to_string());
        sections.push("1. Run verification commands (cargo check, cargo clippy, cargo test)".to_string());
        sections.push("2. If all verification passes, output: `<promise>COMPLETE</promise>`".to_string());
        sections.push("3. If verification fails, fix issues and re-verify before signaling\n".to_string());
        sections.push("**Important**: Only output `<promise>COMPLETE</promise>` after ALL tasks in this story are done AND verification passes.".to_string());

        Ok(sections.join("\n"))
    }

    /// Format tasks for display in the prompt.
    fn format_tasks(&self, story: &Story) -> String {
        let mut lines = Vec::new();
        for task in &story.tasks {
            let checkbox = if task.done { "[x]" } else { "[ ]" };
            lines.push(format!("- {} {} {}", checkbox, task.id, task.description));
        }
        if lines.is_empty() {
            lines.push("(No tasks defined)".to_string());
        }
        lines.push(String::new()); // Trailing newline
        lines.join("\n")
    }

    /// Format scenarios for display in the prompt.
    fn format_scenarios(&self, scenarios: &[Scenario]) -> String {
        if scenarios.is_empty() {
            return "(No scenarios defined)\n".to_string();
        }

        let mut lines = Vec::new();
        for scenario in scenarios {
            lines.push(format!(
                "### {} ({})\n",
                scenario.name, scenario.capability
            ));

            if !scenario.given.is_empty() {
                for given in &scenario.given {
                    lines.push(format!("- **GIVEN** {}", given));
                }
            }
            if !scenario.when.is_empty() {
                lines.push(format!("- **WHEN** {}", scenario.when));
            }
            for then in &scenario.then {
                lines.push(format!("- **THEN** {}", then));
            }
            lines.push(String::new());
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::{Context, Task, VerifyCommands};

    struct MockAdapter {
        story: Story,
        scenarios: Vec<Scenario>,
    }

    impl SpecAdapter for MockAdapter {
        fn stories(&self) -> Result<Vec<Story>> {
            Ok(vec![self.story.clone()])
        }

        fn scenarios(&self) -> Result<Vec<Scenario>> {
            Ok(self.scenarios.clone())
        }

        fn context(&self, _story_id: &str) -> Result<Context> {
            Ok(Context {
                story: self.story.clone(),
                proposal: "Test proposal".to_string(),
                design: "Test design".to_string(),
                scenarios: self.scenarios.clone(),
                verify: VerifyCommands {
                    checks: vec!["cargo check".to_string()],
                    tests: "cargo test".to_string(),
                },
            })
        }

        fn verify_commands(&self) -> Result<VerifyCommands> {
            Ok(VerifyCommands {
                checks: vec!["cargo check".to_string()],
                tests: "cargo test".to_string(),
            })
        }

        fn tool_prompt(&self) -> String {
            "## Tool Instructions\nMock tool instructions".to_string()
        }
    }

    #[test]
    fn for_story_includes_story_header() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![],
        };

        let builder = PromptBuilder::new(&adapter, "test-change");
        let prompt = builder.for_story("1").unwrap();

        assert!(prompt.contains("Story 1: Test Story"));
    }

    #[test]
    fn for_story_includes_tasks() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
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
            scenarios: vec![],
        };

        let builder = PromptBuilder::new(&adapter, "test-change");
        let prompt = builder.for_story("1").unwrap();

        assert!(prompt.contains("[ ] 1.1 First task"));
        assert!(prompt.contains("[x] 1.2 Second task"));
    }

    #[test]
    fn for_story_includes_completion_signal() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![],
        };

        let builder = PromptBuilder::new(&adapter, "test-change");
        let prompt = builder.for_story("1").unwrap();

        assert!(prompt.contains("<promise>COMPLETE</promise>"));
    }

    #[test]
    fn for_story_includes_scenario_focus_instruction() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![Scenario {
                name: "Test Scenario".to_string(),
                capability: "test-capability".to_string(),
                requirement_id: "req-1".to_string(),
                given: vec!["a condition".to_string()],
                when: "action happens".to_string(),
                then: vec!["result occurs".to_string()],
            }],
        };

        let builder = PromptBuilder::new(&adapter, "test-change");
        let prompt = builder.for_story("1").unwrap();

        assert!(prompt.contains("Focus on scenarios relevant to this story"));
        assert!(prompt.contains("Test Scenario"));
    }

    #[test]
    fn for_story_includes_tool_prompt() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![],
        };

        let builder = PromptBuilder::new(&adapter, "test-change");
        let prompt = builder.for_story("1").unwrap();

        assert!(prompt.contains("Mock tool instructions"));
    }

    #[test]
    fn for_story_scopes_to_single_story() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![],
        };

        let builder = PromptBuilder::new(&adapter, "test-change");
        let prompt = builder.for_story("1").unwrap();

        assert!(prompt.contains("Story 1 only"));
        assert!(prompt.contains("Do not work on other stories"));
    }
}
