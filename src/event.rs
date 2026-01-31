use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::App;

const POLL_TIMEOUT: Duration = Duration::from_millis(250);

pub fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(POLL_TIMEOUT)? {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
