//! UI module for screen rendering and interaction.
//!
//! Contains screen-specific modules for the TUI application.

mod loop_screen;
mod preview;
mod result_screen;
mod selection;

pub use loop_screen::render_loop_screen;
pub use preview::render_preview;
pub use result_screen::{render_result_screen, LoopResult};
pub use selection::render_selection;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};

/// ASCII art logo for RalphTool (3 lines, ~18 chars wide).
const RALPH_LOGO: [&str; 3] = [
    "╦═╗┌─┐┬  ┌─┐┬ ┬",
    "╠╦╝├─┤│  ├─┤├─┤",
    "╩╚═┴ ┴┴─┘┴  ┴ ┴",
];

use crate::app::{App, Screen};

/// Context information for rendering the shared header.
pub struct HeaderContext<'a> {
    /// Screen title (e.g., "Selection", "Preview").
    pub title: &'a str,
    /// Optional context info displayed below the title.
    pub context: Option<&'a str>,
    /// Keybindings to display (format: "Key Action").
    pub keybindings: &'a [&'a str],
}

/// Renders the shared header component with three-column layout.
///
/// Layout:
/// - Left: ASCII art logo
/// - Center: Screen title and context
/// - Right: Keybindings
///
/// Returns the height of the header (5 lines including borders).
pub fn render_header(frame: &mut Frame, area: Rect, ctx: &HeaderContext) {
    // The header is 5 lines tall: 1 top border + 3 content + 1 bottom border
    let header_block = Block::default().borders(Borders::ALL);
    let inner = header_block.inner(area);
    frame.render_widget(header_block, area);

    // Calculate column widths
    // Logo is ~16 chars, we give it 18 for padding
    // Keybindings need space for the longest binding
    // Center gets the rest
    let logo_width = 18u16;
    let keybindings_width = 20u16;
    let center_width = inner.width.saturating_sub(logo_width + keybindings_width + 4); // +4 for separators

    // Build the three columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(logo_width),
            Constraint::Length(2), // Separator
            Constraint::Length(center_width),
            Constraint::Length(2), // Separator
            Constraint::Length(keybindings_width),
        ])
        .split(inner);

    // Left column: ASCII art logo
    for (i, line) in RALPH_LOGO.iter().enumerate() {
        if i < inner.height as usize {
            let logo_line = Line::from(Span::styled(
                *line,
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ));
            frame.render_widget(
                ratatui::widgets::Paragraph::new(logo_line),
                Rect::new(columns[0].x, columns[0].y + i as u16, columns[0].width, 1),
            );
        }
    }

    // Separator
    for i in 0..inner.height {
        let sep = Line::from(Span::styled("│", Style::default().fg(Color::DarkGray)));
        frame.render_widget(
            ratatui::widgets::Paragraph::new(sep),
            Rect::new(columns[1].x, columns[1].y + i, 1, 1),
        );
    }

    // Center column: Title and context
    let title_line = Line::from(Span::styled(
        ctx.title,
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ));
    frame.render_widget(
        ratatui::widgets::Paragraph::new(title_line),
        Rect::new(columns[2].x, columns[2].y, columns[2].width, 1),
    );

    if let Some(context) = ctx.context {
        if inner.height > 1 {
            let context_line = Line::from(Span::styled(
                context,
                Style::default().fg(Color::Yellow),
            ));
            frame.render_widget(
                ratatui::widgets::Paragraph::new(context_line),
                Rect::new(columns[2].x, columns[2].y + 1, columns[2].width, 1),
            );
        }
    }

    // Separator
    for i in 0..inner.height {
        let sep = Line::from(Span::styled("│", Style::default().fg(Color::DarkGray)));
        frame.render_widget(
            ratatui::widgets::Paragraph::new(sep),
            Rect::new(columns[3].x, columns[3].y + i, 1, 1),
        );
    }

    // Right column: Keybindings
    for (i, keybinding) in ctx.keybindings.iter().enumerate() {
        if i < inner.height as usize {
            let kb_line = Line::from(Span::styled(
                *keybinding,
                Style::default().fg(Color::DarkGray),
            ));
            frame.render_widget(
                ratatui::widgets::Paragraph::new(kb_line),
                Rect::new(columns[4].x, columns[4].y + i as u16, columns[4].width, 1),
            );
        }
    }
}

pub fn render(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::ChangeSelection => render_selection(frame, app),
        Screen::ConversionPreview => render_preview(frame, app),
        Screen::LoopExecution => render_loop_screen(frame, &app.loop_state, &app.loop_log),
        Screen::LoopResult => render_result_screen(frame, &app.loop_result, app.result_scroll_offset),
    }
}
