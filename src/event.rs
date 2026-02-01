use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::app::{App, Screen};

const POLL_TIMEOUT: Duration = Duration::from_millis(250);

pub fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(POLL_TIMEOUT)? {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match app.screen {
                    Screen::ChangeSelection => handle_selection_events(app, key_event.code)?,
                    Screen::ConversionPreview => {
                        handle_preview_events(app, key_event.code, key_event.modifiers)
                    }
                    Screen::LoopExecution => handle_loop_events(app, key_event.code),
                    Screen::LoopResult => handle_result_events(app, key_event.code),
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

fn handle_preview_events(app: &mut App, code: KeyCode, modifiers: KeyModifiers) {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Char('r') | KeyCode::Char('R') => app.start_loop(),
        KeyCode::Esc => app.back_to_selection(),
        KeyCode::Up => app.scroll_up(),
        KeyCode::Down => app.scroll_down(),
        KeyCode::PageUp => app.page_up(),
        KeyCode::PageDown => app.page_down(),
        KeyCode::Tab if modifiers.contains(KeyModifiers::SHIFT) => app.switch_to_previous_tab(),
        KeyCode::Tab => app.switch_to_next_tab(),
        KeyCode::BackTab => app.switch_to_previous_tab(),
        _ => {}
    }
}

fn handle_loop_events(app: &mut App, code: KeyCode) {
    match code {
        // 'q' sets stop flag - the loop will stop after current agent completes
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            if app.loop_state.running {
                // Request stop and wait for graceful shutdown
                app.request_loop_stop();
            } else {
                // Loop already stopped, can navigate away
                app.cleanup_loop();
                app.back_to_selection();
            }
        }
        _ => {}
    }
}

fn handle_result_events(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Esc => app.back_to_selection(),
        KeyCode::Up => app.result_scroll_up(),
        KeyCode::Down => app.result_scroll_down(),
        _ => {}
    }
}
