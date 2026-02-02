//! Loop screen for displaying Ralph loop progress.
//!
//! This screen shows real-time progress during loop execution:
//! - Progress bar with change name and completion ratio
//! - Story indicator with sliding window (max 5 visible)
//! - Tabbed content (Info/Agent) with scroll support
//!
//! The agent manages its own progress by reading/editing tasks.md directly.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
};

use crate::agent::{Response, StreamEvent};
use crate::app::{App, LoopTab};
use crate::ralph_loop::LoopState;
use super::{centered_rect, render_header_auto, HeaderSection};

/// Keybindings for the loop execution screen.
const LOOP_KEYBINDINGS: &str = "←→ Story  Tab Switch  ↑↓ Scroll  q Stop";

/// Renders the loop execution screen.
pub fn render_loop_screen(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Center the content using responsive width
    let centered = centered_rect(area);

    // Build description with change name and running status
    let status_text = if app.loop_state.running { "Running" } else { "Stopped" };
    let description = format!("{} [{}]", app.loop_state.change_name, status_text);

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

    // Split content area into progress bar, story indicator, tab bar, and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Progress bar with block
            Constraint::Length(1), // Story indicator
            Constraint::Length(1), // Tab bar
            Constraint::Min(5),    // Content area
        ])
        .split(content_area);

    // Render progress bar
    render_progress_bar(frame, chunks[0], &app.loop_state);

    // Render story indicator
    render_story_indicator(frame, chunks[1], app);

    // Render tab bar
    render_tab_bar(frame, chunks[2], app.loop_tab);

    // Render content based on active tab
    match app.loop_tab {
        LoopTab::Info => render_info_tab(frame, chunks[3], app),
        LoopTab::Agent => render_agent_tab(frame, chunks[3], app),
    }
}

/// Renders a progress bar showing change name and completion ratio.
///
/// Display format: "change-name [=========>        ] 3/10"
fn render_progress_bar(frame: &mut Frame, area: Rect, state: &LoopState) {
    let completed = state.completed_stories;
    let total = state.total_stories;

    // Calculate ratio (avoid division by zero)
    let ratio = if total > 0 {
        completed as f64 / total as f64
    } else {
        0.0
    };

    // Build label with completion count
    let label = format!("{}/{}", completed, total);

    let gauge = Gauge::default()
        .block(Block::default().title(format!(" {} ", state.change_name)).borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .ratio(ratio)
        .label(label);

    frame.render_widget(gauge, area);
}

/// Renders the story indicator with sliding window logic.
///
/// Visual states:
/// - Current (in progress): Green color
/// - Completed: Default color
/// - Selected: Underline
///
/// When there are more than 5 stories, shows a sliding window of 5 centered on selection.
fn render_story_indicator(frame: &mut Frame, area: Rect, app: &App) {
    let started = &app.loop_state.started_story_ids;
    if started.is_empty() {
        let empty_line = Line::from(Span::styled(
            " Stories: (none started)",
            Style::default().fg(Color::DarkGray),
        ));
        frame.render_widget(Paragraph::new(empty_line), area);
        return;
    }

    let current_story_id = app.loop_state.current_story_id.as_deref();
    let selected_idx = app.loop_selected_story;
    let visible = app.visible_stories();

    // Find the index offset for visible window
    let offset = if started.len() <= 5 || selected_idx <= 2 {
        0
    } else if selected_idx >= started.len() - 2 {
        started.len() - 5
    } else {
        selected_idx - 2
    };

    let mut spans: Vec<Span> = vec![Span::raw(" Stories: ")];

    // Add ellipsis if there are stories before the window
    if offset > 0 {
        spans.push(Span::styled("... ", Style::default().fg(Color::DarkGray)));
    }

    for (i, story_id) in visible.iter().enumerate() {
        let actual_idx = offset + i;
        let is_current = current_story_id == Some(*story_id);
        let is_selected = actual_idx == selected_idx;

        // Check if story is completed (has a Done event)
        let is_completed = app.story_events
            .get(*story_id)
            .map(|events| events.iter().any(|e| matches!(e, StreamEvent::Done(_))))
            .unwrap_or(false);

        // Build style: green for current, underline for selected
        let mut style = Style::default();
        if is_current && !is_completed {
            style = style.fg(Color::Green);
        }
        if is_selected {
            style = style.add_modifier(Modifier::UNDERLINED);
        }

        // Show story number (1-indexed)
        let display_num = actual_idx + 1;
        let label = format!("{}", display_num);

        spans.push(Span::styled(label, style));

        if i < visible.len() - 1 {
            spans.push(Span::raw(" "));
        }
    }

    // Add ellipsis if there are stories after the window
    if offset + 5 < started.len() {
        spans.push(Span::styled(" ...", Style::default().fg(Color::DarkGray)));
    }

    let line = Line::from(spans);
    frame.render_widget(Paragraph::new(line), area);
}

/// Renders the tab bar for switching between Info and Agent tabs.
fn render_tab_bar(frame: &mut Frame, area: Rect, active_tab: LoopTab) {
    let info_label = match active_tab {
        LoopTab::Info => "[Info]",
        LoopTab::Agent => "Info",
    };
    let agent_label = match active_tab {
        LoopTab::Info => "Agent",
        LoopTab::Agent => "[Agent]",
    };

    let tab_line = Line::from(vec![
        Span::raw(" "),
        Span::styled(
            info_label,
            if active_tab == LoopTab::Info {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
        Span::styled(" | ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            agent_label,
            if active_tab == LoopTab::Agent {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
    ]);

    let tab_bar = Paragraph::new(tab_line);
    frame.render_widget(tab_bar, area);
}

/// Renders the Info tab showing story title and task list with checkboxes.
///
/// Display format:
/// ```text
/// Story 5: Loop Screen UI Rewrite
///
///   ☐ 5.1
///     Create render_progress_bar() function that handles the gauge
///     widget and displays completion ratio
///   ☑ 5.2
///     Create render_story_indicator() function
/// ```
fn render_info_tab(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = Vec::new();

    // Get selected story ID
    let selected_story_id = app.current_story();

    if let Some(story_id) = selected_story_id {
        // Find the story in app.stories
        if let Some(story) = app.stories.iter().find(|s| s.id == story_id) {
            // Story header
            lines.push(Line::from(vec![
                Span::styled("Story ", Style::default().fg(Color::Yellow)),
                Span::styled(&story.id, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(": ", Style::default().fg(Color::Yellow)),
                Span::styled(&story.title, Style::default().add_modifier(Modifier::BOLD)),
            ]));
            lines.push(Line::from(""));

            // Task list with checkboxes
            for task in &story.tasks {
                let checkbox = if task.done { "☑" } else { "☐" };
                let checkbox_style = if task.done {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                let text_style = if task.done {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default()
                };

                // First line: checkbox + task ID
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(checkbox, checkbox_style),
                    Span::raw(" "),
                    Span::styled(&task.id, Style::default().fg(Color::DarkGray)),
                ]));

                // Second line: description with 4-space indentation
                // Paragraph::wrap() will handle line wrapping naturally
                lines.push(Line::from(vec![
                    Span::raw("    "), // 4 spaces indentation
                    Span::styled(task.description.clone(), text_style),
                ]));
            }
        } else {
            // Story not found in loaded stories - show ID only
            lines.push(Line::from(vec![
                Span::styled("Story ID: ", Style::default().fg(Color::Yellow)),
                Span::styled(story_id, Style::default().add_modifier(Modifier::BOLD)),
            ]));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "(Story details not available)",
                Style::default().fg(Color::DarkGray),
            )));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "No story selected",
            Style::default().fg(Color::DarkGray),
        )));
    }

    // Create paragraph with native scroll
    let scroll_offset = app.loop_info_scroll as u16;
    let content = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0));
    frame.render_widget(content, area);
}

/// Renders the Agent tab showing messages with role prefixes and spacing.
///
/// Messages display:
/// - "Assistant:" prefix for regular messages
/// - "Done:" prefix with usage stats in different color for completion
/// - Visual spacing between messages
fn render_agent_tab(frame: &mut Frame, area: Rect, app: &mut App) {
    let mut lines: Vec<Line> = Vec::new();

    // Get selected story ID
    let selected_story_id = app.current_story();

    if let Some(story_id) = selected_story_id {
        if let Some(events) = app.story_events.get(story_id) {
            for (i, event) in events.iter().enumerate() {
                match event {
                    StreamEvent::Message(text) => {
                        render_message_lines(&mut lines, text);
                    }
                    StreamEvent::Done(response) => {
                        render_done_section(&mut lines, response);
                    }
                }

                // Add separator between messages (2 blank lines, except after last)
                if i < events.len() - 1 {
                    lines.push(Line::from(""));
                    lines.push(Line::from(""));
                }
            }
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "No agent messages yet",
            Style::default().fg(Color::DarkGray),
        )));
    }

    // Create paragraph to calculate actual rendered line count
    let block = Block::default().borders(Borders::ALL);
    let inner_area = block.inner(area);
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    // Get actual rendered line count (accounting for wrap)
    let total_lines = paragraph.line_count(inner_area.width);
    let visible_height = inner_area.height as usize;
    let max_scroll = total_lines.saturating_sub(visible_height);

    // Update max_scroll for auto-scroll detection in loop_scroll_down()
    app.loop_agent_max_scroll = max_scroll;

    // Calculate scroll offset: auto-scroll snaps to bottom, otherwise use user position
    let scroll_offset = if app.loop_agent_auto_scroll {
        max_scroll
    } else {
        app.loop_agent_scroll.min(max_scroll)
    };

    // Apply native scroll
    let content = paragraph.scroll((scroll_offset as u16, 0));
    frame.render_widget(content, area);
}

/// Renders a message with "Assistant:" label on its own line.
/// Content is displayed below with 2-space indentation.
/// Consecutive blank lines are compressed to a single blank line.
fn render_message_lines<'a>(lines: &mut Vec<Line<'a>>, text: &str) {
    // Add "Assistant:" label on its own line
    lines.push(Line::from(vec![
        Span::styled("Assistant:", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
    ]));

    let message_lines: Vec<&str> = text.lines().collect();

    if message_lines.is_empty() {
        return;
    }

    // Track if previous line was blank to compress consecutive blank lines
    let mut prev_was_blank = false;

    // All content lines with 2-space indentation
    for line in message_lines.iter() {
        let is_blank = line.trim().is_empty();

        // Skip consecutive blank lines
        if is_blank && prev_was_blank {
            continue;
        }

        lines.push(Line::from(vec![
            Span::raw("  "), // 2 spaces indentation
            Span::raw(line.to_string()),
        ]));

        prev_was_blank = is_blank;
    }
}

/// Renders the Done section with "Done:" label on its own line.
/// Content is displayed below with 2-space indentation.
/// Usage stats are displayed on a separate line.
/// Consecutive blank lines are compressed to a single blank line.
///
/// Display format:
/// ```text
/// Done:
///   (response content)
///   Turns: 5 | Tokens: 1234 | Cost: $0.05
/// ```
fn render_done_section<'a>(lines: &mut Vec<Line<'a>>, response: &Response) {
    // Add "Done:" label on its own line
    lines.push(Line::from(vec![
        Span::styled("Done:", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
    ]));

    let content_lines: Vec<&str> = response.content.lines().collect();

    if content_lines.is_empty() {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled("(no content)", Style::default().fg(Color::DarkGray)),
        ]));
    } else {
        // Track if previous line was blank to compress consecutive blank lines
        let mut prev_was_blank = false;

        // All content lines with 2-space indentation
        for line in content_lines.iter() {
            let is_blank = line.trim().is_empty();

            // Skip consecutive blank lines
            if is_blank && prev_was_blank {
                continue;
            }

            lines.push(Line::from(vec![
                Span::raw("  "), // 2 spaces indentation
                Span::styled(line.to_string(), Style::default().fg(Color::Green)),
            ]));

            prev_was_blank = is_blank;
        }
    }

    // Usage stats line with 2-space indentation
    let stats = format!(
        "Turns: {} | Tokens: {} | Cost: ${:.4}",
        response.turns, response.tokens, response.cost
    );
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(stats, Style::default().fg(Color::Yellow)),
    ]));
}

