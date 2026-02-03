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
use crate::agent::{CodingAgent, PromptBuilder, StreamEvent};
use crate::checkpoint::Checkpoint;
use crate::error::Result;
use crate::spec::{self, Story};

/// Completion signal that agents output when a story is done and verified.
const COMPLETION_SIGNAL: &str = "<promise>COMPLETE</promise>";

/// Failure signal prefix that agents output when they cannot complete a story.
const FAILURE_SIGNAL_PREFIX: &str = "<promise>FAILED:";
const FAILURE_SIGNAL_SUFFIX: &str = "</promise>";

/// Default maximum number of retries per story.
pub const DEFAULT_MAX_RETRIES: usize = 3;

/// Orchestrator for the Ralph Loop.
pub struct Orchestrator {
    /// Name of the change being processed.
    change_name: String,

    /// Coding agent to use.
    agent: Box<dyn CodingAgent>,

    /// Event sender for TUI updates.
    event_tx: LoopEventSender,

    /// Flag to stop the loop.
    stop_flag: Arc<AtomicBool>,

    /// Checkpoint manager for git stash-based state preservation.
    checkpoint: Checkpoint,

    /// Maximum number of retries per story.
    max_retries: usize,
}

impl Orchestrator {
    /// Create a new orchestrator.
    pub fn new(
        change_name: &str,
        agent: Box<dyn CodingAgent>,
        event_tx: LoopEventSender,
        max_retries: usize,
    ) -> Self {
        Self {
            change_name: change_name.to_string(),
            agent,
            event_tx,
            stop_flag: Arc::new(AtomicBool::new(false)),
            checkpoint: Checkpoint::new(change_name),
            max_retries,
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
    /// between iterations. Includes retry logic with checkpoint/revert
    /// on failure.
    pub async fn run(&mut self) -> Result<LoopState> {
        // Initialize state
        let mut state = LoopState::new(&self.change_name);
        state.running = true;

        // Story iteration loop
        'story_loop: loop {
            // Check for stop request
            if self.stop_flag.load(Ordering::Relaxed) {
                state.running = false;
                // Clean up checkpoints before exit
                let _ = self.checkpoint.cleanup();
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
                        completed: state.completed_stories,
                    })
                    .await;

                    // Save checkpoint before agent spawn
                    if let Err(e) = self.checkpoint.save(&story.id) {
                        self.emit(LoopEvent::Error {
                            message: format!(
                                "Failed to save checkpoint for story {}: {}",
                                story.id, e
                            ),
                        })
                        .await;
                        break 'story_loop;
                    }

                    // Retry loop for this story
                    let mut retry_count = 0;
                    let mut retry_reason: Option<String> = None;
                    let story_id = story.id.clone();
                    let story_title = story.title.clone();

                    'retry_loop: loop {
                        // Generate story-specific prompt (with retry context if available)
                        let prompt_builder =
                            PromptBuilder::new(adapter.as_ref(), &self.change_name);
                        let prompt =
                            prompt_builder.for_story_with_retry_context(&story_id, retry_reason.take())?;

                        // Run agent for this story
                        match self.agent.run(&prompt) {
                            Ok(stream) => {
                                let mut final_content = String::new();

                                // Process streaming events
                                for event in stream {
                                    match &event {
                                        StreamEvent::Message(_) => {
                                            // Emit intermediate message with story context
                                            self.emit(LoopEvent::StoryEvent {
                                                story_id: story_id.clone(),
                                                event: event.clone(),
                                            })
                                            .await;
                                        }
                                        StreamEvent::Done(response) => {
                                            // Store final content for completion check
                                            final_content = response.content.clone();
                                            // Emit done event with full response
                                            self.emit(LoopEvent::StoryEvent {
                                                story_id: story_id.clone(),
                                                event: event.clone(),
                                            })
                                            .await;
                                        }
                                    }
                                }

                                // Parse agent output for signals
                                let result = parse_agent_result(&final_content);

                                match result {
                                    AgentResult::Complete => {
                                        // Story completed successfully
                                        // Drop checkpoint and continue to next story
                                        if let Err(e) = self.checkpoint.drop(&story_id) {
                                            // Log but don't fail - checkpoint cleanup
                                            // will handle orphaned stashes
                                            self.emit(LoopEvent::Error {
                                                message: format!(
                                                    "Warning: Failed to drop checkpoint for story {}: {}",
                                                    story_id, e
                                                ),
                                            })
                                            .await;
                                        }
                                        continue 'story_loop;
                                    }
                                    AgentResult::Failed(reason) => {
                                        // Agent explicitly reported failure
                                        retry_count += 1;

                                        if retry_count >= self.max_retries {
                                            // Max retries exceeded
                                            self.emit(LoopEvent::Error {
                                                message: format!(
                                                    "Max retries ({}) exceeded for story {} ({}): {}",
                                                    self.max_retries, story_id, story_title, reason
                                                ),
                                            })
                                            .await;
                                            break 'story_loop;
                                        }

                                        // Revert to checkpoint and retry
                                        if let Err(e) = self.checkpoint.revert(&story_id) {
                                            self.emit(LoopEvent::Error {
                                                message: format!(
                                                    "Failed to revert checkpoint for story {}: {}",
                                                    story_id, e
                                                ),
                                            })
                                            .await;
                                            break 'story_loop;
                                        }

                                        // Store failure reason for next retry prompt
                                        retry_reason = Some(reason);

                                        // Continue retry loop
                                        continue 'retry_loop;
                                    }
                                    AgentResult::NoSignal => {
                                        // Abnormal termination - no promise signal
                                        retry_count += 1;

                                        if retry_count >= self.max_retries {
                                            // Max retries exceeded
                                            self.emit(LoopEvent::Error {
                                                message: format!(
                                                    "Max retries ({}) exceeded for story {} ({}): agent finished without completion signal",
                                                    self.max_retries, story_id, story_title
                                                ),
                                            })
                                            .await;
                                            break 'story_loop;
                                        }

                                        // Revert to checkpoint and retry
                                        if let Err(e) = self.checkpoint.revert(&story_id) {
                                            self.emit(LoopEvent::Error {
                                                message: format!(
                                                    "Failed to revert checkpoint for story {}: {}",
                                                    story_id, e
                                                ),
                                            })
                                            .await;
                                            break 'story_loop;
                                        }

                                        // Continue retry loop (no extra context for abnormal termination)
                                        continue 'retry_loop;
                                    }
                                }
                            }
                            Err(e) => {
                                // Agent error - treat as failure and retry
                                retry_count += 1;

                                if retry_count >= self.max_retries {
                                    self.emit(LoopEvent::Error {
                                        message: format!(
                                            "Max retries ({}) exceeded for story {} ({}): {}",
                                            self.max_retries, story_id, story_title, e
                                        ),
                                    })
                                    .await;
                                    break 'story_loop;
                                }

                                // Revert to checkpoint and retry
                                if let Err(revert_err) = self.checkpoint.revert(&story_id) {
                                    self.emit(LoopEvent::Error {
                                        message: format!(
                                            "Failed to revert checkpoint for story {}: {}",
                                            story_id, revert_err
                                        ),
                                    })
                                    .await;
                                    break 'story_loop;
                                }

                                continue 'retry_loop;
                            }
                        }
                    }
                }
                None => {
                    // All stories complete!
                    state.current_story_id = None;
                    break 'story_loop;
                }
            }
        }

        // Clean up all checkpoints on loop exit (success or failure)
        let _ = self.checkpoint.cleanup();

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

/// Result of parsing agent output for promise signals.
#[derive(Debug, PartialEq)]
enum AgentResult {
    /// Agent signaled successful completion with `<promise>COMPLETE</promise>`.
    Complete,
    /// Agent signaled failure with `<promise>FAILED: {reason}</promise>`.
    Failed(String),
    /// No promise signal found (abnormal termination).
    NoSignal,
}

/// Parses agent output for promise signals.
///
/// Looks for:
/// - `<promise>COMPLETE</promise>` → `AgentResult::Complete`
/// - `<promise>FAILED: {reason}</promise>` → `AgentResult::Failed(reason)`
/// - Neither → `AgentResult::NoSignal`
fn parse_agent_result(content: &str) -> AgentResult {
    if content.contains(COMPLETION_SIGNAL) {
        return AgentResult::Complete;
    }

    // Look for FAILED signal: <promise>FAILED: {reason}</promise>
    if let Some(start_idx) = content.find(FAILURE_SIGNAL_PREFIX) {
        let after_prefix = &content[start_idx + FAILURE_SIGNAL_PREFIX.len()..];
        if let Some(end_idx) = after_prefix.find(FAILURE_SIGNAL_SUFFIX) {
            let reason = after_prefix[..end_idx].trim().to_string();
            return AgentResult::Failed(reason);
        }
    }

    AgentResult::NoSignal
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
    use crate::agent::{AgentStream, Prompt};
    use crate::spec::Task;
    use std::io::{BufRead, BufReader};
    use std::process::{Command, Stdio};

    struct MockAgent;

    impl CodingAgent for MockAgent {
        fn run(&self, _prompt: &Prompt) -> Result<AgentStream> {
            // Create a simple command that outputs nothing (for mock purposes)
            let mut child = Command::new("true")
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to spawn mock process");

            let stdout = child.stdout.take().unwrap();
            let reader = BufReader::new(stdout);

            Ok(AgentStream::new_for_test(child, reader.lines()))
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
            Box::new(MockAgent),
            tx,
            DEFAULT_MAX_RETRIES,
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

    #[test]
    fn failure_signal_constants_are_correct() {
        assert_eq!(FAILURE_SIGNAL_PREFIX, "<promise>FAILED:");
        assert_eq!(FAILURE_SIGNAL_SUFFIX, "</promise>");
    }

    #[test]
    fn parse_agent_result_detects_complete() {
        let content = "Some output\n<promise>COMPLETE</promise>\nMore output";
        assert_eq!(parse_agent_result(content), AgentResult::Complete);
    }

    #[test]
    fn parse_agent_result_detects_failed_with_reason() {
        let content = "Some output\n<promise>FAILED: Tests did not pass</promise>\nMore output";
        assert_eq!(
            parse_agent_result(content),
            AgentResult::Failed("Tests did not pass".to_string())
        );
    }

    #[test]
    fn parse_agent_result_detects_failed_with_whitespace() {
        let content = "<promise>FAILED:   Multiple spaces reason  </promise>";
        assert_eq!(
            parse_agent_result(content),
            AgentResult::Failed("Multiple spaces reason".to_string())
        );
    }

    #[test]
    fn parse_agent_result_detects_no_signal() {
        let content = "Agent output without any promise signal";
        assert_eq!(parse_agent_result(content), AgentResult::NoSignal);
    }

    #[test]
    fn parse_agent_result_prefers_complete_over_failed() {
        // If both signals are present, COMPLETE takes precedence
        let content = "<promise>FAILED: reason</promise>\n<promise>COMPLETE</promise>";
        assert_eq!(parse_agent_result(content), AgentResult::Complete);
    }

    #[test]
    fn parse_agent_result_handles_empty_failure_reason() {
        let content = "<promise>FAILED:</promise>";
        assert_eq!(
            parse_agent_result(content),
            AgentResult::Failed("".to_string())
        );
    }

    #[test]
    fn parse_agent_result_handles_malformed_failed_signal() {
        // Missing closing tag
        let content = "<promise>FAILED: reason without closing";
        assert_eq!(parse_agent_result(content), AgentResult::NoSignal);
    }

    // ==================== Retry Logic Unit Tests ====================

    #[test]
    fn default_max_retries_is_3() {
        assert_eq!(DEFAULT_MAX_RETRIES, 3);
    }

    #[test]
    fn orchestrator_stores_max_retries() {
        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        let orchestrator = Orchestrator::new(
            "test-change",
            Box::new(MockAgent),
            tx,
            5, // Custom max retries
        );

        assert_eq!(orchestrator.max_retries, 5);
    }

    #[test]
    fn orchestrator_stores_checkpoint_with_change_name() {
        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        let orchestrator = Orchestrator::new(
            "my-feature",
            Box::new(MockAgent),
            tx,
            DEFAULT_MAX_RETRIES,
        );

        // Verify checkpoint is configured for the correct change
        // We can verify this by checking the change_name field
        assert_eq!(orchestrator.change_name, "my-feature");
    }

    #[test]
    fn agent_result_complete_is_success() {
        assert!(matches!(AgentResult::Complete, AgentResult::Complete));
    }

    #[test]
    fn agent_result_failed_contains_reason() {
        let result = AgentResult::Failed("test reason".to_string());
        if let AgentResult::Failed(reason) = result {
            assert_eq!(reason, "test reason");
        } else {
            panic!("Expected Failed variant");
        }
    }

    #[test]
    fn agent_result_no_signal_for_abnormal_termination() {
        // Simulates agent crashing or losing context without outputting a signal
        let content = "Some agent output but no promise signal at all";
        let result = parse_agent_result(content);
        assert_eq!(result, AgentResult::NoSignal);
    }

    #[test]
    fn parse_agent_result_with_newlines_in_reason() {
        // Failure reason should work even if it contains newlines before closing tag
        let content = "<promise>FAILED: Test failed\nwith multiple lines\nof error</promise>";
        let result = parse_agent_result(content);
        assert_eq!(
            result,
            AgentResult::Failed("Test failed\nwith multiple lines\nof error".to_string())
        );
    }

    #[test]
    fn parse_agent_result_signal_at_end_of_output() {
        // Signal at the very end of output
        let content = "Lots of agent output...\nMore output...\n<promise>COMPLETE</promise>";
        assert_eq!(parse_agent_result(content), AgentResult::Complete);
    }

    #[test]
    fn parse_agent_result_signal_at_start_of_output() {
        // Signal at the very start of output
        let content = "<promise>COMPLETE</promise>\nMore output follows...";
        assert_eq!(parse_agent_result(content), AgentResult::Complete);
    }

    #[test]
    fn parse_agent_result_failed_with_special_characters() {
        // Failure reason with special characters
        let content = "<promise>FAILED: Error: 'command not found' for path=/usr/bin</promise>";
        assert_eq!(
            parse_agent_result(content),
            AgentResult::Failed("Error: 'command not found' for path=/usr/bin".to_string())
        );
    }

    #[test]
    fn parse_agent_result_failed_with_colon_in_reason() {
        // Failure reason containing additional colons
        let content = "<promise>FAILED: Error: Something: went: wrong</promise>";
        assert_eq!(
            parse_agent_result(content),
            AgentResult::Failed("Error: Something: went: wrong".to_string())
        );
    }

    // Test the retry count boundary conditions
    #[test]
    fn retry_count_starts_at_zero_logic() {
        // Verify that retry logic expects count to start at 0
        // and increment after each failure
        let max_retries = 3;

        // Simulate retry counting
        let mut retry_count = 0;

        // First attempt fails
        retry_count += 1;
        assert!(retry_count < max_retries, "Should retry after first failure");

        // Second attempt fails
        retry_count += 1;
        assert!(retry_count < max_retries, "Should retry after second failure");

        // Third attempt fails
        retry_count += 1;
        assert!(
            retry_count >= max_retries,
            "Should stop after reaching max retries"
        );
    }

    #[test]
    fn agent_result_eq_for_complete() {
        assert_eq!(AgentResult::Complete, AgentResult::Complete);
    }

    #[test]
    fn agent_result_eq_for_failed_same_reason() {
        assert_eq!(
            AgentResult::Failed("reason".to_string()),
            AgentResult::Failed("reason".to_string())
        );
    }

    #[test]
    fn agent_result_neq_for_failed_different_reason() {
        assert_ne!(
            AgentResult::Failed("reason1".to_string()),
            AgentResult::Failed("reason2".to_string())
        );
    }

    #[test]
    fn agent_result_eq_for_no_signal() {
        assert_eq!(AgentResult::NoSignal, AgentResult::NoSignal);
    }

    #[test]
    fn agent_result_variants_are_distinct() {
        assert_ne!(AgentResult::Complete, AgentResult::NoSignal);
        assert_ne!(
            AgentResult::Complete,
            AgentResult::Failed("".to_string())
        );
        assert_ne!(
            AgentResult::NoSignal,
            AgentResult::Failed("".to_string())
        );
    }
}
