//! Conversion preview screen for displaying Ralph domain data.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::{App, PreviewTab};

pub fn render_preview(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create main layout with header, tab bar, content, and help
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header
            Constraint::Length(1), // Tab bar
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

    // Render tab bar
    render_tab_bar(frame, app, chunks[1]);

    // Render content based on active tab
    let lines = match app.active_tab {
        PreviewTab::Tasks => render_tasks_tab(app),
        PreviewTab::UserStories => render_user_stories_tab(app),
    };

    // Apply scroll offset
    let visible_lines: Vec<Line> = lines
        .into_iter()
        .skip(app.get_scroll_offset())
        .collect();

    let content = Paragraph::new(visible_lines)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false });
    frame.render_widget(content, chunks[2]);

    // Help text
    let help = Paragraph::new("↑↓ Scroll  PgUp/PgDn Page  Tab/Shift+Tab Switch  Esc Back  q Quit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(help, chunks[3]);
}

fn render_tab_bar(frame: &mut Frame, app: &App, area: Rect) {
    let tasks_label = match app.active_tab {
        PreviewTab::Tasks => "[Tasks]",
        PreviewTab::UserStories => "Tasks",
    };
    let stories_label = match app.active_tab {
        PreviewTab::Tasks => "User Stories",
        PreviewTab::UserStories => "[User Stories]",
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
            stories_label,
            if app.active_tab == PreviewTab::UserStories {
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

    lines
}

fn render_user_stories_tab(app: &App) -> Vec<Line<'_>> {
    let mut lines: Vec<Line> = Vec::new();

    for story in &app.user_stories {
        // User Story header
        lines.push(Line::from(vec![
            Span::styled("▸ ", Style::default().fg(Color::Yellow)),
            Span::styled(&story.title, Style::default().add_modifier(Modifier::BOLD)),
        ]));

        // Story description
        if !story.description.is_empty() {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(&story.description, Style::default().fg(Color::DarkGray)),
            ]));
        }

        // Nested scenarios for this user story
        let scenarios = app.scenarios_for_story(&story.id);
        if !scenarios.is_empty() {
            for scenario in scenarios {
                // Scenario header (indented under user story)
                lines.push(Line::from(vec![
                    Span::raw("    "),
                    Span::styled("◦ ", Style::default().fg(Color::Magenta)),
                    Span::styled(&scenario.name, Style::default().fg(Color::Cyan)),
                ]));

                // Given steps
                for given in &scenario.given {
                    lines.push(Line::from(vec![
                        Span::raw("        "),
                        Span::styled("GIVEN ", Style::default().fg(Color::Blue)),
                        Span::raw(given.as_str()),
                    ]));
                }

                // When step
                if !scenario.when.is_empty() {
                    lines.push(Line::from(vec![
                        Span::raw("        "),
                        Span::styled("WHEN ", Style::default().fg(Color::Magenta)),
                        Span::raw(scenario.when.as_str()),
                    ]));
                }

                // Then steps
                for then in &scenario.then {
                    lines.push(Line::from(vec![
                        Span::raw("        "),
                        Span::styled("THEN ", Style::default().fg(Color::Green)),
                        Span::raw(then.as_str()),
                    ]));
                }
            }
        }

        lines.push(Line::from(""));
    }

    lines
}
