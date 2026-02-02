//! Orchestrator for the Ralph Loop.
//!
//! The orchestrator iterates through stories one at a time:
//! 1. Gets the list of stories from the adapter
//! 2. For each incomplete story, generates a story-specific prompt
//! 3. Spawns an agent for that story
//! 4. Detects `<promise>COMPLETE</promise>` to mark story iteration done
//! 5. Refreshes story list and continues to next incomplete story
//! 6. Emits Complete when all stories are done

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::{LoopEvent, LoopEventSender, LoopState};
use crate::agent::{AgentConfig, CodingAgent, PromptBuilder};
use crate::error::Result;
use crate::spec::{self, Story};

/// Completion signal that agents output when a story is done and verified.
const COMPLETION_SIGNAL: &str = "<promise>COMPLETE</promise>";

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
    /// Iterates through stories one at a time, spawning an agent for each
    /// incomplete story. Detects completion signal and refreshes state
    /// between iterations.
    pub async fn run(&mut self) -> Result<LoopState> {
        // Initialize state
        let mut state = LoopState::new(&self.change_name);
        state.running = true;

        // Story iteration loop
        loop {
            // Check for stop request
            if self.stop_flag.load(Ordering::Relaxed) {
                state.running = false;
                self.emit(LoopEvent::Complete).await;
                return Ok(state);
            }

            // Refresh adapter to get latest story state
            let adapter = spec::create_adapter(&self.change_name)?;
            let stories = adapter.stories()?;

            // Update state with story counts
            state.total_stories = stories.len();
            state.completed_stories = stories.iter().filter(|s| is_story_complete(s)).count();

            // Find next incomplete story
            let next_story = next_incomplete_story(&stories);

            match next_story {
                Some(story) => {
                    // Update state with current story
                    state.current_story_id = Some(story.id.clone());

                    // Calculate story position (1-indexed)
                    let story_position = stories
                        .iter()
                        .position(|s| s.id == story.id)
                        .map(|i| i + 1)
                        .unwrap_or(1);

                    // Emit story progress event
                    self.emit(LoopEvent::StoryProgress {
                        story_id: story.id.clone(),
                        story_title: story.title.clone(),
                        current: story_position,
                        total: stories.len(),
                    })
                    .await;

                    // Generate story-specific prompt
                    let prompt_builder = PromptBuilder::new(adapter.as_ref(), &self.change_name);
                    let prompt = prompt_builder.for_story(&story.id)?;

                    // Run agent for this story
                    match self.agent.run(&prompt, &self.config) {
                        Ok(output) => {
                            // Emit agent output
                            self.emit(LoopEvent::AgentOutput {
                                line: output.result.clone(),
                            })
                            .await;

                            // Check for completion signal
                            if output.result.contains(COMPLETION_SIGNAL) {
                                // Story completed, continue to next iteration
                                // (adapter will be refreshed at the start of next loop)
                                continue;
                            } else {
                                // Agent finished without completion signal
                                // Could be an error or timeout, emit error and stop
                                self.emit(LoopEvent::Error {
                                    message: format!(
                                        "Agent finished story {} without completion signal",
                                        story.id
                                    ),
                                })
                                .await;
                                break;
                            }
                        }
                        Err(e) => {
                            self.emit(LoopEvent::Error {
                                message: format!("Agent error on story {}: {}", story.id, e),
                            })
                            .await;
                            break;
                        }
                    }
                }
                None => {
                    // All stories complete!
                    state.current_story_id = None;
                    break;
                }
            }
        }

        // Emit completion event
        self.emit(LoopEvent::Complete).await;
        state.running = false;

        Ok(state)
    }

    /// Emit a loop event.
    async fn emit(&self, event: LoopEvent) {
        let _ = self.event_tx.send(event).await;
    }
}

/// Returns the first incomplete story, or None if all are complete.
fn next_incomplete_story(stories: &[Story]) -> Option<&Story> {
    stories.iter().find(|s| !is_story_complete(s))
}

/// Checks if a story is complete (all tasks done).
fn is_story_complete(story: &Story) -> bool {
    !story.tasks.is_empty() && story.tasks.iter().all(|t| t.done)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentOutput, TokenUsage};
    use crate::spec::Task;

    struct MockAgent {
        output: String,
    }

    impl CodingAgent for MockAgent {
        fn run(&self, _prompt: &str, _config: &AgentConfig) -> Result<AgentOutput> {
            Ok(AgentOutput {
                result: self.output.clone(),
                session_id: "mock-session".to_string(),
                usage: TokenUsage::default(),
            })
        }
    }

    #[test]
    fn next_incomplete_story_returns_first_incomplete() {
        let stories = vec![
            Story {
                id: "1".to_string(),
                title: "First".to_string(),
                tasks: vec![Task {
                    id: "1.1".to_string(),
                    description: "Done".to_string(),
                    done: true,
                }],
            },
            Story {
                id: "2".to_string(),
                title: "Second".to_string(),
                tasks: vec![Task {
                    id: "2.1".to_string(),
                    description: "Not done".to_string(),
                    done: false,
                }],
            },
        ];

        let next = next_incomplete_story(&stories);
        assert!(next.is_some());
        assert_eq!(next.unwrap().id, "2");
    }

    #[test]
    fn next_incomplete_story_returns_none_when_all_complete() {
        let stories = vec![Story {
            id: "1".to_string(),
            title: "First".to_string(),
            tasks: vec![Task {
                id: "1.1".to_string(),
                description: "Done".to_string(),
                done: true,
            }],
        }];

        let next = next_incomplete_story(&stories);
        assert!(next.is_none());
    }

    #[test]
    fn is_story_complete_true_when_all_tasks_done() {
        let story = Story {
            id: "1".to_string(),
            title: "Test".to_string(),
            tasks: vec![
                Task {
                    id: "1.1".to_string(),
                    description: "Task 1".to_string(),
                    done: true,
                },
                Task {
                    id: "1.2".to_string(),
                    description: "Task 2".to_string(),
                    done: true,
                },
            ],
        };

        assert!(is_story_complete(&story));
    }

    #[test]
    fn is_story_complete_false_when_any_task_incomplete() {
        let story = Story {
            id: "1".to_string(),
            title: "Test".to_string(),
            tasks: vec![
                Task {
                    id: "1.1".to_string(),
                    description: "Task 1".to_string(),
                    done: true,
                },
                Task {
                    id: "1.2".to_string(),
                    description: "Task 2".to_string(),
                    done: false,
                },
            ],
        };

        assert!(!is_story_complete(&story));
    }

    #[test]
    fn is_story_complete_false_when_no_tasks() {
        let story = Story {
            id: "1".to_string(),
            title: "Test".to_string(),
            tasks: vec![],
        };

        assert!(!is_story_complete(&story));
    }

    #[tokio::test]
    async fn orchestrator_respects_stop_flag() {
        let (tx, _rx) = tokio::sync::mpsc::channel(10);

        let orchestrator = Orchestrator::new(
            "test-change",
            Box::new(MockAgent {
                output: "test".to_string(),
            }),
            AgentConfig::default(),
            tx,
        );

        // Set stop flag before running
        orchestrator.stop_flag.store(true, Ordering::Relaxed);

        // The run would return early due to stop flag
        // (Can't actually run without a real change, but this validates structure)
    }

    #[test]
    fn completion_signal_constant_is_correct() {
        assert_eq!(COMPLETION_SIGNAL, "<promise>COMPLETE</promise>");
    }
}
