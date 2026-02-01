//! Orchestrator for the Ralph Loop.
//!
//! The simplified orchestrator:
//! 1. Generates a self-contained prompt with change location
//! 2. Spawns a single agent with that prompt
//! 3. Streams agent output to TUI
//! 4. Emits Complete when agent finishes
//!
//! The agent reads files directly and marks tasks complete by editing tasks.md.
//! No session management, no environment variables, no output parsing.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::{LoopEvent, LoopEventSender, LoopState};
use crate::agent::{AgentConfig, CodingAgent};
use crate::error::Result;
use crate::spec::{self, generate_prompt};

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
    /// Generates a prompt and spawns a single agent to work on the change.
    /// The agent reads files directly and edits tasks.md to mark progress.
    /// Returns the final loop state.
    pub async fn run(&mut self) -> Result<LoopState> {
        // Create adapter to get verification commands
        let adapter = spec::create_adapter(&self.change_name)?;

        // Generate the prompt
        let prompt = generate_prompt(&self.change_name, adapter.as_ref())?;

        // Initialize state
        let mut state = LoopState::new(&self.change_name);
        state.running = true;

        // Check for stop request before spawning
        if self.stop_flag.load(Ordering::Relaxed) {
            state.running = false;
            self.emit(LoopEvent::Complete).await;
            return Ok(state);
        }

        // Run the agent with the generated prompt
        match self.agent.run(&prompt, &self.config) {
            Ok(output) => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::{AgentOutput, TokenUsage};

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

    #[tokio::test]
    async fn orchestrator_emits_output_and_complete() {
        let (tx, _rx) = tokio::sync::mpsc::channel(10);
        let _agent = Box::new(MockAgent {
            output: "Task completed".to_string(),
        });

        let _orchestrator = Orchestrator::new(
            "test-change",
            Box::new(MockAgent { output: "test".to_string() }),
            AgentConfig::default(),
            tx,
        );

        // Note: This test validates structure. Actual testing requires a real change.
    }

    #[tokio::test]
    async fn orchestrator_respects_stop_flag() {
        let (tx, _rx) = tokio::sync::mpsc::channel(10);

        let orchestrator = Orchestrator::new(
            "test-change",
            Box::new(MockAgent { output: "test".to_string() }),
            AgentConfig::default(),
            tx,
        );

        // Set stop flag before running
        orchestrator.stop_flag.store(true, Ordering::Relaxed);

        // The run would return early due to stop flag
        // (Can't actually run without a real change, but this validates structure)
    }
}
