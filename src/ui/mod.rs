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
#[allow(unused_imports)]
pub use result_screen::VerificationResult;
pub use selection::render_selection;

use ratatui::prelude::*;

use crate::app::{App, Screen};

pub fn render(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::ChangeSelection => render_selection(frame, app),
        Screen::ConversionPreview => render_preview(frame, app),
        Screen::LoopExecution => render_loop_screen(frame, &app.loop_state, &app.loop_log),
        Screen::LoopResult => render_result_screen(frame, &app.loop_result, app.result_scroll_offset),
    }
}
