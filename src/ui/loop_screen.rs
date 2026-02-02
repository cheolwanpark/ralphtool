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
use super::{centered_rect, render_header_auto, HeaderSection};

/// Keybindings for the loop execution screen (single string for new header format).
const LOOP_KEYBINDINGS: &str = "q Stop";

/// Renders the loop execution screen.
pub fn render_loop_screen(frame: &mut Frame, state: &LoopState, log: &[String]) {
    let area = frame.area();

    // Center the content using responsive width
    let centered = centered_rect(area);

    // Build description with change name and running status
    let status_text = if state.running { "Running" } else { "Stopped" };
    let description = format!("{} [{}]", state.change_name, status_text);

    // Header section data
    let header = HeaderSection {
        title: "◆ Loop Execution",
        description: &description,
        keybindings: LOOP_KEYBINDINGS,
    };

    // Render header (auto-selects full or compact based on terminal height)
    let header_height = render_header_auto(frame, centered, &header);

    // Calculate content area (remaining space after header)
    let content_y = centered.y + header_height;
    let content_height = centered.height.saturating_sub(header_height);
    let content_area = Rect::new(centered.x, content_y, centered.width, content_height);

    // Split content area into status and log sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Status
            Constraint::Min(10),   // Log
        ])
        .split(content_area);

    // Status
    render_status(frame, chunks[0], state);

    // Log
    render_log(frame, chunks[1], log);
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
