//! Completion screen for displaying cleanup/keep options after Ralph loop finishes.

use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use super::{centered_rect, render_header_auto, HeaderSection};
use crate::checkpoint::CompletionOption;

/// Data for rendering the completion screen.
pub struct CompletionData {
    /// Number of stories that were completed.
    pub stories_completed: usize,
    /// Total number of stories.
    pub stories_total: usize,
    /// Name of the original branch to return to (for cleanup option).
    pub original_branch: String,
    /// Name of the Ralph branch (ralph/{change}).
    pub ralph_branch: String,
    /// Currently selected option (0 = Cleanup, 1 = Keep).
    pub selected_option: usize,
    /// Whether an operation is in progress.
    pub in_progress: bool,
    /// Progress message to display during operation.
    pub progress_message: Option<String>,
    /// Reason for completion (success, max retries, user stop).
    pub completion_reason: CompletionReason,
}

/// Reason why the loop completed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionReason {
    /// All stories completed successfully.
    Success,
    /// Max retries exceeded for a story.
    MaxRetries { story_id: String },
    /// User requested stop via 'q' key.
    UserStop,
}

impl Default for CompletionData {
    fn default() -> Self {
        Self {
            stories_completed: 0,
            stories_total: 0,
            original_branch: "main".to_string(),
            ralph_branch: "ralph/change".to_string(),
            selected_option: 0,
            in_progress: false,
            progress_message: None,
            completion_reason: CompletionReason::Success,
        }
    }
}

impl CompletionData {
    /// Returns the currently selected CompletionOption.
    pub fn selected_completion_option(&self) -> CompletionOption {
        match self.selected_option {
            0 => CompletionOption::Cleanup,
            _ => CompletionOption::Keep,
        }
    }

    /// Selects the cleanup option.
    pub fn select_cleanup(&mut self) {
        self.selected_option = 0;
    }

    /// Selects the keep option.
    pub fn select_keep(&mut self) {
        self.selected_option = 1;
    }

    /// Toggles between cleanup and keep options.
    pub fn toggle_option(&mut self) {
        self.selected_option = 1 - self.selected_option;
    }
}

/// Renders the completion screen.
pub fn render_completion_screen(frame: &mut Frame, data: &CompletionData) {
    let area = frame.area();
    let centered = centered_rect(area);

    // Header
    let header = HeaderSection {
        title: "\u{25c6} Loop Complete",
        description: &completion_description(data),
        keybindings: "c Cleanup  k Keep  Enter Confirm  q Cancel",
    };

    let header_height = render_header_auto(frame, centered, &header);
    let content_y = centered.y + header_height;
    let content_height = centered.height.saturating_sub(header_height);

    // Content area
    let content_area = Rect::new(centered.x, content_y, centered.width, content_height);

    if data.in_progress {
        render_progress(frame, content_area, data);
    } else {
        render_options(frame, content_area, data);
    }
}

/// Returns a description based on the completion reason.
fn completion_description(data: &CompletionData) -> String {
    match &data.completion_reason {
        CompletionReason::Success => {
            format!(
                "All {} stories completed successfully!",
                data.stories_total
            )
        }
        CompletionReason::MaxRetries { story_id } => {
            format!(
                "Max retries exceeded for {}. {} of {} stories completed.",
                story_id, data.stories_completed, data.stories_total
            )
        }
        CompletionReason::UserStop => {
            format!(
                "Loop stopped. {} of {} stories completed.",
                data.stories_completed, data.stories_total
            )
        }
    }
}

/// Renders the options section.
fn render_options(frame: &mut Frame, area: Rect, data: &CompletionData) {
    let mut y = area.y + 1;

    // Summary section
    let summary = format!(
        "Stories: {}/{} completed",
        data.stories_completed, data.stories_total
    );
    let summary_line = Line::from(Span::styled(summary, Style::default().fg(Color::White)));
    frame.render_widget(
        Paragraph::new(summary_line),
        Rect::new(area.x, y, area.width, 1),
    );
    y += 2;

    // Branch info
    let branch_info = format!(
        "Current: {}  |  Original: {}",
        data.ralph_branch, data.original_branch
    );
    let branch_line = Line::from(Span::styled(branch_info, Style::default().fg(Color::DarkGray)));
    frame.render_widget(
        Paragraph::new(branch_line),
        Rect::new(area.x, y, area.width, 1),
    );
    y += 3;

    // Options title
    let options_title = Line::from(Span::styled(
        "Choose what to do with your changes:",
        Style::default().fg(Color::Yellow),
    ));
    frame.render_widget(
        Paragraph::new(options_title),
        Rect::new(area.x, y, area.width, 1),
    );
    y += 2;

    // Cleanup option
    render_option(
        frame,
        Rect::new(area.x, y, area.width, 3),
        "Cleanup",
        &format!(
            "Return to {} with all changes uncommitted (ready for review)",
            data.original_branch
        ),
        data.selected_option == 0,
        'c',
    );
    y += 4;

    // Keep option
    render_option(
        frame,
        Rect::new(area.x, y, area.width, 3),
        "Keep",
        &format!(
            "Stay on {} with checkpoint commits preserved",
            data.ralph_branch
        ),
        data.selected_option == 1,
        'k',
    );
}

/// Renders a single option.
fn render_option(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    description: &str,
    selected: bool,
    shortcut: char,
) {
    let (indicator, title_style, desc_style) = if selected {
        (
            "\u{25b6} ",  // Filled triangle
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            Style::default().fg(Color::White),
        )
    } else {
        (
            "  ",
            Style::default().fg(Color::White),
            Style::default().fg(Color::DarkGray),
        )
    };

    // Title line with indicator and shortcut
    let title_line = Line::from(vec![
        Span::styled(indicator, title_style),
        Span::styled(format!("[{}] ", shortcut), Style::default().fg(Color::Yellow)),
        Span::styled(title, title_style),
    ]);
    frame.render_widget(
        Paragraph::new(title_line),
        Rect::new(area.x, area.y, area.width, 1),
    );

    // Description line (indented)
    let desc_line = Line::from(Span::styled(format!("    {}", description), desc_style));
    frame.render_widget(
        Paragraph::new(desc_line),
        Rect::new(area.x, area.y + 1, area.width, 1),
    );
}

/// Renders the progress indicator during an operation.
fn render_progress(frame: &mut Frame, area: Rect, data: &CompletionData) {
    let y = area.y + area.height / 2;

    let message = data
        .progress_message
        .as_deref()
        .unwrap_or("Processing...");

    let progress_line = Line::from(vec![
        Span::styled("\u{231b} ", Style::default().fg(Color::Yellow)),  // Hourglass
        Span::styled(message, Style::default().fg(Color::White)),
    ]);

    frame.render_widget(
        Paragraph::new(progress_line),
        Rect::new(area.x, y, area.width, 1),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_completion_data() {
        let data = CompletionData::default();
        assert_eq!(data.selected_option, 0);
        assert_eq!(data.selected_completion_option(), CompletionOption::Cleanup);
    }

    #[test]
    fn select_cleanup_sets_option_to_zero() {
        let mut data = CompletionData::default();
        data.selected_option = 1;
        data.select_cleanup();
        assert_eq!(data.selected_option, 0);
    }

    #[test]
    fn select_keep_sets_option_to_one() {
        let mut data = CompletionData::default();
        data.select_keep();
        assert_eq!(data.selected_option, 1);
    }

    #[test]
    fn toggle_option_switches_between_options() {
        let mut data = CompletionData::default();
        assert_eq!(data.selected_option, 0);

        data.toggle_option();
        assert_eq!(data.selected_option, 1);

        data.toggle_option();
        assert_eq!(data.selected_option, 0);
    }

    #[test]
    fn selected_completion_option_returns_correct_variant() {
        let mut data = CompletionData::default();
        assert_eq!(data.selected_completion_option(), CompletionOption::Cleanup);

        data.select_keep();
        assert_eq!(data.selected_completion_option(), CompletionOption::Keep);
    }

    #[test]
    fn completion_description_success() {
        let data = CompletionData {
            stories_total: 5,
            completion_reason: CompletionReason::Success,
            ..Default::default()
        };
        let desc = completion_description(&data);
        assert!(desc.contains("5 stories completed successfully"));
    }

    #[test]
    fn completion_description_max_retries() {
        let data = CompletionData {
            stories_completed: 2,
            stories_total: 5,
            completion_reason: CompletionReason::MaxRetries {
                story_id: "story-3".to_string(),
            },
            ..Default::default()
        };
        let desc = completion_description(&data);
        assert!(desc.contains("Max retries exceeded"));
        assert!(desc.contains("story-3"));
        assert!(desc.contains("2 of 5"));
    }

    #[test]
    fn completion_description_user_stop() {
        let data = CompletionData {
            stories_completed: 3,
            stories_total: 5,
            completion_reason: CompletionReason::UserStop,
            ..Default::default()
        };
        let desc = completion_description(&data);
        assert!(desc.contains("Loop stopped"));
        assert!(desc.contains("3 of 5"));
    }
}
