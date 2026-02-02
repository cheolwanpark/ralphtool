//! Result screen for reviewing changes after loop completion.
//!
//! This screen displays:
//! - Summary of completed work
//! - Tabbed interface with Tasks and Changed Files tabs

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use super::{centered_rect, render_header_auto, HeaderSection};
use crate::app::ResultTab;
use crate::spec::Story;

/// Keybindings for the result screen (single string for new header format).
const RESULT_KEYBINDINGS: &str = "↑↓ Scroll  Tab Switch  Esc Back  q Quit";

/// Result data for display.
#[derive(Debug, Clone, Default)]
pub struct LoopResult {
    /// Name of the change that was processed.
    pub change_name: String,

    /// Number of stories completed.
    pub stories_completed: usize,

    /// Total number of stories.
    pub stories_total: usize,

    /// Number of tasks completed.
    pub tasks_completed: usize,

    /// Total number of tasks.
    pub tasks_total: usize,

    /// Files that were changed during the loop.
    pub changed_files: Vec<String>,

    /// Stories with tasks for display in Tasks tab.
    pub stories: Vec<Story>,
}

/// Renders the result review screen.
pub fn render_result_screen(
    frame: &mut Frame,
    result: &LoopResult,
    active_tab: ResultTab,
    tasks_scroll: usize,
    files_scroll: usize,
) {
    let area = frame.area();

    // Center the content using responsive width
    let centered = centered_rect(area);

    // Build description with change name and completion status
    let description = format!("Loop Complete: {}", result.change_name);

    // Header section data
    let header = HeaderSection {
        title: "◆ Result",
        description: &description,
        keybindings: RESULT_KEYBINDINGS,
    };

    // Render header (auto-selects full or compact based on terminal height)
    let header_height = render_header_auto(frame, centered, &header);

    // Calculate content area (remaining space after header)
    let content_y = centered.y + header_height;
    let content_height = centered.height.saturating_sub(header_height);
    let content_area = Rect::new(centered.x, content_y, centered.width, content_height);

    // Split content area into summary and tabbed content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Summary
            Constraint::Length(3), // Tab bar
            Constraint::Min(5),    // Tab content
        ])
        .split(content_area);

    // Summary
    render_summary(frame, chunks[0], result);

    // Tab bar
    render_tabs(frame, chunks[1], active_tab);

    // Tab content
    match active_tab {
        ResultTab::Tasks => render_tasks_tab(frame, chunks[2], result, tasks_scroll),
        ResultTab::ChangedFiles => render_changed_files(frame, chunks[2], result, files_scroll),
    }
}

fn render_summary(frame: &mut Frame, area: Rect, result: &LoopResult) {
    let summary = format!(
        "Stories: {}/{} completed\n\
         Tasks: {}/{} completed",
        result.stories_completed, result.stories_total, result.tasks_completed, result.tasks_total
    );

    let summary_widget = Paragraph::new(summary)
        .block(Block::default().title(" Summary ").borders(Borders::ALL));
    frame.render_widget(summary_widget, area);
}

/// Renders the tab bar with Tasks and Changed Files tabs.
fn render_tabs(frame: &mut Frame, area: Rect, active_tab: ResultTab) {
    let tasks_style = if active_tab == ResultTab::Tasks {
        Style::default().fg(Color::Black).bg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let files_style = if active_tab == ResultTab::ChangedFiles {
        Style::default().fg(Color::Black).bg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let tabs = Line::from(vec![
        Span::raw(" "),
        Span::styled(" Tasks ", tasks_style),
        Span::raw("  "),
        Span::styled(" Changed Files ", files_style),
        Span::raw(" "),
    ]);

    let tabs_widget = Paragraph::new(tabs)
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(tabs_widget, area);
}

/// Renders the Tasks tab showing stories with task checkboxes.
fn render_tasks_tab(frame: &mut Frame, area: Rect, result: &LoopResult, scroll_offset: usize) {
    let visible_height = (area.height as usize).saturating_sub(2); // Account for borders

    // Build list items from stories and their tasks
    let mut items: Vec<ListItem> = Vec::new();
    for story in &result.stories {
        // Story title line
        items.push(ListItem::new(format!("## {}", story.title)).style(Style::default().bold()));

        // Task lines with checkboxes
        for task in &story.tasks {
            let checkbox = if task.done { "[x]" } else { "[ ]" };
            items.push(ListItem::new(format!("  {} {}", checkbox, task.description)));
        }

        // Empty line after each story
        items.push(ListItem::new(""));
    }

    // Apply scrolling
    let visible_items: Vec<ListItem> = items
        .into_iter()
        .skip(scroll_offset)
        .take(visible_height)
        .collect();

    let tasks_list = List::new(visible_items)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(tasks_list, area);
}

/// Renders the Changed Files tab with color-coded file status.
fn render_changed_files(frame: &mut Frame, area: Rect, result: &LoopResult, scroll_offset: usize) {
    let visible_height = (area.height as usize).saturating_sub(2); // Account for borders

    let items: Vec<ListItem> = result
        .changed_files
        .iter()
        .skip(scroll_offset)
        .take(visible_height)
        .map(|file| {
            let style = if file.starts_with('A') {
                Style::default().fg(Color::Green)
            } else if file.starts_with('D') {
                Style::default().fg(Color::Red)
            } else if file.starts_with('M') {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(file.as_str()).style(style)
        })
        .collect();

    let title = format!(" Changed Files ({}) ", result.changed_files.len());
    let files_list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL));

    frame.render_widget(files_list, area);
}
