use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::JoinHandle;

use crate::ralph_loop::{LoopEvent, LoopState};
use crate::spec::openspec::{ChangeInfo, OpenSpecAdapter};
use crate::spec::SpecAdapter;
use crate::spec::{Scenario, Story};
use crate::ui::LoopResult;
use anyhow::Result;

/// The current screen being displayed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    /// Screen for selecting a completed OpenSpec change.
    ChangeSelection,
    /// Screen for previewing conversion results.
    ConversionPreview,
    /// Screen for displaying loop progress.
    LoopExecution,
    /// Screen for reviewing loop results.
    LoopResult,
}

/// Tab selection for the preview screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreviewTab {
    #[default]
    Tasks,
    Scenarios,
}

pub struct App {
    pub running: bool,
    /// Current screen being displayed.
    pub screen: Screen,
    /// Name of the selected change (if any).
    pub selected_change_name: Option<String>,
    /// Loaded stories from the selected change.
    pub stories: Vec<Story>,
    /// Loaded scenarios from the selected change.
    pub scenarios: Vec<Scenario>,
    /// List of available changes for selection.
    pub available_changes: Vec<ChangeInfo>,
    /// Currently selected index in the change selection list.
    pub selected_index: usize,
    /// Scroll offset for preview screen.
    pub scroll_offset: usize,
    /// Active tab in the preview screen.
    pub active_tab: PreviewTab,
    /// Scroll offset for the Tasks tab.
    pub tasks_scroll_offset: usize,
    /// Scroll offset for the Scenarios tab.
    pub scenarios_scroll_offset: usize,
    /// Loop execution state.
    pub loop_state: LoopState,
    /// Log messages during loop execution.
    pub loop_log: Vec<String>,
    /// Loop result for review.
    pub loop_result: LoopResult,
    /// Scroll offset for result screen.
    pub result_scroll_offset: usize,
    /// Receiver for loop events from the orchestrator.
    pub loop_event_rx: Option<Receiver<LoopEvent>>,
    /// Stop flag to signal the orchestrator to stop.
    pub loop_stop_flag: Option<Arc<AtomicBool>>,
    /// Handle to the orchestrator thread.
    pub loop_thread: Option<JoinHandle<()>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            screen: Screen::ChangeSelection,
            selected_change_name: None,
            stories: Vec::new(),
            scenarios: Vec::new(),
            available_changes: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            active_tab: PreviewTab::default(),
            tasks_scroll_offset: 0,
            scenarios_scroll_offset: 0,
            loop_state: LoopState::new(""),
            loop_log: Vec::new(),
            loop_result: LoopResult::default(),
            result_scroll_offset: 0,
            loop_event_rx: None,
            loop_stop_flag: None,
            loop_thread: None,
        }
    }

    /// Starts the loop execution for the selected change.
    pub fn start_loop(&mut self) {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::mpsc;

        use crate::agent::{AgentConfig, ClaudeAgent};
        use crate::ralph_loop::Orchestrator;

        if let Some(ref name) = self.selected_change_name {
            // Initialize loop state
            let mut state = LoopState::new(name);
            state.running = true;
            self.loop_state = state;
            self.loop_log.clear();

            // Create channel for events (std::sync::mpsc for TUI compatibility)
            let (tx, rx) = mpsc::channel();
            self.loop_event_rx = Some(rx);

            // Create stop flag
            let stop_flag = Arc::new(AtomicBool::new(false));
            self.loop_stop_flag = Some(Arc::clone(&stop_flag));

            // Spawn orchestrator in background thread with tokio runtime
            let change_name = name.clone();
            let handle = std::thread::spawn(move || {
                // Create tokio runtime for async orchestrator
                let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

                rt.block_on(async {
                    // Create tokio channel, then bridge to std channel
                    let (tokio_tx, mut tokio_rx) = tokio::sync::mpsc::channel::<LoopEvent>(100);

                    // Spawn a task to forward events from tokio channel to std channel
                    let std_tx = tx;
                    tokio::spawn(async move {
                        while let Some(event) = tokio_rx.recv().await {
                            if std_tx.send(event).is_err() {
                                break;
                            }
                        }
                    });

                    // Create and run orchestrator
                    let agent = Box::new(ClaudeAgent::new());
                    let config = AgentConfig::default();
                    let mut orchestrator =
                        Orchestrator::new(&change_name, agent, config, tokio_tx);

                    // Set the stop flag on the orchestrator
                    let orch_stop = orchestrator.stop_handle();
                    tokio::spawn(async move {
                        loop {
                            if stop_flag.load(Ordering::Relaxed) {
                                orch_stop.store(true, Ordering::Relaxed);
                                break;
                            }
                            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        }
                    });

                    let _ = orchestrator.run().await;
                });
            });

            self.loop_thread = Some(handle);
            self.screen = Screen::LoopExecution;
        }
    }

    /// Adds a log message to the loop log.
    pub fn add_loop_log(&mut self, message: String) {
        self.loop_log.push(message);
    }

    /// Transitions to the result screen with the given result.
    pub fn show_loop_result(&mut self, result: LoopResult) {
        self.loop_result = result;
        self.result_scroll_offset = 0;
        self.screen = Screen::LoopResult;
    }

    /// Builds a LoopResult from current state and git diff.
    pub fn build_loop_result(&self) -> LoopResult {
        // Get changed files from git diff
        let changed_files = Self::get_changed_files();

        LoopResult {
            change_name: self.selected_change_name.clone().unwrap_or_default(),
            stories_completed: 0,
            stories_total: 0,
            tasks_completed: 0,
            changed_files,
            verification_status: Vec::new(),
        }
    }

    /// Gets changed files from git diff.
    fn get_changed_files() -> Vec<String> {
        use std::process::Command;

        let output = Command::new("git")
            .args(["diff", "--name-status", "HEAD"])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(String::from)
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    /// Scrolls up in the result screen.
    pub fn result_scroll_up(&mut self) {
        self.result_scroll_offset = self.result_scroll_offset.saturating_sub(1);
    }

    /// Scrolls down in the result screen.
    pub fn result_scroll_down(&mut self) {
        self.result_scroll_offset = self.result_scroll_offset.saturating_add(1);
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Loads the list of available changes.
    pub fn load_changes(&mut self) -> Result<()> {
        self.available_changes = OpenSpecAdapter::list_changes()
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        // Filter to only show completed changes
        self.available_changes.retain(|c| {
            OpenSpecAdapter::is_complete(&c.name)
                .map_err(|e| anyhow::anyhow!("{}", e))
                .unwrap_or(false)
        });
        Ok(())
    }

    /// Loads data from the selected change.
    pub fn load_selected_change(&mut self) -> Result<()> {
        if let Some(ref name) = self.selected_change_name {
            let adapter = OpenSpecAdapter::new(name)
                .map_err(|e| anyhow::anyhow!("{}", e))?;
            self.stories = adapter.stories()
                .map_err(|e| anyhow::anyhow!("{}", e))?;
            self.scenarios = adapter.scenarios()
                .map_err(|e| anyhow::anyhow!("{}", e))?;
        }
        Ok(())
    }

    /// Selects a change by index and loads its data.
    pub fn select_change(&mut self, index: usize) -> Result<()> {
        if index < self.available_changes.len() {
            self.selected_change_name = Some(self.available_changes[index].name.clone());
            self.load_selected_change()?;
            self.screen = Screen::ConversionPreview;
            self.scroll_offset = 0;
        }
        Ok(())
    }

    /// Navigates back to the selection screen.
    pub fn back_to_selection(&mut self) {
        self.screen = Screen::ChangeSelection;
        // Preserve selected_index for when user returns
    }

    /// Returns scenarios that belong to a specific capability.
    pub fn scenarios_for_capability(&self, capability: &str) -> Vec<&Scenario> {
        self.scenarios
            .iter()
            .filter(|scenario| scenario.capability == capability)
            .collect()
    }

    /// Returns a sorted list of unique capability names from all scenarios.
    pub fn unique_capabilities(&self) -> Vec<String> {
        let mut capabilities: Vec<String> = self
            .scenarios
            .iter()
            .map(|s| s.capability.clone())
            .collect();
        capabilities.sort();
        capabilities.dedup();
        capabilities
    }

    /// Moves selection up in the change list.
    pub fn select_previous(&mut self) {
        if !self.available_changes.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.available_changes.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    /// Moves selection down in the change list.
    pub fn select_next(&mut self) {
        if !self.available_changes.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.available_changes.len();
        }
    }

    /// Returns a mutable reference to the current tab's scroll offset.
    fn current_scroll_offset(&mut self) -> &mut usize {
        match self.active_tab {
            PreviewTab::Tasks => &mut self.tasks_scroll_offset,
            PreviewTab::Scenarios => &mut self.scenarios_scroll_offset,
        }
    }

    /// Returns the current tab's scroll offset.
    pub fn get_scroll_offset(&self) -> usize {
        match self.active_tab {
            PreviewTab::Tasks => self.tasks_scroll_offset,
            PreviewTab::Scenarios => self.scenarios_scroll_offset,
        }
    }

    /// Scrolls up in the preview screen.
    pub fn scroll_up(&mut self) {
        let offset = self.current_scroll_offset();
        *offset = offset.saturating_sub(1);
    }

    /// Scrolls down in the preview screen.
    pub fn scroll_down(&mut self) {
        let offset = self.current_scroll_offset();
        *offset = offset.saturating_add(1);
    }

    /// Page up in the preview screen.
    pub fn page_up(&mut self) {
        let offset = self.current_scroll_offset();
        *offset = offset.saturating_sub(10);
    }

    /// Page down in the preview screen.
    pub fn page_down(&mut self) {
        let offset = self.current_scroll_offset();
        *offset = offset.saturating_add(10);
    }

    /// Switches to the next tab in the preview screen.
    pub fn switch_to_next_tab(&mut self) {
        self.active_tab = match self.active_tab {
            PreviewTab::Tasks => PreviewTab::Scenarios,
            PreviewTab::Scenarios => PreviewTab::Tasks,
        };
    }

    /// Switches to the previous tab in the preview screen.
    pub fn switch_to_previous_tab(&mut self) {
        self.active_tab = match self.active_tab {
            PreviewTab::Tasks => PreviewTab::Scenarios,
            PreviewTab::Scenarios => PreviewTab::Tasks,
        };
    }

    /// Processes loop events from the orchestrator.
    /// Returns true if the loop has completed.
    pub fn process_loop_events(&mut self) -> bool {
        use std::sync::mpsc::TryRecvError;

        // Collect events first to avoid borrow issues
        let mut events = Vec::new();
        let mut completed = false;
        let mut disconnected = false;

        if let Some(ref rx) = self.loop_event_rx {
            loop {
                match rx.try_recv() {
                    Ok(event) => events.push(event),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        disconnected = true;
                        break;
                    }
                }
            }
        } else {
            return false;
        }

        // Now process the collected events
        for event in events {
            match event {
                LoopEvent::StoryProgress {
                    story_id,
                    story_title,
                    current,
                    total,
                } => {
                    // Update loop state with current story info
                    self.loop_state.current_story_id = Some(story_id.clone());
                    self.loop_state.total_stories = total;
                    self.add_loop_log(format!(
                        "Starting story {}/{}: {} - {}",
                        current, total, story_id, story_title
                    ));
                }
                LoopEvent::AgentOutput { line } => {
                    // Truncate long lines for log display
                    let display_line = if line.len() > 100 {
                        format!("{}...", &line[..100])
                    } else {
                        line
                    };
                    self.add_loop_log(display_line);
                }
                LoopEvent::Error { message } => {
                    self.add_loop_log(format!("Error: {}", message));
                }
                LoopEvent::Complete => {
                    self.loop_state.running = false;
                    self.add_loop_log("Loop completed".to_string());
                    completed = true;
                }
            }
        }

        if disconnected {
            self.loop_state.running = false;
            return true;
        }

        completed
    }

    /// Requests the loop to stop gracefully.
    pub fn request_loop_stop(&mut self) {
        use std::sync::atomic::Ordering;

        if let Some(ref flag) = self.loop_stop_flag {
            flag.store(true, Ordering::Relaxed);
            self.add_loop_log("Stop requested, waiting for agent to finish...".to_string());
        }
    }

    /// Checks if the loop thread has finished.
    pub fn is_loop_thread_finished(&self) -> bool {
        match &self.loop_thread {
            Some(handle) => handle.is_finished(),
            None => true,
        }
    }

    /// Cleans up loop resources and transitions back to selection.
    pub fn cleanup_loop(&mut self) {
        // Take ownership of the thread handle and join it
        if let Some(handle) = self.loop_thread.take() {
            let _ = handle.join();
        }

        // Clear loop-related state
        self.loop_event_rx = None;
        self.loop_stop_flag = None;
        self.loop_state = LoopState::new("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;
    use std::sync::mpsc;

    #[test]
    fn process_loop_events_handles_story_progress() {
        let mut app = App::new();
        let (tx, rx) = mpsc::channel();
        app.loop_event_rx = Some(rx);

        tx.send(LoopEvent::StoryProgress {
            story_id: "1".to_string(),
            story_title: "First Story".to_string(),
            current: 1,
            total: 3,
        })
        .unwrap();

        let completed = app.process_loop_events();

        assert!(!completed);
        assert_eq!(app.loop_state.current_story_id, Some("1".to_string()));
        assert_eq!(app.loop_state.total_stories, 3);
        assert!(app
            .loop_log
            .iter()
            .any(|l| l.contains("Starting story 1/3")));
    }

    #[test]
    fn process_loop_events_handles_agent_output() {
        let mut app = App::new();
        let (tx, rx) = mpsc::channel();
        app.loop_event_rx = Some(rx);

        tx.send(LoopEvent::AgentOutput {
            line: "Agent is working...".to_string(),
        })
        .unwrap();

        let completed = app.process_loop_events();

        assert!(!completed);
        assert!(app.loop_log.iter().any(|l| l.contains("Agent is working")));
    }

    #[test]
    fn process_loop_events_handles_complete() {
        let mut app = App::new();
        let (tx, rx) = mpsc::channel();
        app.loop_event_rx = Some(rx);
        app.loop_state.running = true;

        tx.send(LoopEvent::Complete).unwrap();

        let completed = app.process_loop_events();

        assert!(completed);
        assert!(!app.loop_state.running);
        assert!(app.loop_log.iter().any(|l| l.contains("Loop completed")));
    }

    #[test]
    fn process_loop_events_handles_error() {
        let mut app = App::new();
        let (tx, rx) = mpsc::channel();
        app.loop_event_rx = Some(rx);

        tx.send(LoopEvent::Error {
            message: "Test error".to_string(),
        })
        .unwrap();

        app.process_loop_events();

        assert!(app.loop_log.iter().any(|l| l.contains("Error: Test error")));
    }

    #[test]
    fn process_loop_events_returns_false_when_no_receiver() {
        let mut app = App::new();
        app.loop_event_rx = None;

        let completed = app.process_loop_events();

        assert!(!completed);
    }

    #[test]
    fn request_loop_stop_sets_flag() {
        let mut app = App::new();
        let stop_flag = Arc::new(AtomicBool::new(false));
        app.loop_stop_flag = Some(Arc::clone(&stop_flag));

        app.request_loop_stop();

        assert!(stop_flag.load(Ordering::Relaxed));
        assert!(app.loop_log.iter().any(|l| l.contains("Stop requested")));
    }

    #[test]
    fn request_loop_stop_handles_no_flag() {
        let mut app = App::new();
        app.loop_stop_flag = None;

        app.request_loop_stop(); // Should not panic

        assert!(app.loop_log.is_empty());
    }

    #[test]
    fn cleanup_loop_clears_state() {
        let mut app = App::new();
        let (_, rx) = mpsc::channel::<LoopEvent>();
        app.loop_event_rx = Some(rx);
        app.loop_stop_flag = Some(Arc::new(AtomicBool::new(false)));
        app.loop_state = LoopState::new("test-change");
        app.loop_state.running = true;

        app.cleanup_loop();

        assert!(app.loop_event_rx.is_none());
        assert!(app.loop_stop_flag.is_none());
        assert!(!app.loop_state.running);
    }

    #[test]
    fn is_loop_thread_finished_returns_true_when_none() {
        let app = App::new();
        assert!(app.is_loop_thread_finished());
    }

    #[test]
    fn build_loop_result_captures_state() {
        let mut app = App::new();
        app.selected_change_name = Some("my-change".to_string());

        let result = app.build_loop_result();

        assert_eq!(result.change_name, "my-change");
    }
}
