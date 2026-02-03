//! UI module for screen rendering and interaction.
//!
//! Contains screen-specific modules for the TUI application.

mod completion_screen;
mod loop_screen;
mod preview;
mod result_screen;
mod selection;

pub use completion_screen::{render_completion_screen, CompletionData, CompletionReason};
pub use loop_screen::render_loop_screen;
pub use preview::render_preview;
pub use result_screen::{render_result_screen, LoopResult};
pub use selection::render_selection;

use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

// Layout constants for centered container
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


/// Calculates responsive content width as 85% of terminal width, clamped to [60, 140].
///
/// This provides a fluid layout that adapts to terminal size while maintaining
/// readability bounds:
/// - Minimum 60 columns ensures content remains readable
/// - Maximum 140 columns prevents overly wide text
/// - 85% provides comfortable margins without wasting space
pub(crate) fn responsive_width(terminal_width: u16) -> u16 {
    let target = (terminal_width as f32 * 0.85) as u16;
    target.clamp(60, 140)
}

/// Calculates responsive content height as 90% of terminal height, clamped to [20, 50].
///
/// This provides a fluid vertical layout that adapts to terminal size while maintaining
/// usability bounds:
/// - Minimum 20 rows ensures header + content area remain usable
/// - Maximum 50 rows prevents excessive vertical spread on tall terminals
/// - 90% (higher than width's 85%) because vertical space is more constrained
pub(crate) fn responsive_height(terminal_height: u16) -> u16 {
    let target = (terminal_height as f32 * 0.90) as u16;
    target.clamp(20, 50)
}

/// Calculates a centered rectangle within the given area using responsive width and height.
///
/// Returns a `Rect` that is:
/// - Responsive width (85% of terminal, clamped to [60, 140]), centered horizontally
/// - Responsive height (90% of terminal, clamped to [20, 50]), centered vertically
pub(crate) fn centered_rect(area: Rect) -> Rect {
    let width = responsive_width(area.width);
    let height = responsive_height(area.height);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}

use crate::app::{App, Screen};

pub fn render(frame: &mut Frame, app: &mut App) {
    match app.screen {
        Screen::ChangeSelection => render_selection(frame, app),
        Screen::ConversionPreview => render_preview(frame, app),
        Screen::LoopExecution => render_loop_screen(frame, app),
        Screen::LoopCompletion => render_completion_screen(frame, &app.completion_data),
        Screen::LoopResult => render_result_screen(
            frame,
            &app.loop_result,
            app.result_tab,
            app.result_tasks_scroll,
            app.result_scroll_offset,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== responsive_width tests ==========

    #[test]
    fn test_responsive_width_standard_terminal() {
        // 85% of 100 = 85
        assert_eq!(responsive_width(100), 85);
    }

    #[test]
    fn test_responsive_width_minimum_clamp() {
        // 85% of 70 = 59.5 -> 59, but clamped to 60
        assert_eq!(responsive_width(70), 60);
    }

    #[test]
    fn test_responsive_width_maximum_clamp() {
        // 85% of 200 = 170, but clamped to 140
        assert_eq!(responsive_width(200), 140);
    }

    #[test]
    fn test_responsive_width_wide_terminal() {
        // 85% of 150 = 127.5 -> 127
        assert_eq!(responsive_width(150), 127);
    }

    // ========== responsive_height tests ==========

    #[test]
    fn test_responsive_height_standard_terminal() {
        // 90% of 40 = 36
        assert_eq!(responsive_height(40), 36);
    }

    #[test]
    fn test_responsive_height_tall_terminal_clamped() {
        // 90% of 80 = 72, but clamped to 50
        assert_eq!(responsive_height(80), 50);
    }

    #[test]
    fn test_responsive_height_short_terminal_clamped() {
        // 90% of 18 = 16.2 -> 16, but clamped to 20
        assert_eq!(responsive_height(18), 20);
    }

    #[test]
    fn test_responsive_height_minimum_bound() {
        // 90% of 20 = 18, but clamped to 20
        assert_eq!(responsive_height(20), 20);
    }

    #[test]
    fn test_responsive_height_very_short_terminal() {
        // 90% of 15 = 13.5 -> 13, but clamped to 20
        assert_eq!(responsive_height(15), 20);
    }

    // ========== centered_rect tests ==========

    #[test]
    fn test_centered_rect_80x24() {
        // Terminal 80x24 (standard small terminal)
        let area = Rect::new(0, 0, 80, 24);
        let result = centered_rect(area);

        // Width: 85% of 80 = 68
        assert_eq!(result.width, 68);
        // Height: 90% of 24 = 21.6 -> 21
        assert_eq!(result.height, 21);
        // X centering: (80 - 68) / 2 = 6
        assert_eq!(result.x, 6);
        // Y centering: (24 - 21) / 2 = 1
        assert_eq!(result.y, 1);
    }

    #[test]
    fn test_centered_rect_120x40() {
        // Terminal 120x40 (standard medium terminal)
        let area = Rect::new(0, 0, 120, 40);
        let result = centered_rect(area);

        // Width: 85% of 120 = 102
        assert_eq!(result.width, 102);
        // Height: 90% of 40 = 36
        assert_eq!(result.height, 36);
        // X centering: (120 - 102) / 2 = 9
        assert_eq!(result.x, 9);
        // Y centering: (40 - 36) / 2 = 2
        assert_eq!(result.y, 2);
    }

    #[test]
    fn test_centered_rect_200x80() {
        // Terminal 200x80 (large terminal)
        let area = Rect::new(0, 0, 200, 80);
        let result = centered_rect(area);

        // Width: 85% of 200 = 170, clamped to 140
        assert_eq!(result.width, 140);
        // Height: 90% of 80 = 72, clamped to 50
        assert_eq!(result.height, 50);
        // X centering: (200 - 140) / 2 = 30
        assert_eq!(result.x, 30);
        // Y centering: (80 - 50) / 2 = 15
        assert_eq!(result.y, 15);
    }

    #[test]
    fn test_centered_rect_preserves_origin_offset() {
        // Test that centered_rect works correctly with non-zero origin
        let area = Rect::new(10, 5, 100, 40);
        let result = centered_rect(area);

        // Width: 85% of 100 = 85
        assert_eq!(result.width, 85);
        // Height: 90% of 40 = 36
        assert_eq!(result.height, 36);
        // X centering: 10 + (100 - 85) / 2 = 10 + 7 = 17
        assert_eq!(result.x, 17);
        // Y centering: 5 + (40 - 36) / 2 = 5 + 2 = 7
        assert_eq!(result.y, 7);
    }

    // ========== Header auto-selection tests ==========
    // These tests verify the header decision logic based on centered area height

    #[test]
    fn test_header_selection_full_header_for_tall_area() {
        // Area with height >= 24 should use full header
        let area = Rect::new(0, 0, 100, 36);
        // MIN_HEIGHT_FOR_LOGO is 24
        assert!(area.height >= MIN_HEIGHT_FOR_LOGO);
    }

    #[test]
    fn test_header_selection_compact_for_short_area() {
        // Area with height < 24 should use compact header
        let area = Rect::new(0, 0, 100, 21);
        // MIN_HEIGHT_FOR_LOGO is 24
        assert!(area.height < MIN_HEIGHT_FOR_LOGO);
    }

    #[test]
    fn test_header_selection_80x24_terminal() {
        // For 80x24 terminal, centered area height is 21, so compact header is used
        let area = Rect::new(0, 0, 80, 24);
        let centered = centered_rect(area);
        // 21 < 24, so compact header
        assert!(centered.height < MIN_HEIGHT_FOR_LOGO);
        assert_eq!(centered.height, 21);
    }

    #[test]
    fn test_header_selection_120x40_terminal() {
        // For 120x40 terminal, centered area height is 36, so full header is used
        let area = Rect::new(0, 0, 120, 40);
        let centered = centered_rect(area);
        // 36 >= 24, so full header
        assert!(centered.height >= MIN_HEIGHT_FOR_LOGO);
        assert_eq!(centered.height, 36);
    }

    #[test]
    fn test_header_selection_200x80_terminal() {
        // For 200x80 terminal, centered area height is 50 (capped), so full header is used
        let area = Rect::new(0, 0, 200, 80);
        let centered = centered_rect(area);
        // 50 >= 24, so full header
        assert!(centered.height >= MIN_HEIGHT_FOR_LOGO);
        assert_eq!(centered.height, 50);
    }

    #[test]
    fn test_header_selection_boundary_at_27_rows() {
        // At 27 rows terminal height: 90% = 24.3 -> 24, which equals MIN_HEIGHT_FOR_LOGO
        let area = Rect::new(0, 0, 100, 27);
        let centered = centered_rect(area);
        // 24 >= 24, so full header should be used
        assert!(centered.height >= MIN_HEIGHT_FOR_LOGO);
        assert_eq!(centered.height, 24);
    }

    #[test]
    fn test_header_selection_boundary_at_26_rows() {
        // At 26 rows terminal height: 90% = 23.4 -> 23, which is < MIN_HEIGHT_FOR_LOGO
        let area = Rect::new(0, 0, 100, 26);
        let centered = centered_rect(area);
        // 23 < 24, so compact header should be used
        assert!(centered.height < MIN_HEIGHT_FOR_LOGO);
        assert_eq!(centered.height, 23);
    }
}
