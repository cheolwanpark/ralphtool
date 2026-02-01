use crate::ralph::{Scenario, Story, UserStory};
use crate::ralph::openspec::{ChangeInfo, OpenSpecAdapter};
use anyhow::Result;

/// The current screen being displayed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    /// Screen for selecting a completed OpenSpec change.
    ChangeSelection,
    /// Screen for previewing conversion results.
    ConversionPreview,
}

pub struct App {
    pub running: bool,
    /// Current screen being displayed.
    pub screen: Screen,
    /// Name of the selected change (if any).
    pub selected_change_name: Option<String>,
    /// Loaded stories from the selected change.
    pub stories: Vec<Story>,
    /// Loaded user stories from the selected change.
    pub user_stories: Vec<UserStory>,
    /// Loaded scenarios from the selected change.
    pub scenarios: Vec<Scenario>,
    /// List of available changes for selection.
    pub available_changes: Vec<ChangeInfo>,
    /// Currently selected index in the change selection list.
    pub selected_index: usize,
    /// Scroll offset for preview screen.
    pub scroll_offset: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            screen: Screen::ChangeSelection,
            selected_change_name: None,
            stories: Vec::new(),
            user_stories: Vec::new(),
            scenarios: Vec::new(),
            available_changes: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Loads the list of available changes.
    pub fn load_changes(&mut self) -> Result<()> {
        self.available_changes = OpenSpecAdapter::list_changes()?;
        // Filter to only show completed changes
        self.available_changes.retain(|c| {
            OpenSpecAdapter::is_complete(&c.name).unwrap_or(false)
        });
        Ok(())
    }

    /// Loads data from the selected change.
    pub fn load_selected_change(&mut self) -> Result<()> {
        if let Some(ref name) = self.selected_change_name {
            let adapter = OpenSpecAdapter::new(name)?;
            self.stories = adapter.list_tasks()?;
            self.user_stories = adapter.list_stories()?;
            self.scenarios = adapter.list_scenarios()?;
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

    /// Scrolls up in the preview screen.
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Scrolls down in the preview screen.
    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    /// Page up in the preview screen.
    pub fn page_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(10);
    }

    /// Page down in the preview screen.
    pub fn page_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(10);
    }
}

// Import traits for method resolution
use crate::ralph::{StoryProvider, TaskSource, VerificationSource};
