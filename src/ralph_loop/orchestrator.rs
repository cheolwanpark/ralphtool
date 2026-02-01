//! Orchestrator for the Ralph Loop.
//!
//! The orchestrator drives the main loop by:
//! 1. Initializing a scoped session with exclusive lock
//! 2. Iterating through incomplete stories via session
//! 3. Generating instructions for each story
//! 4. Spawning an agent with session environment variables
//! 5. Parsing output to detect task completions
//! 6. Flushing learnings on completion

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use regex::Regex;

use super::{LoopEvent, LoopEventSender, LoopState};
use crate::agent::{AgentConfig, AgentOutput, CodingAgent};
use crate::error::Result;
use crate::session::instructions::generate_instructions;
use crate::session::ScopedSession;
use crate::spec;

/// Orchestrator for the Ralph Loop.
pub struct Orchestrator {
    /// Name of the change being processed.
    change_name: String,

    /// Coding agent to use.
    agent: Box<dyn CodingAgent>,

    /// Agent configuration.
    config: AgentConfig,

    /// Event sender for TUI updates.
    event_tx: LoopEventSender,

    /// Flag to stop the loop.
    stop_flag: Arc<AtomicBool>,
}

impl Orchestrator {
    /// Create a new orchestrator.
    pub fn new(
        change_name: &str,
        agent: Box<dyn CodingAgent>,
        config: AgentConfig,
        event_tx: LoopEventSender,
    ) -> Self {
        Self {
            change_name: change_name.to_string(),
            agent,
            config,
            event_tx,
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Get a handle to stop the loop.
    pub fn stop_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.stop_flag)
    }

    /// Run the orchestration loop.
    ///
    /// Initializes a scoped session, iterates through incomplete stories,
    /// and spawns an agent for each with session environment variables.
    /// Returns the final loop state.
    pub async fn run(&mut self) -> Result<LoopState> {
        // Initialize scoped session (acquires exclusive lock)
        let mut session = ScopedSession::init(&self.change_name)?;

        // Get initial story count
        let adapter = spec::create_adapter(&self.change_name)?;
        let stories = adapter.stories()?;

        let mut state = LoopState::new(&self.change_name);
        state.stories_total = stories.len();
        state.stories_completed = stories.iter().filter(|s| s.is_complete()).count();
        state.running = true;

        // Process stories via session iteration
        while let Some(story_id) = session.next_story()? {
            // Check for stop request
            if self.stop_flag.load(Ordering::Relaxed) {
                break;
            }

            // Create adapter to get story details
            let mut adapter = spec::create_adapter(&self.change_name)?;
            let stories = adapter.stories()?;
            let story = stories.iter().find(|s| s.id == story_id);

            let story = match story {
                Some(s) => s,
                None => continue,
            };

            // Update state
            state.current_story = Some(story.id.clone());
            state.tasks_total = story.tasks.len();
            state.tasks_completed = story.tasks.iter().filter(|t| t.done).count();

            // Emit story started event
            self.emit(LoopEvent::StoryStarted {
                story_id: story.id.clone(),
                title: story.title.clone(),
            })
            .await;

            // Generate instructions for this story
            let instructions = generate_instructions(adapter.as_ref(), &story.id)?;

            // Create agent config with session environment variables
            let agent_config = AgentConfig::new()
                .with_env(session.env());
            let agent_config = AgentConfig {
                max_turns: self.config.max_turns,
                timeout: self.config.timeout,
                env: agent_config.env,
            };

            // Run the agent with session environment
            match self.agent.run(&instructions, &agent_config) {
                Ok(output) => {
                    // Parse output for task completions
                    let completed_tasks = self.parse_task_completions(&output);

                    // Mark tasks as done
                    for task_id in &completed_tasks {
                        if let Err(e) = adapter.mark_done(task_id) {
                            self.emit(LoopEvent::Error {
                                message: format!("Failed to mark task {} done: {}", task_id, e),
                            })
                            .await;
                        } else {
                            self.emit(LoopEvent::TaskCompleted {
                                task_id: task_id.clone(),
                            })
                            .await;
                            state.tasks_completed += 1;
                        }
                    }

                    // Emit agent output
                    self.emit(LoopEvent::AgentOutput {
                        line: output.result.clone(),
                    })
                    .await;
                }
                Err(e) => {
                    self.emit(LoopEvent::Error {
                        message: format!("Agent error: {}", e),
                    })
                    .await;
                }
            }

            // Reload adapter to check story completion
            let updated_adapter = spec::create_adapter(&self.change_name)?;
            let updated_stories = updated_adapter.stories()?;

            if let Some(updated) = updated_stories.iter().find(|s| s.id == story_id) {
                if updated.is_complete() {
                    self.emit(LoopEvent::StoryCompleted {
                        story_id: story_id.clone(),
                    })
                    .await;
                    state.stories_completed += 1;
                }
            }
        }

        // Flush session (releases lock, cleans up session file)
        // Learnings would be passed here if accumulated
        if let Err(e) = session.flush(&[]) {
            self.emit(LoopEvent::Error {
                message: format!("Failed to flush session: {}", e),
            })
            .await;
        }

        // Emit completion event
        self.emit(LoopEvent::Complete).await;

        state.running = false;
        state.current_story = None;

        Ok(state)
    }

    /// Parse agent output for task completions.
    ///
    /// Looks for patterns like:
    /// - "ralphtool agent task done 1.1"
    /// - Task IDs mentioned after "completed" or "done"
    fn parse_task_completions(&self, output: &AgentOutput) -> Vec<String> {
        let mut completed = Vec::new();

        // Pattern: ralphtool agent task done <task_id>
        let re = Regex::new(r"ralphtool\s+agent\s+task\s+done\s+(\d+\.\d+)").unwrap();
        for cap in re.captures_iter(&output.result) {
            if let Some(task_id) = cap.get(1) {
                completed.push(task_id.as_str().to_string());
            }
        }

        // Deduplicate
        completed.sort();
        completed.dedup();

        completed
    }

    /// Emit a loop event.
    async fn emit(&self, event: LoopEvent) {
        let _ = self.event_tx.send(event).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::TokenUsage;

    #[test]
    fn parses_task_completions_from_output() {
        let output = AgentOutput {
            result: "I ran ralphtool agent task done 1.1 and then ralphtool agent task done 1.2"
                .to_string(),
            session_id: String::new(),
            usage: TokenUsage::default(),
        };

        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        let orchestrator = Orchestrator::new(
            "test",
            Box::new(MockAgent),
            AgentConfig::default(),
            tx,
        );

        let tasks = orchestrator.parse_task_completions(&output);
        assert_eq!(tasks, vec!["1.1", "1.2"]);
    }

    #[test]
    fn deduplicates_task_completions() {
        let output = AgentOutput {
            result: "ralphtool agent task done 1.1 ... ralphtool agent task done 1.1".to_string(),
            session_id: String::new(),
            usage: TokenUsage::default(),
        };

        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        let orchestrator = Orchestrator::new(
            "test",
            Box::new(MockAgent),
            AgentConfig::default(),
            tx,
        );

        let tasks = orchestrator.parse_task_completions(&output);
        assert_eq!(tasks, vec!["1.1"]);
    }

    struct MockAgent;

    impl CodingAgent for MockAgent {
        fn run(&self, _prompt: &str, _config: &AgentConfig) -> Result<AgentOutput> {
            Ok(AgentOutput {
                result: "Mock output".to_string(),
                session_id: "mock-session".to_string(),
                usage: TokenUsage::default(),
            })
        }
    }
}
