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
pub fn render_loop_screen(frame: &mut Frame, app: &App) {
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
///   ☐ 5.1 Create render_progress_bar() function
///   ☑ 5.2 Create render_story_indicator() function
///   ...
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

                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(checkbox, checkbox_style),
                    Span::raw(" "),
                    Span::styled(&task.id, Style::default().fg(Color::DarkGray)),
                    Span::raw(" "),
                    Span::styled(&task.description, text_style),
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

    // Apply scroll offset
    let visible_lines: Vec<Line> = lines
        .into_iter()
        .skip(app.loop_info_scroll)
        .collect();

    let content = Paragraph::new(visible_lines)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false });
    frame.render_widget(content, area);
}

/// Renders the Agent tab showing messages with role prefixes and spacing.
///
/// Messages display:
/// - "Assistant:" prefix for regular messages
/// - "Done:" prefix with usage stats in different color for completion
/// - Visual spacing between messages
fn render_agent_tab(frame: &mut Frame, area: Rect, app: &App) {
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

                // Add separator between messages (except after last)
                if i < events.len() - 1 {
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

    // Calculate total content height for scroll snap
    let total_height = lines.len();
    let visible_height = area.height.saturating_sub(2) as usize; // Account for borders

    // Apply scroll offset with snap logic
    let scroll_offset = calculate_scroll_offset(
        app.loop_agent_scroll,
        total_height,
        visible_height,
    );

    let visible_lines: Vec<Line> = lines
        .into_iter()
        .skip(scroll_offset)
        .collect();

    let content = Paragraph::new(visible_lines)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false });
    frame.render_widget(content, area);
}

/// Renders a message with "Assistant:" prefix.
/// Handles multi-line messages by prefixing only the first line.
fn render_message_lines<'a>(lines: &mut Vec<Line<'a>>, text: &str) {
    let message_lines: Vec<&str> = text.lines().collect();

    if message_lines.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("Assistant: ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
        ]));
        return;
    }

    // First line with prefix
    lines.push(Line::from(vec![
        Span::styled("Assistant: ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
        Span::raw(message_lines[0].to_string()),
    ]));

    // Subsequent lines without prefix (indented to align)
    for line in message_lines.iter().skip(1) {
        lines.push(Line::from(vec![
            Span::raw("           "), // 11 spaces to align with "Assistant: "
            Span::raw(line.to_string()),
        ]));
    }
}

/// Renders the Done section with usage stats in a distinct color.
///
/// Display format:
/// ```text
/// Done: [response content]
///       Turns: 5 | Tokens: 1234 | Cost: $0.05
/// ```
fn render_done_section<'a>(lines: &mut Vec<Line<'a>>, response: &Response) {
    // Done header in green/cyan
    let content_lines: Vec<&str> = response.content.lines().collect();

    if content_lines.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("Done: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("(no content)", Style::default().fg(Color::DarkGray)),
        ]));
    } else {
        // First line with prefix
        lines.push(Line::from(vec![
            Span::styled("Done: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(content_lines[0].to_string(), Style::default().fg(Color::Green)),
        ]));

        // Subsequent content lines
        for line in content_lines.iter().skip(1) {
            lines.push(Line::from(vec![
                Span::raw("      "), // 6 spaces to align with "Done: "
                Span::styled(line.to_string(), Style::default().fg(Color::Green)),
            ]));
        }
    }

    // Usage stats line in a muted color
    let stats = format!(
        "Turns: {} | Tokens: {} | Cost: ${:.4}",
        response.turns, response.tokens, response.cost
    );
    lines.push(Line::from(vec![
        Span::raw("      "),
        Span::styled(stats, Style::default().fg(Color::Yellow)),
    ]));
}

/// Calculates the scroll offset with snap-to-bottom behavior.
///
/// Scroll snap logic:
/// - When at or near bottom, keep scrolled to show newest content
/// - When user scrolls up, respect their position
/// - Returns the actual scroll offset to use
fn calculate_scroll_offset(
    user_scroll: usize,
    total_height: usize,
    visible_height: usize,
) -> usize {
    if total_height <= visible_height {
        // Content fits, no scrolling needed
        return 0;
    }

    let max_scroll = total_height.saturating_sub(visible_height);

    // If user scroll is at or beyond max, snap to bottom
    if user_scroll >= max_scroll {
        max_scroll
    } else {
        // Respect user's scroll position
        user_scroll.min(max_scroll)
    }
}

/// Checks if the current scroll position is at the bottom (for auto-scroll).
///
/// Returns true if scrolled to bottom, meaning auto-scroll should be active.
#[allow(dead_code)]
pub fn is_at_bottom(scroll_offset: usize, total_height: usize, visible_height: usize) -> bool {
    if total_height <= visible_height {
        true
    } else {
        let max_scroll = total_height.saturating_sub(visible_height);
        scroll_offset >= max_scroll
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_scroll_offset_content_fits() {
        // Content fits in view, no scroll needed
        assert_eq!(calculate_scroll_offset(0, 10, 20), 0);
        assert_eq!(calculate_scroll_offset(5, 10, 20), 0);
    }

    #[test]
    fn test_calculate_scroll_offset_snap_to_bottom() {
        // User at max scroll, snap to bottom
        let total = 30;
        let visible = 10;
        let max_scroll = 20;

        assert_eq!(calculate_scroll_offset(max_scroll, total, visible), max_scroll);
        assert_eq!(calculate_scroll_offset(max_scroll + 5, total, visible), max_scroll);
    }

    #[test]
    fn test_calculate_scroll_offset_respects_user_position() {
        // User scrolled up, respect their position
        let total = 30;
        let visible = 10;

        assert_eq!(calculate_scroll_offset(5, total, visible), 5);
        assert_eq!(calculate_scroll_offset(15, total, visible), 15);
    }

    #[test]
    fn test_is_at_bottom_content_fits() {
        assert!(is_at_bottom(0, 10, 20));
    }

    #[test]
    fn test_is_at_bottom_scrolled_to_end() {
        assert!(is_at_bottom(20, 30, 10));
    }

    #[test]
    fn test_is_at_bottom_not_at_end() {
        assert!(!is_at_bottom(10, 30, 10));
    }
}
