use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::{App, Screen};

const POLL_TIMEOUT: Duration = Duration::from_millis(250);

pub fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(POLL_TIMEOUT)? {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match app.screen {
                    Screen::ChangeSelection => handle_selection_events(app, key_event.code)?,
                    Screen::ConversionPreview => handle_preview_events(app, key_event.code),
                }
            }
        }
    }
    Ok(())
}

fn handle_selection_events(app: &mut App, code: KeyCode) -> Result<()> {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Up => app.select_previous(),
        KeyCode::Down => app.select_next(),
        KeyCode::Enter => {
            if !app.available_changes.is_empty() {
                app.select_change(app.selected_index)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_preview_events(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Esc => app.back_to_selection(),
        KeyCode::Up => app.scroll_up(),
        KeyCode::Down => app.scroll_down(),
        KeyCode::PageUp => app.page_up(),
        KeyCode::PageDown => app.page_down(),
        _ => {}
    }
}
