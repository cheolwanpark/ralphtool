//! Conversion preview screen for displaying Ralph domain data.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::{App, PreviewTab};
use super::{centered_rect, render_header_auto, HeaderSection};

/// Keybindings for the preview screen (single string for new header format).
const PREVIEW_KEYBINDINGS: &str = "↑↓ Scroll  Tab Switch  R Run  Esc Back  q Quit";

pub fn render_preview(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Center the content using responsive width
    let centered = centered_rect(area);

    // Build description with change name and counts as context
    let change_name = app.selected_change_name.as_deref().unwrap_or("Unknown");
    let task_count: usize = app.stories.iter().flat_map(|s| &s.tasks).count();
    let story_count = app.stories.len();
    let scenario_count = app.scenarios.len();

    let description = format!(
        "{}: {} tasks, {} stories, {} scenarios",
        change_name, task_count, story_count, scenario_count
    );

    // Header section data
    let header = HeaderSection {
        title: "◆ Preview",
        description: &description,
        keybindings: PREVIEW_KEYBINDINGS,
    };

    // Render header (auto-selects full or compact based on terminal height)
    let header_height = render_header_auto(frame, centered, &header);

    // Calculate content area (remaining space after header)
    let content_y = centered.y + header_height;
    let content_height = centered.height.saturating_sub(header_height);
    let content_area = Rect::new(centered.x, content_y, centered.width, content_height);

    // Split content area into tab bar and main content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Tab bar
            Constraint::Min(1),    // Content
        ])
        .split(content_area);

    // Render tab bar
    render_tab_bar(frame, app, chunks[0]);

    // Render content based on active tab
    let lines = match app.active_tab {
        PreviewTab::Tasks => render_tasks_tab(app),
        PreviewTab::Scenarios => render_scenarios_tab(app),
    };

    // Create paragraph with native scroll
    let scroll_offset = app.get_scroll_offset() as u16;
    let content = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0));
    frame.render_widget(content, chunks[1]);
}

fn render_tab_bar(frame: &mut Frame, app: &App, area: Rect) {
    let tasks_label = match app.active_tab {
        PreviewTab::Tasks => "[Tasks]",
        PreviewTab::Scenarios => "Tasks",
    };
    let scenarios_label = match app.active_tab {
        PreviewTab::Tasks => "Scenarios",
        PreviewTab::Scenarios => "[Scenarios]",
    };

    let tab_line = Line::from(vec![
        Span::raw(" "),
        Span::styled(
            tasks_label,
            if app.active_tab == PreviewTab::Tasks {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
        Span::styled(" | ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            scenarios_label,
            if app.active_tab == PreviewTab::Scenarios {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
    ]);

    let tab_bar = Paragraph::new(tab_line);
    frame.render_widget(tab_bar, area);
}

fn render_tasks_tab(app: &App) -> Vec<Line<'_>> {
    let mut lines: Vec<Line> = Vec::new();

    for story in &app.stories {
        lines.push(Line::from(vec![
            Span::styled("▸ ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("Story {}: {}", story.id, story.title),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]));

        for task in &story.tasks {
            let checkbox = if task.done { "[x]" } else { "[ ]" };
            let style = if task.done {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(checkbox, style),
                Span::raw(" "),
                Span::styled(&task.id, Style::default().fg(Color::DarkGray)),
                Span::raw(" "),
                Span::raw(&task.description),
            ]));
        }
        lines.push(Line::from(""));
    }

    lines
}

fn render_scenarios_tab(app: &App) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    // Group scenarios by capability (alphabetically sorted)
    for capability in app.unique_capabilities() {
        let scenarios = app.scenarios_for_capability(&capability);
        if scenarios.is_empty() {
            continue;
        }

        // Capability header
        lines.push(Line::from(vec![
            Span::styled("▸ ", Style::default().fg(Color::Yellow)),
            Span::styled(
                capability.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]));

        // Group scenarios by requirement_id within capability
        let mut requirements: Vec<String> = scenarios
            .iter()
            .map(|s| s.requirement_id.clone())
            .collect();
        requirements.sort();
        requirements.dedup();

        for requirement_id in requirements {
            // Requirement sub-header
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled("● ", Style::default().fg(Color::Cyan)),
                Span::styled(requirement_id.clone(), Style::default().fg(Color::White)),
            ]));

            // Scenarios under this requirement
            for scenario in scenarios.iter().filter(|s| s.requirement_id == requirement_id) {
                // Scenario header (indented under requirement)
                lines.push(Line::from(vec![
                    Span::raw("        "),
                    Span::styled("◦ ", Style::default().fg(Color::Magenta)),
                    Span::styled(scenario.name.clone(), Style::default().fg(Color::Cyan)),
                ]));

                // Given steps
                for given in &scenario.given {
                    lines.push(Line::from(vec![
                        Span::raw("            "),
                        Span::styled("GIVEN ", Style::default().fg(Color::Blue)),
                        Span::raw(given.clone()),
                    ]));
                }

                // When step
                if !scenario.when.is_empty() {
                    lines.push(Line::from(vec![
                        Span::raw("            "),
                        Span::styled("WHEN ", Style::default().fg(Color::Magenta)),
                        Span::raw(scenario.when.clone()),
                    ]));
                }

                // Then steps
                for then in &scenario.then {
                    lines.push(Line::from(vec![
                        Span::raw("            "),
                        Span::styled("THEN ", Style::default().fg(Color::Green)),
                        Span::raw(then.clone()),
                    ]));
                }
            }
        }

        lines.push(Line::from(""));
    }

    lines
}
