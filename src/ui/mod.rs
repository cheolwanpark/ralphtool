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
use ratatui::widgets::Paragraph;

// Layout constants for centered container
pub const MAX_WIDTH: u16 = 100;
const MIN_HEIGHT_FOR_LOGO: u16 = 24;
const HEADER_LINES: u16 = 8;
const HEADER_LINES_COMPACT: u16 = 1;

/// ASCII art logo for RalphTool (Slim Block style, 2 lines).
const LOGO: [&str; 2] = [
    "█▀█ ▄▀█ █   █▀█ █ █",
    "█▀▄ █▀█ █▄▄ █▀▀ █▀█",
];

/// Header section data for the new vertical header layout.
///
/// This struct holds all the information needed to render the header section,
/// which includes the logo, title, description, and keybindings.
pub struct HeaderSection<'a> {
    /// Screen title with diamond icon prefix (e.g., "◆ Change Selection").
    pub title: &'a str,
    /// Brief description of the screen's purpose or context.
    pub description: &'a str,
    /// Keybindings to display (e.g., "↑↓ Navigate  Enter Select  q Quit").
    pub keybindings: &'a str,
}

/// Renders the full header section with logo (8 lines total).
///
/// Layout:
/// ```text
/// Line 1-2: Logo (█▀█ ▄▀█ █   █▀█ █ █ / █▀▄ █▀█ █▄▄ █▀▀ █▀█)
/// Line 3: Blank
/// Line 4: Title (e.g., "◆ Change Selection")
/// Line 5: Description
/// Line 6: Blank
/// Line 7: Keybindings
/// Line 8: Blank (bottom padding)
/// ```
fn render_header_section(frame: &mut Frame, area: Rect, header: &HeaderSection) {
    // Line 1-2: Logo
    for (i, line) in LOGO.iter().enumerate() {
        let logo_line = Line::from(Span::styled(
            *line,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));
        frame.render_widget(
            Paragraph::new(logo_line),
            Rect::new(area.x, area.y + i as u16, area.width, 1),
        );
    }

    // Line 3: Blank (implicit - we skip to line 4)

    // Line 4: Title
    let title_line = Line::from(Span::styled(
        header.title,
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ));
    frame.render_widget(
        Paragraph::new(title_line),
        Rect::new(area.x, area.y + 3, area.width, 1),
    );

    // Line 5: Description
    let desc_line = Line::from(Span::styled(
        header.description,
        Style::default().fg(Color::Yellow),
    ));
    frame.render_widget(
        Paragraph::new(desc_line),
        Rect::new(area.x, area.y + 4, area.width, 1),
    );

    // Line 6: Blank (implicit - we skip to line 7)

    // Line 7: Keybindings
    let kb_line = Line::from(Span::styled(
        header.keybindings,
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(
        Paragraph::new(kb_line),
        Rect::new(area.x, area.y + 6, area.width, 1),
    );

    // Line 8: Blank (bottom padding, implicit)
}

/// Renders the compact header (single line) for small terminals.
///
/// Format: "◆ Title │ keybindings"
/// Example: "◆ Selection │ ↑↓ Navigate  Enter Select  q Quit"
fn render_header_compact(frame: &mut Frame, area: Rect, header: &HeaderSection) {
    // Extract short title (remove "◆ " prefix if present, or use as-is)
    let title = header.title;

    // Build the compact line: "Title │ keybindings"
    let compact_line = Line::from(vec![
        Span::styled(
            title,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" │ ", Style::default().fg(Color::DarkGray)),
        Span::styled(header.keybindings, Style::default().fg(Color::DarkGray)),
    ]);

    frame.render_widget(Paragraph::new(compact_line), area);
}

/// Chooses and renders the appropriate header based on terminal height.
///
/// - If terminal height >= MIN_HEIGHT_FOR_LOGO (24): renders full 8-line header with logo
/// - If terminal height < MIN_HEIGHT_FOR_LOGO: renders compact single-line header
///
/// Returns the height used by the header (HEADER_LINES or HEADER_LINES_COMPACT).
pub(crate) fn render_header_auto(frame: &mut Frame, area: Rect, header: &HeaderSection) -> u16 {
    if area.height >= MIN_HEIGHT_FOR_LOGO {
        // Use full header with logo
        let header_area = Rect::new(area.x, area.y, area.width, HEADER_LINES);
        render_header_section(frame, header_area, header);
        HEADER_LINES
    } else {
        // Use compact header
        let header_area = Rect::new(area.x, area.y, area.width, HEADER_LINES_COMPACT);
        render_header_compact(frame, header_area, header);
        HEADER_LINES_COMPACT
    }
}


/// Calculates a centered rectangle within the given area, constrained by max dimensions.
///
/// Returns a `Rect` that is:
/// - At most `max_width` wide, centered horizontally
/// - At most `max_height` tall, centered vertically
/// - If the area is smaller than the max dimensions, uses the full area dimension
pub(crate) fn centered_rect(area: Rect, max_width: u16, max_height: u16) -> Rect {
    let width = area.width.min(max_width);
    let height = area.height.min(max_height);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}

use crate::app::{App, Screen};

pub fn render(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::ChangeSelection => render_selection(frame, app),
        Screen::ConversionPreview => render_preview(frame, app),
        Screen::LoopExecution => render_loop_screen(frame, &app.loop_state, &app.loop_log),
        Screen::LoopResult => render_result_screen(frame, &app.loop_result, app.result_scroll_offset),
    }
}
