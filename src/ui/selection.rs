//! Change selection screen for browsing and selecting OpenSpec changes.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render_selection(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create main layout with title, list, and help
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(5),    // List
            Constraint::Length(3), // Help
        ])
        .split(area);

    // Title
    let title = Paragraph::new("Select a Completed Change")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" ralphtool "));
    frame.render_widget(title, chunks[0]);

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

    // Help text
    let help = Paragraph::new("↑↓ Navigate  Enter Select  q Quit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, chunks[2]);
}
