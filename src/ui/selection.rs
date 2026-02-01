//! Change selection screen for browsing and selecting OpenSpec changes.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;
use super::{render_header, HeaderContext};

/// Keybindings for the selection screen.
const SELECTION_KEYBINDINGS: [&str; 3] = [
    "↑↓ Navigate",
    "Enter Select",
    "q Quit",
];

pub fn render_selection(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create main layout with header and list
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Header (5 lines including borders)
            Constraint::Min(5),    // List
        ])
        .split(area);

    // Header
    let header_ctx = HeaderContext {
        title: "Selection",
        context: Some("Select a Completed Change"),
        keybindings: &SELECTION_KEYBINDINGS,
    };
    render_header(frame, chunks[0], &header_ctx);

    // Change list or empty state
    if app.available_changes.is_empty() {
        let empty = Paragraph::new("No completed changes available")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title(" Changes "));
        frame.render_widget(empty, chunks[1]);
    } else {
        let items: Vec<ListItem> = app
            .available_changes
            .iter()
            .enumerate()
            .map(|(i, change)| {
                let content = format!(
                    "  {} ({}/{} tasks) - {}",
                    change.name,
                    change.completed_tasks,
                    change.total_tasks,
                    &change.last_modified[..10] // Just the date part
                );
                let style = if i == app.selected_index {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Changes "))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_widget(list, chunks[1]);
    }
}
