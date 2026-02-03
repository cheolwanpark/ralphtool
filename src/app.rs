use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Instant;

use crate::agent::StreamEvent;
use crate::ralph_loop::{LoopEvent, LoopState, DEFAULT_MAX_RETRIES, DEFAULT_COMMAND_TIMEOUT_SECS};
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

/// Tab selection for the loop execution screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)] // Used in Story 4 and Story 5
pub enum LoopTab {
    /// Shows story title and task list with checkboxes.
    #[default]
    Info,
    /// Shows agent messages and responses.
    Agent,
}

/// Tab selection for the result screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)] // Used in Story 3 and Story 4
pub enum ResultTab {
    /// Shows stories with their tasks and completion status.
    #[default]
    Tasks,
    /// Shows list of changed files from git diff.
    ChangedFiles,
}

/// Action to take after a quit key press during loop execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForceQuitAction {
    /// First press: request graceful stop.
    Graceful,
    /// Second press: show hint about force-quit.
    Hint,
    /// Third press: force-quit immediately.
    ForceQuit,
    /// Loop already stopped: navigate back to selection.
    NavigateBack,
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
    /// Stream events per story, keyed by story_id.
    pub story_events: HashMap<String, Vec<StreamEvent>>,
    /// Currently selected story index for navigation.
    pub loop_selected_story: usize,
    /// Active tab in the loop execution screen.
    pub loop_tab: LoopTab,
    /// Scroll offset for the Info tab.
    pub loop_info_scroll: usize,
    /// Scroll offset for the Agent tab.
    pub loop_agent_scroll: usize,
    /// Auto-scroll flag for the Agent tab (snap-to-bottom behavior).
    pub loop_agent_auto_scroll: bool,
    /// Last known max scroll position for Agent tab (updated during render).
    pub loop_agent_max_scroll: usize,
    /// Loop result for review.
    pub loop_result: LoopResult,
    /// Scroll offset for result screen (used for Changed Files tab).
    pub result_scroll_offset: usize,
    /// Active tab in the result screen.
    pub result_tab: ResultTab,
    /// Scroll offset for the Tasks tab in result screen.
    pub result_tasks_scroll: usize,
    /// Receiver for loop events from the orchestrator.
    pub loop_event_rx: Option<Receiver<LoopEvent>>,
    /// Stop flag to signal the orchestrator to stop.
    pub loop_stop_flag: Option<Arc<AtomicBool>>,
    /// Handle to the orchestrator thread.
    pub loop_thread: Option<JoinHandle<()>>,
    /// Maximum number of retries per story (CLI: --max-retries).
    pub max_retries: usize,
    /// Timeout in seconds for external commands (CLI: --command-timeout).
    pub command_timeout: u64,
    /// Count of consecutive 'q' presses for force-quit mechanism.
    pub quit_press_count: usize,
    /// Time of last 'q' press for tracking consecutive presses.
    pub last_quit_time: Option<Instant>,
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
            story_events: HashMap::new(),
            loop_selected_story: 0,
            loop_tab: LoopTab::default(),
            loop_info_scroll: 0,
            loop_agent_scroll: 0,
            loop_agent_auto_scroll: true,
            loop_agent_max_scroll: 0,
            loop_result: LoopResult::default(),
            result_scroll_offset: 0,
            result_tab: ResultTab::default(),
            result_tasks_scroll: 0,
            loop_event_rx: None,
            loop_stop_flag: None,
            loop_thread: None,
            max_retries: DEFAULT_MAX_RETRIES,
            command_timeout: DEFAULT_COMMAND_TIMEOUT_SECS,
            quit_press_count: 0,
            last_quit_time: None,
        }
    }

    /// Sets the maximum number of retries per story.
    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets the timeout in seconds for external commands.
    pub fn with_command_timeout(mut self, timeout: u64) -> Self {
        self.command_timeout = timeout;
        self
    }

    /// Starts the loop execution for the selected change.
    pub fn start_loop(&mut self) {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::mpsc;

        use crate::agent::ClaudeAgent;
        use crate::ralph_loop::Orchestrator;

        if let Some(ref name) = self.selected_change_name {
            // Initialize loop state
            let mut state = LoopState::new(name);
            state.running = true;
            self.loop_state = state;
            self.story_events.clear();
            self.loop_selected_story = 0;
            self.loop_tab = LoopTab::default();
            self.loop_info_scroll = 0;
            self.loop_agent_scroll = 0;
            self.loop_agent_auto_scroll = true;
            self.loop_agent_max_scroll = 0;

            // Create channel for events (std::sync::mpsc for TUI compatibility)
            let (tx, rx) = mpsc::channel();
            self.loop_event_rx = Some(rx);

            // Create stop flag
            let stop_flag = Arc::new(AtomicBool::new(false));
            self.loop_stop_flag = Some(Arc::clone(&stop_flag));

            // Spawn orchestrator in background thread with tokio runtime
            let change_name = name.clone();
            let max_retries = self.max_retries;
            let command_timeout = self.command_timeout;
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
                    let mut orchestrator =
                        Orchestrator::new(&change_name, agent, tokio_tx, max_retries)
                            .with_command_timeout(command_timeout);

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

    /// Transitions to the result screen with the given result.
    pub fn show_loop_result(&mut self, result: LoopResult) {
        self.loop_result = result;
        self.result_scroll_offset = 0;
        self.result_tab = ResultTab::default();
        self.result_tasks_scroll = 0;
        self.screen = Screen::LoopResult;
    }

    /// Builds a LoopResult from current state and git diff.
    pub fn build_loop_result(&self) -> LoopResult {
        // Get changed files from git diff
        let changed_files = Self::get_changed_files();

        // Re-parse tasks.md to get updated completion status
        let stories = if let Some(ref name) = self.selected_change_name {
            OpenSpecAdapter::new(name)
                .ok()
                .map(|adapter| adapter.stories().unwrap_or_default())
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        // Calculate completion statistics
        let stories_total = stories.len();
        let stories_completed = stories.iter().filter(|s| s.is_complete()).count();
        let tasks_total: usize = stories.iter().map(|s| s.tasks.len()).sum();
        let tasks_completed: usize = stories
            .iter()
            .flat_map(|s| &s.tasks)
            .filter(|t| t.done)
            .count();

        LoopResult {
            change_name: self.selected_change_name.clone().unwrap_or_default(),
            stories_completed,
            stories_total,
            tasks_completed,
            tasks_total,
            changed_files,
            stories,
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

    /// Switches between Tasks and ChangedFiles tabs in the result screen.
    pub fn switch_result_tab(&mut self) {
        self.result_tab = match self.result_tab {
            ResultTab::Tasks => ResultTab::ChangedFiles,
            ResultTab::ChangedFiles => ResultTab::Tasks,
        };
    }

    /// Scrolls up in the Tasks tab of the result screen.
    #[allow(dead_code)] // Provided for direct Tasks tab scrolling; result_scroll_up is used in event handling
    pub fn result_tasks_scroll_up(&mut self) {
        self.result_tasks_scroll = self.result_tasks_scroll.saturating_sub(1);
    }

    /// Scrolls down in the Tasks tab of the result screen.
    #[allow(dead_code)] // Provided for direct Tasks tab scrolling; result_scroll_down is used in event handling
    pub fn result_tasks_scroll_down(&mut self) {
        self.result_tasks_scroll = self.result_tasks_scroll.saturating_add(1);
    }

    /// Scrolls up in the result screen (uses appropriate offset based on active tab).
    pub fn result_scroll_up(&mut self) {
        match self.result_tab {
            ResultTab::Tasks => self.result_tasks_scroll = self.result_tasks_scroll.saturating_sub(1),
            ResultTab::ChangedFiles => self.result_scroll_offset = self.result_scroll_offset.saturating_sub(1),
        }
    }

    /// Scrolls down in the result screen (uses appropriate offset based on active tab).
    pub fn result_scroll_down(&mut self) {
        match self.result_tab {
            ResultTab::Tasks => self.result_tasks_scroll = self.result_tasks_scroll.saturating_add(1),
            ResultTab::ChangedFiles => self.result_scroll_offset = self.result_scroll_offset.saturating_add(1),
        }
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
                    story_title: _,
                    current: _,
                    total,
                    completed,
                } => {
                    // Update loop state with current story info
                    self.loop_state.current_story_id = Some(story_id.clone());
                    self.loop_state.total_stories = total;
                    self.loop_state.completed_stories = completed;

                    // Track started stories and auto-select new story
                    if !self.loop_state.started_story_ids.contains(&story_id) {
                        self.loop_state.started_story_ids.push(story_id.clone());
                        // Auto-select newly started story
                        self.loop_selected_story = self.loop_state.started_story_ids.len() - 1;
                        // Reset scroll positions for the new story
                        self.loop_info_scroll = 0;
                        self.loop_agent_scroll = 0;
                        self.loop_agent_auto_scroll = true;
                    }

                    // Initialize story_events entry if not present
                    self.story_events.entry(story_id).or_default();
                }
                LoopEvent::StoryEvent { story_id, event } => {
                    // Track started stories if not already tracked
                    if !self.loop_state.started_story_ids.contains(&story_id) {
                        self.loop_state.started_story_ids.push(story_id.clone());
                        // Auto-select newly started story
                        self.loop_selected_story = self.loop_state.started_story_ids.len() - 1;
                        // Reset scroll positions for the new story
                        self.loop_info_scroll = 0;
                        self.loop_agent_scroll = 0;
                        self.loop_agent_auto_scroll = true;
                    }

                    // Store the full StreamEvent in story_events HashMap
                    self.story_events
                        .entry(story_id)
                        .or_default()
                        .push(event);
                }
                LoopEvent::Error { message: _ } => {
                    // Errors are logged but not stored in story_events
                }
                LoopEvent::Complete => {
                    self.loop_state.running = false;
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
        }
    }

    /// Duration in seconds for tracking consecutive 'q' presses.
    const FORCE_QUIT_WINDOW_SECS: u64 = 3;

    /// Number of consecutive 'q' presses required for force-quit.
    #[allow(dead_code)] // Used in force_quit_hint() which is tested but not yet integrated in UI
    const FORCE_QUIT_PRESS_COUNT: usize = 3;

    /// Handles a quit key press during loop execution.
    ///
    /// Returns `ForceQuitAction` indicating what action should be taken:
    /// - `Graceful`: Request graceful stop (first press)
    /// - `Hint`: Stop already requested, show hint about force-quit (second press)
    /// - `ForceQuit`: Third press within 3 seconds, force-quit immediately
    /// - `NavigateBack`: Loop already stopped, navigate back to selection
    pub fn handle_quit_press(&mut self) -> ForceQuitAction {
        use std::time::Duration;

        if !self.loop_state.running {
            // Loop already stopped, allow normal navigation
            return ForceQuitAction::NavigateBack;
        }

        let now = Instant::now();
        let window = Duration::from_secs(Self::FORCE_QUIT_WINDOW_SECS);

        // Check if this press is within the time window of the last press
        let within_window = self.last_quit_time
            .map(|t| now.duration_since(t) < window)
            .unwrap_or(false);

        if within_window {
            self.quit_press_count += 1;
        } else {
            // Reset count if outside window
            self.quit_press_count = 1;
        }
        self.last_quit_time = Some(now);

        match self.quit_press_count {
            1 => {
                // First press: request graceful stop
                self.request_loop_stop();
                ForceQuitAction::Graceful
            }
            2 => {
                // Second press: show hint about force-quit
                ForceQuitAction::Hint
            }
            _ => {
                // Third or more presses: force-quit
                // Attempt cleanup before exit
                self.cleanup_loop();
                ForceQuitAction::ForceQuit
            }
        }
    }

    /// Resets the quit press counter (called when navigating away from loop screen).
    pub fn reset_quit_counter(&mut self) {
        self.quit_press_count = 0;
        self.last_quit_time = None;
    }

    /// Returns a message indicating force-quit status.
    ///
    /// This is displayed in the UI when graceful stop is requested but not yet complete.
    #[allow(dead_code)] // Public API for TUI integration, tested but not yet wired to UI rendering
    pub fn force_quit_hint(&self) -> Option<String> {
        if self.quit_press_count >= 1 && self.loop_state.running {
            let remaining = Self::FORCE_QUIT_PRESS_COUNT.saturating_sub(self.quit_press_count);
            if remaining > 0 {
                Some(format!("Press 'q' {} more time{} to force-quit", remaining, if remaining == 1 { "" } else { "s" }))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns the IDs of visible stories for the indicator display (max 5).
    /// Uses a sliding window centered on the selected story when there are more than 5 stories.
    #[allow(dead_code)] // Used in Story 5 UI rewrite
    pub fn visible_stories(&self) -> Vec<&str> {
        let started = &self.loop_state.started_story_ids;
        let count = started.len();

        if count <= 5 {
            // Show all stories if 5 or fewer
            started.iter().map(|s| s.as_str()).collect()
        } else {
            // Sliding window of 5, centered on selected story
            let selected = self.loop_selected_story;
            let half_window = 2;

            let start = if selected <= half_window {
                0
            } else if selected >= count - half_window - 1 {
                count - 5
            } else {
                selected - half_window
            };

            started[start..start + 5]
                .iter()
                .map(|s| s.as_str())
                .collect()
        }
    }

    /// Returns the currently selected story ID, if any.
    #[allow(dead_code)] // Used in Story 5 UI rewrite
    pub fn current_story(&self) -> Option<&str> {
        self.loop_state
            .started_story_ids
            .get(self.loop_selected_story)
            .map(|s| s.as_str())
    }

    /// Returns true if the user can navigate to the previous story.
    pub fn can_navigate_left(&self) -> bool {
        self.loop_selected_story > 0
    }

    /// Returns true if the user can navigate to the next story.
    pub fn can_navigate_right(&self) -> bool {
        let started_count = self.loop_state.started_story_ids.len();
        started_count > 0 && self.loop_selected_story < started_count - 1
    }

    /// Navigates to the previous story (left arrow).
    pub fn navigate_to_previous_story(&mut self) {
        if self.can_navigate_left() {
            self.loop_selected_story -= 1;
            // Reset scroll positions for the new story
            self.loop_info_scroll = 0;
            self.loop_agent_scroll = 0;
            self.loop_agent_auto_scroll = true;
        }
    }

    /// Navigates to the next story (right arrow).
    pub fn navigate_to_next_story(&mut self) {
        if self.can_navigate_right() {
            self.loop_selected_story += 1;
            // Reset scroll positions for the new story
            self.loop_info_scroll = 0;
            self.loop_agent_scroll = 0;
            self.loop_agent_auto_scroll = true;
        }
    }

    /// Switches between Info and Agent tabs in the loop screen.
    pub fn switch_loop_tab(&mut self) {
        self.loop_tab = match self.loop_tab {
            LoopTab::Info => LoopTab::Agent,
            LoopTab::Agent => LoopTab::Info,
        };
    }

    /// Scrolls up in the loop execution screen (current tab).
    /// For Agent tab, disables auto-scroll since user is reading history.
    pub fn loop_scroll_up(&mut self) {
        match self.loop_tab {
            LoopTab::Info => self.loop_info_scroll = self.loop_info_scroll.saturating_sub(1),
            LoopTab::Agent => {
                self.loop_agent_scroll = self.loop_agent_scroll.saturating_sub(1);
                self.loop_agent_auto_scroll = false;
            }
        }
    }

    /// Scrolls down in the loop execution screen (current tab).
    /// For Agent tab, re-enables auto-scroll when reaching bottom.
    pub fn loop_scroll_down(&mut self) {
        match self.loop_tab {
            LoopTab::Info => self.loop_info_scroll = self.loop_info_scroll.saturating_add(1),
            LoopTab::Agent => {
                self.loop_agent_scroll = self.loop_agent_scroll.saturating_add(1);
                // Re-enable auto-scroll when reaching bottom
                if self.loop_agent_scroll >= self.loop_agent_max_scroll {
                    self.loop_agent_auto_scroll = true;
                }
            }
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

        // Clear story navigation and tab state
        self.story_events.clear();
        self.loop_selected_story = 0;
        self.loop_tab = LoopTab::default();
        self.loop_info_scroll = 0;
        self.loop_agent_scroll = 0;
        self.loop_agent_auto_scroll = true;
        self.loop_agent_max_scroll = 0;
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
            completed: 0,
        })
        .unwrap();

        let completed = app.process_loop_events();

        assert!(!completed);
        assert_eq!(app.loop_state.current_story_id, Some("1".to_string()));
        assert_eq!(app.loop_state.total_stories, 3);
        // Story should be tracked and auto-selected
        assert!(app.loop_state.started_story_ids.contains(&"1".to_string()));
        assert_eq!(app.loop_selected_story, 0);
        // Story events entry should be initialized
        assert!(app.story_events.contains_key("1"));
    }

    #[test]
    fn process_loop_events_handles_story_event() {
        let mut app = App::new();
        let (tx, rx) = mpsc::channel();
        app.loop_event_rx = Some(rx);

        tx.send(LoopEvent::StoryEvent {
            story_id: "1".to_string(),
            event: StreamEvent::Message("Agent is working...".to_string()),
        })
        .unwrap();

        let completed = app.process_loop_events();

        assert!(!completed);
        assert!(app.loop_state.started_story_ids.contains(&"1".to_string()));
        // Event should be stored in story_events
        let events = app.story_events.get("1").expect("Story events should exist");
        assert_eq!(events.len(), 1);
        match &events[0] {
            StreamEvent::Message(msg) => assert_eq!(msg, "Agent is working..."),
            _ => panic!("Expected Message event"),
        }
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

        let completed = app.process_loop_events();

        // Errors don't trigger completion
        assert!(!completed);
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
    }

    #[test]
    fn request_loop_stop_handles_no_flag() {
        let mut app = App::new();
        app.loop_stop_flag = None;

        app.request_loop_stop(); // Should not panic
    }

    #[test]
    fn cleanup_loop_clears_state() {
        let mut app = App::new();
        let (_, rx) = mpsc::channel::<LoopEvent>();
        app.loop_event_rx = Some(rx);
        app.loop_stop_flag = Some(Arc::new(AtomicBool::new(false)));
        app.loop_state = LoopState::new("test-change");
        app.loop_state.running = true;

        // Set up story navigation state
        app.story_events.insert("1".to_string(), vec![StreamEvent::Message("test".to_string())]);
        app.loop_selected_story = 2;
        app.loop_tab = LoopTab::Agent;
        app.loop_info_scroll = 5;
        app.loop_agent_scroll = 10;
        app.loop_agent_auto_scroll = false;
        app.loop_agent_max_scroll = 100;

        app.cleanup_loop();

        // Verify original state is cleared
        assert!(app.loop_event_rx.is_none());
        assert!(app.loop_stop_flag.is_none());
        assert!(!app.loop_state.running);

        // Verify new state fields are cleared
        assert!(app.story_events.is_empty());
        assert_eq!(app.loop_selected_story, 0);
        assert_eq!(app.loop_tab, LoopTab::Info);
        assert_eq!(app.loop_info_scroll, 0);
        assert_eq!(app.loop_agent_scroll, 0);
        assert!(app.loop_agent_auto_scroll);
        assert_eq!(app.loop_agent_max_scroll, 0);
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

    #[test]
    fn visible_stories_returns_all_when_five_or_fewer() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ];

        let visible = app.visible_stories();

        assert_eq!(visible, vec!["1", "2", "3"]);
    }

    #[test]
    fn visible_stories_returns_sliding_window_when_more_than_five() {
        let mut app = App::new();
        app.loop_state.started_story_ids = (1..=8).map(|i| i.to_string()).collect();
        app.loop_selected_story = 4; // Select story "5" (0-indexed)

        let visible = app.visible_stories();

        // Should show window centered on selection: 3, 4, 5, 6, 7
        assert_eq!(visible.len(), 5);
        assert_eq!(visible, vec!["3", "4", "5", "6", "7"]);
    }

    #[test]
    fn visible_stories_window_at_start() {
        let mut app = App::new();
        app.loop_state.started_story_ids = (1..=8).map(|i| i.to_string()).collect();
        app.loop_selected_story = 0; // Select first story

        let visible = app.visible_stories();

        // Should show first 5
        assert_eq!(visible, vec!["1", "2", "3", "4", "5"]);
    }

    #[test]
    fn visible_stories_window_at_end() {
        let mut app = App::new();
        app.loop_state.started_story_ids = (1..=8).map(|i| i.to_string()).collect();
        app.loop_selected_story = 7; // Select last story

        let visible = app.visible_stories();

        // Should show last 5
        assert_eq!(visible, vec!["4", "5", "6", "7", "8"]);
    }

    #[test]
    fn current_story_returns_selected() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];
        app.loop_selected_story = 1;

        assert_eq!(app.current_story(), Some("b"));
    }

    #[test]
    fn current_story_returns_none_when_empty() {
        let app = App::new();

        assert_eq!(app.current_story(), None);
    }

    #[test]
    fn can_navigate_left_when_not_first() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec!["1".to_string(), "2".to_string()];
        app.loop_selected_story = 1;

        assert!(app.can_navigate_left());
    }

    #[test]
    fn cannot_navigate_left_when_first() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec!["1".to_string(), "2".to_string()];
        app.loop_selected_story = 0;

        assert!(!app.can_navigate_left());
    }

    #[test]
    fn can_navigate_right_when_not_last() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec!["1".to_string(), "2".to_string()];
        app.loop_selected_story = 0;

        assert!(app.can_navigate_right());
    }

    #[test]
    fn cannot_navigate_right_when_last() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec!["1".to_string(), "2".to_string()];
        app.loop_selected_story = 1;

        assert!(!app.can_navigate_right());
    }

    #[test]
    fn cannot_navigate_when_empty() {
        let app = App::new();

        assert!(!app.can_navigate_left());
        assert!(!app.can_navigate_right());
    }

    #[test]
    fn auto_selects_new_story_on_progress() {
        let mut app = App::new();
        let (tx, rx) = mpsc::channel();
        app.loop_event_rx = Some(rx);

        // Send first story progress
        tx.send(LoopEvent::StoryProgress {
            story_id: "1".to_string(),
            story_title: "First".to_string(),
            current: 1,
            total: 3,
            completed: 0,
        })
        .unwrap();
        app.process_loop_events();
        assert_eq!(app.loop_selected_story, 0);

        // Send second story progress
        tx.send(LoopEvent::StoryProgress {
            story_id: "2".to_string(),
            story_title: "Second".to_string(),
            current: 2,
            total: 3,
            completed: 1,
        })
        .unwrap();
        app.process_loop_events();
        assert_eq!(app.loop_selected_story, 1);
    }

    #[test]
    fn navigate_to_previous_story_decrements_selected() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        app.loop_selected_story = 2;

        app.navigate_to_previous_story();

        assert_eq!(app.loop_selected_story, 1);
    }

    #[test]
    fn navigate_to_previous_story_stops_at_first() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec!["1".to_string(), "2".to_string()];
        app.loop_selected_story = 0;

        app.navigate_to_previous_story();

        assert_eq!(app.loop_selected_story, 0);
    }

    #[test]
    fn navigate_to_next_story_increments_selected() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        app.loop_selected_story = 0;

        app.navigate_to_next_story();

        assert_eq!(app.loop_selected_story, 1);
    }

    #[test]
    fn navigate_to_next_story_stops_at_last_started() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec!["1".to_string(), "2".to_string()];
        app.loop_selected_story = 1;

        app.navigate_to_next_story();

        // Should not advance beyond last started story
        assert_eq!(app.loop_selected_story, 1);
    }

    #[test]
    fn navigate_does_not_select_unstarted_stories() {
        let mut app = App::new();
        // Only 2 stories have started, but there might be more in total
        app.loop_state.started_story_ids = vec!["1".to_string(), "2".to_string()];
        app.loop_state.total_stories = 5;
        app.loop_selected_story = 1; // At last started story

        // Try to navigate right - should be blocked
        app.navigate_to_next_story();
        assert_eq!(app.loop_selected_story, 1);

        // Navigate left should work
        app.navigate_to_previous_story();
        assert_eq!(app.loop_selected_story, 0);

        // At first story, left should be blocked
        app.navigate_to_previous_story();
        assert_eq!(app.loop_selected_story, 0);
    }

    #[test]
    fn navigate_to_previous_story_resets_scroll_and_auto_scroll() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        app.loop_selected_story = 2;
        // Set non-default scroll positions
        app.loop_info_scroll = 10;
        app.loop_agent_scroll = 15;
        app.loop_agent_auto_scroll = false;

        app.navigate_to_previous_story();

        assert_eq!(app.loop_selected_story, 1);
        assert_eq!(app.loop_info_scroll, 0);
        assert_eq!(app.loop_agent_scroll, 0);
        assert!(app.loop_agent_auto_scroll);
    }

    #[test]
    fn navigate_to_next_story_resets_scroll_and_auto_scroll() {
        let mut app = App::new();
        app.loop_state.started_story_ids = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        app.loop_selected_story = 0;
        // Set non-default scroll positions
        app.loop_info_scroll = 10;
        app.loop_agent_scroll = 15;
        app.loop_agent_auto_scroll = false;

        app.navigate_to_next_story();

        assert_eq!(app.loop_selected_story, 1);
        assert_eq!(app.loop_info_scroll, 0);
        assert_eq!(app.loop_agent_scroll, 0);
        assert!(app.loop_agent_auto_scroll);
    }

    #[test]
    fn auto_select_new_story_resets_scroll_and_auto_scroll() {
        let mut app = App::new();
        let (tx, rx) = mpsc::channel();
        app.loop_event_rx = Some(rx);

        // First story, set scroll positions
        tx.send(LoopEvent::StoryProgress {
            story_id: "1".to_string(),
            story_title: "First".to_string(),
            current: 1,
            total: 3,
            completed: 0,
        })
        .unwrap();
        app.process_loop_events();

        // Simulate user scrolling
        app.loop_info_scroll = 10;
        app.loop_agent_scroll = 15;
        app.loop_agent_auto_scroll = false;

        // Second story arrives (new story auto-selection)
        tx.send(LoopEvent::StoryProgress {
            story_id: "2".to_string(),
            story_title: "Second".to_string(),
            current: 2,
            total: 3,
            completed: 1,
        })
        .unwrap();
        app.process_loop_events();

        // Should auto-select and reset scroll positions
        assert_eq!(app.loop_selected_story, 1);
        assert_eq!(app.loop_info_scroll, 0);
        assert_eq!(app.loop_agent_scroll, 0);
        assert!(app.loop_agent_auto_scroll);
    }

    #[test]
    fn switch_loop_tab_toggles_between_info_and_agent() {
        let mut app = App::new();
        assert_eq!(app.loop_tab, LoopTab::Info);

        app.switch_loop_tab();
        assert_eq!(app.loop_tab, LoopTab::Agent);

        app.switch_loop_tab();
        assert_eq!(app.loop_tab, LoopTab::Info);
    }

    #[test]
    fn loop_scroll_respects_current_tab() {
        let mut app = App::new();

        // Default is Info tab
        app.loop_scroll_down();
        app.loop_scroll_down();
        assert_eq!(app.loop_info_scroll, 2);
        assert_eq!(app.loop_agent_scroll, 0);

        // Switch to Agent tab
        app.switch_loop_tab();
        app.loop_scroll_down();
        assert_eq!(app.loop_info_scroll, 2);
        assert_eq!(app.loop_agent_scroll, 1);

        // Scroll up
        app.loop_scroll_up();
        assert_eq!(app.loop_agent_scroll, 0);
    }

    #[test]
    fn loop_scroll_up_disables_auto_scroll_on_agent_tab() {
        let mut app = App::new();
        app.loop_tab = LoopTab::Agent;
        app.loop_agent_auto_scroll = true;
        app.loop_agent_scroll = 5;

        app.loop_scroll_up();

        assert_eq!(app.loop_agent_scroll, 4);
        assert!(!app.loop_agent_auto_scroll);
    }

    #[test]
    fn loop_scroll_up_does_not_affect_auto_scroll_on_info_tab() {
        let mut app = App::new();
        app.loop_tab = LoopTab::Info;
        app.loop_agent_auto_scroll = true;
        app.loop_info_scroll = 5;

        app.loop_scroll_up();

        assert_eq!(app.loop_info_scroll, 4);
        // Info tab doesn't affect auto_scroll
        assert!(app.loop_agent_auto_scroll);
    }

    #[test]
    fn loop_scroll_down_enables_auto_scroll_when_at_bottom() {
        let mut app = App::new();
        app.loop_tab = LoopTab::Agent;
        app.loop_agent_auto_scroll = false;
        app.loop_agent_scroll = 9;
        app.loop_agent_max_scroll = 10;

        app.loop_scroll_down();

        assert_eq!(app.loop_agent_scroll, 10);
        assert!(app.loop_agent_auto_scroll);
    }

    #[test]
    fn loop_scroll_down_does_not_enable_auto_scroll_before_bottom() {
        let mut app = App::new();
        app.loop_tab = LoopTab::Agent;
        app.loop_agent_auto_scroll = false;
        app.loop_agent_scroll = 5;
        app.loop_agent_max_scroll = 10;

        app.loop_scroll_down();

        assert_eq!(app.loop_agent_scroll, 6);
        // Not at bottom yet, auto_scroll stays false
        assert!(!app.loop_agent_auto_scroll);
    }

    #[test]
    fn loop_scroll_down_enables_auto_scroll_when_beyond_bottom() {
        let mut app = App::new();
        app.loop_tab = LoopTab::Agent;
        app.loop_agent_auto_scroll = false;
        app.loop_agent_scroll = 10;
        app.loop_agent_max_scroll = 10;

        // Even if already at/beyond max, scrolling down should enable auto_scroll
        app.loop_scroll_down();

        assert_eq!(app.loop_agent_scroll, 11); // Would be clamped during render
        assert!(app.loop_agent_auto_scroll);
    }

    #[test]
    fn auto_selects_new_story_on_story_event() {
        let mut app = App::new();
        let (tx, rx) = mpsc::channel();
        app.loop_event_rx = Some(rx);

        // Send first story event (without prior StoryProgress)
        tx.send(LoopEvent::StoryEvent {
            story_id: "1".to_string(),
            event: StreamEvent::Message("Working on story 1...".to_string()),
        })
        .unwrap();
        app.process_loop_events();
        assert_eq!(app.loop_selected_story, 0);
        assert_eq!(app.loop_state.started_story_ids.len(), 1);

        // Send second story event for a new story
        tx.send(LoopEvent::StoryEvent {
            story_id: "2".to_string(),
            event: StreamEvent::Message("Working on story 2...".to_string()),
        })
        .unwrap();
        app.process_loop_events();

        // Should auto-select the new story
        assert_eq!(app.loop_selected_story, 1);
        assert_eq!(app.loop_state.started_story_ids.len(), 2);
    }

    #[test]
    fn does_not_change_selection_for_existing_story() {
        let mut app = App::new();
        let (tx, rx) = mpsc::channel();
        app.loop_event_rx = Some(rx);

        // Set up two stories
        tx.send(LoopEvent::StoryProgress {
            story_id: "1".to_string(),
            story_title: "First".to_string(),
            current: 1,
            total: 3,
            completed: 0,
        })
        .unwrap();
        tx.send(LoopEvent::StoryProgress {
            story_id: "2".to_string(),
            story_title: "Second".to_string(),
            current: 2,
            total: 3,
            completed: 1,
        })
        .unwrap();
        app.process_loop_events();
        assert_eq!(app.loop_selected_story, 1); // Auto-selected second story

        // User navigates back to first story
        app.navigate_to_previous_story();
        assert_eq!(app.loop_selected_story, 0);

        // Another event comes in for story 2 (existing story)
        tx.send(LoopEvent::StoryEvent {
            story_id: "2".to_string(),
            event: StreamEvent::Message("More work on story 2...".to_string()),
        })
        .unwrap();
        app.process_loop_events();

        // Selection should stay on story 1 (user's choice)
        assert_eq!(app.loop_selected_story, 0);
    }

    #[test]
    fn navigate_between_completed_stories_while_agent_works() {
        use crate::agent::Response;

        let mut app = App::new();
        let (tx, rx) = mpsc::channel();
        app.loop_event_rx = Some(rx);
        app.loop_state.running = true;

        // Story 1: Completed
        tx.send(LoopEvent::StoryProgress {
            story_id: "1".to_string(),
            story_title: "First Story".to_string(),
            current: 1,
            total: 3,
            completed: 0,
        })
        .unwrap();
        tx.send(LoopEvent::StoryEvent {
            story_id: "1".to_string(),
            event: StreamEvent::Message("Working on story 1...".to_string()),
        })
        .unwrap();
        tx.send(LoopEvent::StoryEvent {
            story_id: "1".to_string(),
            event: StreamEvent::Done(Response {
                content: "Story 1 complete".to_string(),
                turns: 5,
                tokens: 1000,
                cost: 0.01,
            }),
        })
        .unwrap();

        // Story 2: Completed
        tx.send(LoopEvent::StoryProgress {
            story_id: "2".to_string(),
            story_title: "Second Story".to_string(),
            current: 2,
            total: 3,
            completed: 1,
        })
        .unwrap();
        tx.send(LoopEvent::StoryEvent {
            story_id: "2".to_string(),
            event: StreamEvent::Done(Response {
                content: "Story 2 complete".to_string(),
                turns: 3,
                tokens: 800,
                cost: 0.008,
            }),
        })
        .unwrap();

        // Story 3: In progress (agent currently working)
        tx.send(LoopEvent::StoryProgress {
            story_id: "3".to_string(),
            story_title: "Third Story".to_string(),
            current: 3,
            total: 3,
            completed: 2,
        })
        .unwrap();
        tx.send(LoopEvent::StoryEvent {
            story_id: "3".to_string(),
            event: StreamEvent::Message("Agent working on story 3...".to_string()),
        })
        .unwrap();

        app.process_loop_events();

        // Verify initial state: auto-selected to current (story 3)
        assert_eq!(app.loop_selected_story, 2);
        assert_eq!(app.loop_state.started_story_ids.len(), 3);
        assert_eq!(app.loop_state.current_story_id, Some("3".to_string()));

        // User navigates back to completed story 1
        app.navigate_to_previous_story();
        app.navigate_to_previous_story();
        assert_eq!(app.loop_selected_story, 0);
        assert_eq!(app.current_story(), Some("1"));

        // Verify story 1 events are accessible
        let story1_events = app.story_events.get("1").unwrap();
        assert_eq!(story1_events.len(), 2); // Message + Done

        // Navigate to completed story 2
        app.navigate_to_next_story();
        assert_eq!(app.loop_selected_story, 1);
        assert_eq!(app.current_story(), Some("2"));

        // Verify story 2 events are accessible
        let story2_events = app.story_events.get("2").unwrap();
        assert_eq!(story2_events.len(), 1); // Done only

        // Navigate to in-progress story 3
        app.navigate_to_next_story();
        assert_eq!(app.loop_selected_story, 2);
        assert_eq!(app.current_story(), Some("3"));

        // Cannot navigate beyond the current story
        assert!(!app.can_navigate_right());
        app.navigate_to_next_story();
        assert_eq!(app.loop_selected_story, 2); // Still on story 3
    }

    #[test]
    fn switch_result_tab_toggles_between_tasks_and_changed_files() {
        let mut app = App::new();
        assert_eq!(app.result_tab, ResultTab::Tasks);

        app.switch_result_tab();
        assert_eq!(app.result_tab, ResultTab::ChangedFiles);

        app.switch_result_tab();
        assert_eq!(app.result_tab, ResultTab::Tasks);
    }

    #[test]
    fn result_tasks_scroll_up_decreases_offset() {
        let mut app = App::new();
        app.result_tasks_scroll = 5;

        app.result_tasks_scroll_up();

        assert_eq!(app.result_tasks_scroll, 4);
    }

    #[test]
    fn result_tasks_scroll_down_increases_offset() {
        let mut app = App::new();
        app.result_tasks_scroll = 5;

        app.result_tasks_scroll_down();

        assert_eq!(app.result_tasks_scroll, 6);
    }

    #[test]
    fn result_tasks_scroll_stops_at_zero() {
        let mut app = App::new();
        app.result_tasks_scroll = 0;

        app.result_tasks_scroll_up();

        assert_eq!(app.result_tasks_scroll, 0);
    }

    #[test]
    fn result_scroll_respects_active_tab() {
        let mut app = App::new();
        // Default tab is Tasks
        app.result_tasks_scroll = 3;

        app.result_scroll_down();
        assert_eq!(app.result_tasks_scroll, 4);
        assert_eq!(app.result_scroll_offset, 0); // ChangedFiles tab unchanged

        // Switch to ChangedFiles tab
        app.switch_result_tab();
        app.result_scroll_down();
        assert_eq!(app.result_scroll_offset, 1);
        assert_eq!(app.result_tasks_scroll, 4); // Tasks tab unchanged
    }

    #[test]
    fn result_scroll_preserves_position_when_switching_tabs() {
        let mut app = App::new();

        // Scroll in Tasks tab
        app.result_tasks_scroll = 10;

        // Switch to ChangedFiles and scroll
        app.switch_result_tab();
        app.result_scroll_offset = 5;

        // Switch back to Tasks
        app.switch_result_tab();

        // Both scroll positions preserved
        assert_eq!(app.result_tasks_scroll, 10);
        assert_eq!(app.result_scroll_offset, 5);
    }

    #[test]
    fn with_max_retries_sets_value() {
        let app = App::new().with_max_retries(5);
        assert_eq!(app.max_retries, 5);
    }

    #[test]
    fn default_max_retries_is_default_constant() {
        let app = App::new();
        assert_eq!(app.max_retries, DEFAULT_MAX_RETRIES);
    }

    // ==================== Force-Quit Tests ====================

    #[test]
    fn force_quit_first_press_returns_graceful() {
        let mut app = App::new();
        app.loop_state.running = true;

        let action = app.handle_quit_press();

        assert_eq!(action, ForceQuitAction::Graceful);
        assert_eq!(app.quit_press_count, 1);
    }

    #[test]
    fn force_quit_second_press_returns_hint() {
        let mut app = App::new();
        app.loop_state.running = true;

        app.handle_quit_press(); // First press
        let action = app.handle_quit_press(); // Second press

        assert_eq!(action, ForceQuitAction::Hint);
        assert_eq!(app.quit_press_count, 2);
    }

    #[test]
    fn force_quit_third_press_returns_force_quit() {
        let mut app = App::new();
        app.loop_state.running = true;

        app.handle_quit_press(); // First press
        app.handle_quit_press(); // Second press
        let action = app.handle_quit_press(); // Third press

        assert_eq!(action, ForceQuitAction::ForceQuit);
        assert_eq!(app.quit_press_count, 3);
    }

    #[test]
    fn force_quit_when_not_running_returns_navigate_back() {
        let mut app = App::new();
        app.loop_state.running = false;

        let action = app.handle_quit_press();

        assert_eq!(action, ForceQuitAction::NavigateBack);
    }

    #[test]
    fn force_quit_resets_after_timeout() {
        use std::time::Duration;

        let mut app = App::new();
        app.loop_state.running = true;

        // Simulate a press that happened 5 seconds ago (outside the 3-second window)
        app.quit_press_count = 2;
        app.last_quit_time = Some(Instant::now() - Duration::from_secs(5));

        // This should reset the counter since we're outside the time window
        let action = app.handle_quit_press();

        assert_eq!(action, ForceQuitAction::Graceful);
        assert_eq!(app.quit_press_count, 1); // Reset to 1, not 3
    }

    #[test]
    fn reset_quit_counter_clears_state() {
        let mut app = App::new();
        app.quit_press_count = 2;
        app.last_quit_time = Some(Instant::now());

        app.reset_quit_counter();

        assert_eq!(app.quit_press_count, 0);
        assert!(app.last_quit_time.is_none());
    }

    #[test]
    fn force_quit_hint_shows_remaining_presses() {
        let mut app = App::new();
        app.loop_state.running = true;

        // After first press
        app.handle_quit_press();
        let hint = app.force_quit_hint();
        assert!(hint.is_some());
        assert!(hint.unwrap().contains("2 more times"));

        // After second press
        app.handle_quit_press();
        let hint = app.force_quit_hint();
        assert!(hint.is_some());
        assert!(hint.unwrap().contains("1 more time"));
    }

    #[test]
    fn force_quit_hint_none_when_not_running() {
        let mut app = App::new();
        app.loop_state.running = false;

        let hint = app.force_quit_hint();
        assert!(hint.is_none());
    }

    #[test]
    fn force_quit_hint_none_before_first_press() {
        let mut app = App::new();
        app.loop_state.running = true;
        app.quit_press_count = 0;

        let hint = app.force_quit_hint();
        assert!(hint.is_none());
    }
}
