//! Conversion preview screen for displaying Ralph domain data.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

pub fn render_preview(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create main layout with header and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header
            Constraint::Min(10),   // Content
            Constraint::Length(3), // Help
        ])
        .split(area);

    // Header with change name and counts
    let change_name = app.selected_change_name.as_deref().unwrap_or("Unknown");
    let task_count: usize = app.stories.iter().flat_map(|s| &s.tasks).count();
    let story_count = app.user_stories.len();
    let scenario_count = app.scenarios.len();

    let header_text = vec![
        Line::from(vec![
            Span::styled("Change: ", Style::default().fg(Color::DarkGray)),
            Span::styled(change_name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(
                format!("{} tasks, {} stories, {} scenarios", task_count, story_count, scenario_count),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title(" Preview "));
    frame.render_widget(header, chunks[0]);

    // Build content lines
    let mut lines: Vec<Line> = Vec::new();

    // Tasks section
    lines.push(Line::from(Span::styled(
        "═══ TASKS ═══",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    for story in &app.stories {
        lines.push(Line::from(vec![
            Span::styled("▸ ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("Story {}: {}", story.id, story.title),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]));

        for task in &story.tasks {
            let checkbox = if task.complete { "[x]" } else { "[ ]" };
            let style = if task.complete {
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

    // Stories section
    lines.push(Line::from(Span::styled(
        "═══ USER STORIES ═══",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    for story in &app.user_stories {
        lines.push(Line::from(vec![
            Span::styled("▸ ", Style::default().fg(Color::Yellow)),
            Span::styled(&story.title, Style::default().add_modifier(Modifier::BOLD)),
        ]));

        if !story.description.is_empty() {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(&story.description, Style::default().fg(Color::DarkGray)),
            ]));
        }

        if !story.acceptance_criteria.is_empty() {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled("Acceptance Criteria:", Style::default().fg(Color::Green)),
            ]));
            for criteria in &story.acceptance_criteria {
                lines.push(Line::from(vec![
                    Span::raw("      • "),
                    Span::raw(criteria),
                ]));
            }
        }
        lines.push(Line::from(""));
    }

    // Scenarios section
    lines.push(Line::from(Span::styled(
        "═══ SCENARIOS ═══",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    for scenario in &app.scenarios {
        lines.push(Line::from(vec![
            Span::styled("▸ ", Style::default().fg(Color::Yellow)),
            Span::styled(&scenario.name, Style::default().add_modifier(Modifier::BOLD)),
        ]));

        for given in &scenario.given {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled("GIVEN ", Style::default().fg(Color::Blue)),
                Span::raw(given),
            ]));
        }

        if !scenario.when.is_empty() {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled("WHEN ", Style::default().fg(Color::Magenta)),
                Span::raw(&scenario.when),
            ]));
        }

        for then in &scenario.then {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled("THEN ", Style::default().fg(Color::Green)),
                Span::raw(then),
            ]));
        }
        lines.push(Line::from(""));
    }

    // Apply scroll offset
    let visible_lines: Vec<Line> = lines
        .into_iter()
        .skip(app.scroll_offset)
        .collect();

    let content = Paragraph::new(visible_lines)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false });
    frame.render_widget(content, chunks[1]);

    // Help text
    let help = Paragraph::new("↑↓ Scroll  PgUp/PgDn Page  Esc Back  q Quit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, chunks[2]);
}
