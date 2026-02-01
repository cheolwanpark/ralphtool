//! Loop screen for displaying Ralph loop progress.
//!
//! This screen shows real-time progress during loop execution:
//! - Current story being processed
//! - Task completion progress
//! - Agent output (if any)

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, List, ListItem},
};

use crate::ralph_loop::LoopState;
use super::{render_header as render_shared_header, HeaderContext};

/// Keybindings for the loop execution screen.
const LOOP_KEYBINDINGS: [&str; 1] = [
    "q Stop",
];

/// Renders the loop execution screen.
pub fn render_loop_screen(frame: &mut Frame, state: &LoopState, log: &[String]) {
    let area = frame.area();

    // Split into header, progress, and log sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Header (3 content + 2 borders)
            Constraint::Length(5), // Progress
            Constraint::Min(10),   // Log
        ])
        .split(area);

    // Header using shared component
    let status = if state.running { "Running" } else { "Stopped" };
    let story_info = if let Some(ref story) = state.current_story {
        format!("Story: {}", story)
    } else {
        "Waiting...".to_string()
    };
    let context_info = format!(
        "{} | {} [{}]",
        state.change_name, story_info, status
    );

    let header_ctx = HeaderContext {
        title: "Loop Execution",
        context: Some(&context_info),
        keybindings: &LOOP_KEYBINDINGS,
    };
    render_shared_header(frame, chunks[0], &header_ctx);

    // Progress
    render_progress(frame, chunks[1], state);

    // Log
    render_log(frame, chunks[2], log);
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
