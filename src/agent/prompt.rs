//! Prompt generation for coding agents.
//!
//! This module generates story-specific prompts for AI coding agents.
//! The prompt tells the agent how to work on a single story of a change,
//! with relevant scenarios and completion signal instructions.

use super::Prompt;
use crate::error::Result;
use crate::ralph_loop::learnings::learnings_path;
use crate::spec::{Scenario, SpecAdapter, Story};

/// Builder for generating story-specific agent prompts.
pub struct PromptBuilder<'a> {
    adapter: &'a dyn SpecAdapter,
    change_name: String,
    /// Optional learnings content to include in the prompt.
    learnings_content: Option<String>,
}

impl<'a> PromptBuilder<'a> {
    /// Create a new PromptBuilder for a change.
    pub fn new(adapter: &'a dyn SpecAdapter, change_name: &str) -> Self {
        Self {
            adapter,
            change_name: change_name.to_string(),
            learnings_content: None,
        }
    }

    /// Set optional learnings content to include in prompts.
    ///
    /// When set, prompts will include a "Shared Learnings" section with:
    /// - Instructions on what to record (discoveries, decisions, gotchas)
    /// - The path to the learnings file
    /// - The existing learnings content
    pub fn with_learnings(mut self, learnings_content: Option<String>) -> Self {
        self.learnings_content = learnings_content;
        self
    }

    /// Generate a prompt for working on a specific story.
    ///
    /// The prompt includes:
    /// - Story ID and title
    /// - Tasks belonging to this story
    /// - All scenarios with instruction to focus on relevant ones
    /// - Spec tool usage instructions from adapter
    /// - Completion signal instructions (`<promise>COMPLETE</promise>`)
    ///
    /// This is a convenience wrapper around [`for_story_with_retry_context`] with no retry context.
    #[allow(dead_code)] // Public API - used by tests and external callers
    pub fn for_story(&self, story_id: &str) -> Result<Prompt> {
        self.for_story_with_retry_context(story_id, None)
    }

    /// Generate a prompt for working on a specific story with optional retry context.
    ///
    /// When `retry_reason` is `Some`, includes a "Previous Attempt Failed" section
    /// explaining why the last attempt failed, helping the agent avoid the same issues.
    ///
    /// The prompt includes:
    /// - Story ID and title
    /// - Previous attempt failure reason (if retrying with explicit FAILED signal)
    /// - Tasks belonging to this story
    /// - All scenarios with instruction to focus on relevant ones
    /// - Spec tool usage instructions from adapter
    /// - Completion and failure signal instructions
    pub fn for_story_with_retry_context(
        &self,
        story_id: &str,
        retry_reason: Option<String>,
    ) -> Result<Prompt> {
        let context = self.adapter.context(story_id)?;
        let all_scenarios = self.adapter.scenarios()?;

        let mut sections = Vec::new();

        // Header
        sections.push(format!(
            "# Working on Story {}: {}\n",
            context.story.id, context.story.title
        ));

        // Previous Attempt Failed section (only on retries with explicit FAILED signal)
        if let Some(reason) = retry_reason {
            sections.push("## Previous Attempt Failed\n".to_string());
            sections.push(format!(
                "The previous attempt failed with the following reason:\n> {}\n",
                reason
            ));
            sections.push(
                "Please address this issue in your current attempt. \
                 Review the failure reason and try a different approach if needed.\n"
                    .to_string(),
            );
        }

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

        // Shared Learnings section (only when learnings content exists)
        if let Some(ref content) = self.learnings_content {
            sections.push("## Shared Learnings\n".to_string());
            sections.push(
                "Previous stories have recorded the following learnings. \
                 Use this information to avoid repeating work and maintain consistency.\n"
                    .to_string(),
            );
            sections.push(format!(
                "**What to record**: As you work, add discoveries, decisions, and gotchas \
                 to the learnings file at `{}`.\n",
                learnings_path(&self.change_name).display()
            ));
            sections.push("### Current Learnings\n".to_string());
            sections.push(format!("```markdown\n{}\n```\n", content));
        }

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
        sections.push("**Important**: Only output `<promise>COMPLETE</promise>` after ALL tasks in this story are done AND verification passes.\n".to_string());

        // Failure signal instructions
        sections.push("## Failure Signal\n".to_string());
        sections.push("If you cannot complete the story after multiple attempts:\n".to_string());
        sections.push("- Output: `<promise>FAILED: {reason}</promise>` where `{reason}` explains why completion is not possible".to_string());
        sections.push("- The orchestrator will revert changes, include your reason in the next retry prompt, and try again".to_string());
        sections.push("- Use this for: unresolvable test failures, missing dependencies, unclear requirements, blocked tasks\n".to_string());
        sections.push("**Note**: Prefer fixing issues and completing. Only use FAILED when you truly cannot proceed.".to_string());

        Ok(Prompt {
            system: String::new(),
            user: sections.join("\n"),
        })
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

        assert!(prompt.user.contains("Story 1: Test Story"));
        assert!(prompt.system.is_empty());
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

        assert!(prompt.user.contains("[ ] 1.1 First task"));
        assert!(prompt.user.contains("[x] 1.2 Second task"));
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

        assert!(prompt.user.contains("<promise>COMPLETE</promise>"));
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

        assert!(prompt.user.contains("Focus on scenarios relevant to this story"));
        assert!(prompt.user.contains("Test Scenario"));
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

        assert!(prompt.user.contains("Mock tool instructions"));
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

        assert!(prompt.user.contains("Story 1 only"));
        assert!(prompt.user.contains("Do not work on other stories"));
    }

    #[test]
    fn for_story_includes_failure_signal_instructions() {
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

        assert!(prompt.user.contains("## Failure Signal"));
        assert!(prompt.user.contains("<promise>FAILED:"));
        assert!(prompt.user.contains("</promise>"));
    }

    #[test]
    fn for_story_with_retry_context_includes_failure_reason() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![],
        };

        let builder = PromptBuilder::new(&adapter, "test-change");
        let prompt = builder
            .for_story_with_retry_context("1", Some("Tests failed due to missing mock".to_string()))
            .unwrap();

        assert!(prompt.user.contains("## Previous Attempt Failed"));
        assert!(prompt.user.contains("Tests failed due to missing mock"));
        assert!(prompt.user.contains("address this issue"));
    }

    #[test]
    fn for_story_with_retry_context_none_has_no_failure_section() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![],
        };

        let builder = PromptBuilder::new(&adapter, "test-change");
        let prompt = builder.for_story_with_retry_context("1", None).unwrap();

        assert!(!prompt.user.contains("## Previous Attempt Failed"));
    }

    #[test]
    fn for_story_delegates_to_for_story_with_retry_context() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![],
        };

        let builder = PromptBuilder::new(&adapter, "test-change");
        let prompt1 = builder.for_story("1").unwrap();
        let prompt2 = builder.for_story_with_retry_context("1", None).unwrap();

        // Both should produce the same output
        assert_eq!(prompt1.user, prompt2.user);
    }

    #[test]
    fn for_story_with_learnings_includes_learnings_section() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![],
        };

        let learnings = "## Story 1 Learnings\n- Discovered pattern X\n- Decided on approach Y";
        let builder = PromptBuilder::new(&adapter, "test-change")
            .with_learnings(Some(learnings.to_string()));
        let prompt = builder.for_story("1").unwrap();

        assert!(prompt.user.contains("## Shared Learnings"));
        assert!(prompt.user.contains("discoveries, decisions, and gotchas"));
        assert!(prompt.user.contains("test-change-learnings.md"));
        assert!(prompt.user.contains("Discovered pattern X"));
        assert!(prompt.user.contains("Decided on approach Y"));
    }

    #[test]
    fn for_story_without_learnings_omits_learnings_section() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![],
        };

        let builder = PromptBuilder::new(&adapter, "test-change")
            .with_learnings(None);
        let prompt = builder.for_story("1").unwrap();

        assert!(!prompt.user.contains("## Shared Learnings"));
        assert!(!prompt.user.contains("learnings file"));
    }

    #[test]
    fn for_story_default_has_no_learnings() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![],
        };

        // Default builder (without calling with_learnings)
        let builder = PromptBuilder::new(&adapter, "test-change");
        let prompt = builder.for_story("1").unwrap();

        assert!(!prompt.user.contains("## Shared Learnings"));
    }

    #[test]
    fn learnings_section_appears_before_verification_scenarios() {
        let adapter = MockAdapter {
            story: Story {
                id: "1".to_string(),
                title: "Test Story".to_string(),
                tasks: vec![],
            },
            scenarios: vec![],
        };

        let learnings = "## Some Learnings";
        let builder = PromptBuilder::new(&adapter, "test-change")
            .with_learnings(Some(learnings.to_string()));
        let prompt = builder.for_story("1").unwrap();

        // Find positions of both sections
        let learnings_pos = prompt.user.find("## Shared Learnings").unwrap();
        let scenarios_pos = prompt.user.find("## Verification Scenarios").unwrap();

        // Learnings should appear before scenarios
        assert!(learnings_pos < scenarios_pos);
    }
}
