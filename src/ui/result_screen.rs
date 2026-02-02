//! Result screen for reviewing changes after loop completion.
//!
//! This screen displays:
//! - Summary of completed work
//! - Tabbed interface with Tasks and Changed Files tabs

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
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
    // Build lines from stories and their tasks
    let mut lines: Vec<Line> = Vec::new();
    for story in &result.stories {
        // Story title line
        lines.push(Line::from(Span::styled(
            format!("## {}", story.title),
            Style::default().add_modifier(Modifier::BOLD),
        )));

        // Task lines with checkboxes
        for task in &story.tasks {
            let checkbox = if task.done { "[x]" } else { "[ ]" };
            lines.push(Line::from(format!("  {} {}", checkbox, task.description)));
        }

        // Empty line after each story
        lines.push(Line::from(""));
    }

    // Create paragraph to calculate actual rendered line count
    let block = Block::default().borders(Borders::ALL);
    let inner_area = block.inner(area);
    let paragraph = Paragraph::new(lines).block(block);

    // Get actual rendered line count
    let total_lines = paragraph.line_count(inner_area.width);
    let visible_height = inner_area.height as usize;
    let max_scroll = total_lines.saturating_sub(visible_height);

    // Clamp scroll offset to valid bounds
    let clamped_scroll = scroll_offset.min(max_scroll) as u16;

    // Apply native scroll
    let content = paragraph.scroll((clamped_scroll, 0));
    frame.render_widget(content, area);
}

/// Renders the Changed Files tab with color-coded file status.
fn render_changed_files(frame: &mut Frame, area: Rect, result: &LoopResult, scroll_offset: usize) {
    // Build lines with color-coded status character only (not filename)
    let lines: Vec<Line> = result
        .changed_files
        .iter()
        .map(|file| {
            // Split into status character and filename
            // Format is typically "M\tfilename" or "M filename"
            if file.is_empty() {
                return Line::from(Span::raw(""));
            }

            let status = &file[0..1];
            let filename = file[1..].trim_start();

            let status_style = match status {
                "A" => Style::default().fg(Color::Green),
                "D" => Style::default().fg(Color::Red),
                "M" => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            };

            Line::from(vec![
                Span::styled(status, status_style),
                Span::raw(" "),
                Span::raw(filename),
            ])
        })
        .collect();

    let title = format!(" Changed Files ({}) ", result.changed_files.len());

    // Create paragraph to calculate actual rendered line count
    let block = Block::default().title(title).borders(Borders::ALL);
    let inner_area = block.inner(area);
    let paragraph = Paragraph::new(lines).block(block);

    // Get actual rendered line count
    let total_lines = paragraph.line_count(inner_area.width);
    let visible_height = inner_area.height as usize;
    let max_scroll = total_lines.saturating_sub(visible_height);

    // Clamp scroll offset to valid bounds
    let clamped_scroll = scroll_offset.min(max_scroll) as u16;

    // Apply native scroll
    let content = paragraph.scroll((clamped_scroll, 0));
    frame.render_widget(content, area);
}
