//! Loop screen for displaying Ralph loop progress.
//!
//! This screen shows real-time progress during loop execution:
//! - Current change being processed
//! - Agent output log
//!
//! The agent manages its own progress by reading/editing tasks.md directly.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::ralph_loop::LoopState;
use super::{render_header as render_shared_header, HeaderContext};

/// Keybindings for the loop execution screen.
const LOOP_KEYBINDINGS: [&str; 1] = [
    "q Stop",
];

/// Renders the loop execution screen.
pub fn render_loop_screen(frame: &mut Frame, state: &LoopState, log: &[String]) {
    let area = frame.area();

    // Split into header, status, and log sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Header (3 content + 2 borders)
            Constraint::Length(3), // Status
            Constraint::Min(10),   // Log
        ])
        .split(area);

    // Header using shared component
    let status = if state.running { "Running" } else { "Stopped" };
    let context_info = format!("{} [{}]", state.change_name, status);

    let header_ctx = HeaderContext {
        title: "Loop Execution",
        context: Some(&context_info),
        keybindings: &LOOP_KEYBINDINGS,
    };
    render_shared_header(frame, chunks[0], &header_ctx);

    // Status
    render_status(frame, chunks[1], state);

    // Log
    render_log(frame, chunks[2], log);
}

fn render_status(frame: &mut Frame, area: Rect, state: &LoopState) {
    let status_text = if state.running {
        "Agent is working on the change. Progress is tracked via tasks.md edits."
    } else {
        "Agent has stopped. Check the log for details."
    };

    let status = Paragraph::new(status_text)
        .block(Block::default().title(" Status ").borders(Borders::ALL))
        .style(Style::default().fg(if state.running { Color::Green } else { Color::Yellow }));

    frame.render_widget(status, area);
}

fn render_log(frame: &mut Frame, area: Rect, log: &[String]) {
    let items: Vec<ListItem> = log
        .iter()
        .rev() // Show newest first
        .take(area.height as usize - 2) // Account for borders
        .map(|line| ListItem::new(line.as_str()))
        .collect();

    let log_list = List::new(items)
        .block(Block::default().title(" Log ").borders(Borders::ALL));

    frame.render_widget(log_list, area);
}
