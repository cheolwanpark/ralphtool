//! Change selection screen for browsing and selecting OpenSpec changes.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;
use super::{centered_rect, render_header_auto, HeaderSection};

/// Keybindings for the selection screen (single string for new header format).
const SELECTION_KEYBINDINGS: &str = "↑↓ Navigate  Enter Select  q Quit";

pub fn render_selection(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Center the content using responsive width
    let centered = centered_rect(area);

    // Header section data
    let header = HeaderSection {
        title: "◆ Change Selection",
        description: "Select a change to preview and run",
        keybindings: SELECTION_KEYBINDINGS,
    };

    // Render header (auto-selects full or compact based on terminal height)
    let header_height = render_header_auto(frame, centered, &header);

    // Calculate content area (remaining space after header)
    // Using percentage-based approach: header ~20%, content ~80%
    let content_y = centered.y + header_height;
    let content_height = centered.height.saturating_sub(header_height);
    let content_area = Rect::new(centered.x, content_y, centered.width, content_height);

    // Change list or empty state
    if app.available_changes.is_empty() {
        let empty = Paragraph::new("No completed changes available")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title(" Changes "));
        frame.render_widget(empty, content_area);
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
        frame.render_widget(list, content_area);
    }
}
