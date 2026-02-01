//! Loop screen for displaying Ralph loop progress.
//!
//! This screen shows real-time progress during loop execution:
//! - Current story being processed
//! - Task completion progress
//! - Agent output (if any)

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
};

use crate::ralph_loop::LoopState;

/// Renders the loop execution screen.
pub fn render_loop_screen(frame: &mut Frame, state: &LoopState, log: &[String]) {
    let area = frame.area();

    // Split into header, progress, and log sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(5), // Progress
            Constraint::Min(10),   // Log
            Constraint::Length(1), // Footer
        ])
        .split(area);

    // Header
    render_header(frame, chunks[0], state);

    // Progress
    render_progress(frame, chunks[1], state);

    // Log
    render_log(frame, chunks[2], log);

    // Footer
    render_footer(frame, chunks[3]);
}

fn render_header(frame: &mut Frame, area: Rect, state: &LoopState) {
    let title = format!(" Ralph Loop: {} ", state.change_name);
    let status = if state.running {
        "Running"
    } else {
        "Stopped"
    };

    let header = Paragraph::new(format!(
        "{} [{}]",
        if let Some(ref story) = state.current_story {
            format!("Story: {}", story)
        } else {
            "Waiting...".to_string()
        },
        status
    ))
    .block(Block::default().title(title).borders(Borders::ALL));

    frame.render_widget(header, area);
}

fn render_progress(frame: &mut Frame, area: Rect, state: &LoopState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(2)])
        .margin(1)
        .split(area);

    // Story progress
    let story_ratio = if state.stories_total > 0 {
        state.stories_completed as f64 / state.stories_total as f64
    } else {
        0.0
    };

    let story_gauge = Gauge::default()
        .block(Block::default().title("Stories"))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(story_ratio)
        .label(format!(
            "{}/{}",
            state.stories_completed, state.stories_total
        ));

    frame.render_widget(story_gauge, chunks[0]);

    // Task progress
    let task_ratio = if state.tasks_total > 0 {
        state.tasks_completed as f64 / state.tasks_total as f64
    } else {
        0.0
    };

    let task_gauge = Gauge::default()
        .block(Block::default().title("Tasks (current story)"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(task_ratio)
        .label(format!(
            "{}/{}",
            state.tasks_completed, state.tasks_total
        ));

    frame.render_widget(task_gauge, chunks[1]);
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

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new(" Press 'q' to stop the loop ")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, area);
}
