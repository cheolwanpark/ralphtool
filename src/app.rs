use crate::spec::{Scenario, Story};
use crate::spec::openspec::{ChangeInfo, OpenSpecAdapter};
use crate::spec::SpecAdapter;
use anyhow::Result;

/// The current screen being displayed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    /// Screen for selecting a completed OpenSpec change.
    ChangeSelection,
    /// Screen for previewing conversion results.
    ConversionPreview,
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

    /// Returns scenarios that belong to a specific story.
    pub fn scenarios_for_story(&self, story_id: &str) -> Vec<&Scenario> {
        self.scenarios
            .iter()
            .filter(|scenario| scenario.story_id == story_id)
            .collect()
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
}
