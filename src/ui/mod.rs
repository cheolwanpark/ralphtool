//! UI module for screen rendering and interaction.
//!
//! Contains screen-specific modules for the TUI application.

mod preview;
mod selection;

pub use preview::render_preview;
pub use selection::render_selection;

use ratatui::prelude::*;

use crate::app::{App, Screen};

pub fn render(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::ChangeSelection => render_selection(frame, app),
        Screen::ConversionPreview => render_preview(frame, app),
    }
}
