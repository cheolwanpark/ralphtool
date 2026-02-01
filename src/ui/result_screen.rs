//! Result screen for reviewing changes after loop completion.
//!
//! This screen displays:
//! - Summary of completed work
//! - Changed files (from git diff)
//! - Verification status

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use super::{render_header as render_shared_header, HeaderContext};

/// Keybindings for the result screen.
const RESULT_KEYBINDINGS: [&str; 3] = [
    "↑↓ Scroll",
    "Esc Back",
    "q Quit",
];

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

    /// Files that were changed during the loop.
    pub changed_files: Vec<String>,

    /// Verification status messages.
    pub verification_status: Vec<VerificationResult>,
}

/// Result of a verification check.
#[derive(Debug, Clone)]
pub struct VerificationResult {
    /// Name of the check.
    pub name: String,
    /// Whether the check passed.
    pub passed: bool,
    /// Output or error message.
    #[allow(dead_code)]
    pub message: Option<String>,
}

/// Renders the result review screen.
pub fn render_result_screen(frame: &mut Frame, result: &LoopResult, scroll_offset: usize) {
    let area = frame.area();

    // Split into header, summary, files, and verification sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Header (3 content + 2 borders)
            Constraint::Length(5), // Summary
            Constraint::Min(5),    // Changed files
            Constraint::Length(6), // Verification
        ])
        .split(area);

    // Header using shared component
    let context_info = format!("Loop Complete: {}", result.change_name);
    let header_ctx = HeaderContext {
        title: "Result",
        context: Some(&context_info),
        keybindings: &RESULT_KEYBINDINGS,
    };
    render_shared_header(frame, chunks[0], &header_ctx);

    // Summary
    render_summary(frame, chunks[1], result);

    // Changed files
    render_changed_files(frame, chunks[2], result, scroll_offset);

    // Verification status
    render_verification(frame, chunks[3], result);
}

fn render_summary(frame: &mut Frame, area: Rect, result: &LoopResult) {
    let summary = format!(
        "Stories: {}/{}\n\
         Tasks: {}",
        result.stories_completed, result.stories_total, result.tasks_completed
    );

    let summary_widget = Paragraph::new(summary)
        .block(Block::default().title(" Summary ").borders(Borders::ALL));
    frame.render_widget(summary_widget, area);
}

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

fn render_verification(frame: &mut Frame, area: Rect, result: &LoopResult) {
    let items: Vec<ListItem> = result
        .verification_status
        .iter()
        .map(|v| {
            let (symbol, style) = if v.passed {
                ("✓", Style::default().fg(Color::Green))
            } else {
                ("✗", Style::default().fg(Color::Red))
            };
            ListItem::new(format!("{} {}", symbol, v.name)).style(style)
        })
        .collect();

    let verification_list = List::new(items)
        .block(Block::default().title(" Verification ").borders(Borders::ALL));

    frame.render_widget(verification_list, area);
}
