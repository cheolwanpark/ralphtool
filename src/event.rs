use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEvent, MouseEventKind};

use crate::app::{App, Screen};

const POLL_TIMEOUT: Duration = Duration::from_millis(250);

pub fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(POLL_TIMEOUT)? {
        match event::read()? {
            Event::Key(key_event) => {
                if key_event.kind == KeyEventKind::Press {
                    match app.screen {
                        Screen::ChangeSelection => handle_selection_events(app, key_event.code)?,
                        Screen::ConversionPreview => {
                            handle_preview_events(app, key_event.code, key_event.modifiers)
                        }
                        Screen::LoopExecution => handle_loop_events(app, key_event.code),
                        Screen::LoopCompletion => handle_completion_events(app, key_event.code),
                        Screen::LoopResult => handle_result_events(app, key_event.code),
                    }
                }
            }
            Event::Mouse(mouse_event) => match app.screen {
                Screen::ConversionPreview => handle_preview_mouse(app, mouse_event),
                Screen::LoopExecution => handle_loop_mouse(app, mouse_event),
                Screen::LoopResult => handle_result_mouse(app, mouse_event),
                _ => {}
            },
            _ => {}
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
    use crate::app::ForceQuitAction;

    match code {
        // 'q' handles force-quit mechanism with tracking of consecutive presses
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            match app.handle_quit_press() {
                ForceQuitAction::Graceful => {
                    // First press: graceful stop requested
                    // UI will show hint on subsequent presses
                }
                ForceQuitAction::Hint => {
                    // Second press: hint already shown via force_quit_hint()
                    // No additional action needed
                }
                ForceQuitAction::ForceQuit => {
                    // Third press: force-quit immediately
                    // Cleanup was already attempted in handle_quit_press()
                    std::process::exit(1);
                }
                ForceQuitAction::NavigateBack => {
                    // Loop already stopped, can navigate away
                    app.reset_quit_counter();
                    app.cleanup_loop();
                    app.back_to_selection();
                }
            }
        }
        // Story navigation
        KeyCode::Left => app.navigate_to_previous_story(),
        KeyCode::Right => app.navigate_to_next_story(),
        // Tab switching
        KeyCode::Tab => app.switch_loop_tab(),
        // Scroll within current tab
        KeyCode::Up => app.loop_scroll_up(),
        KeyCode::Down => app.loop_scroll_down(),
        _ => {}
    }
}

fn handle_completion_events(app: &mut App, code: KeyCode) {
    // Don't process input while an operation is in progress
    if app.completion_data.in_progress {
        return;
    }

    match code {
        // Select cleanup option
        KeyCode::Char('c') | KeyCode::Char('C') => {
            app.completion_data.select_cleanup();
        }
        // Select keep option
        KeyCode::Char('k') | KeyCode::Char('K') => {
            app.completion_data.select_keep();
        }
        // Toggle between options
        KeyCode::Up | KeyCode::Down => {
            app.completion_data.toggle_option();
        }
        // Confirm selection - note: actual cleanup is handled in the main loop
        // This just sets in_progress and returns the option to be processed
        KeyCode::Enter => {
            // Mark as pending confirmation - will be handled by main loop
            app.completion_data.in_progress = true;
        }
        // Cancel and go back to selection without cleanup
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
            app.cleanup_loop();
            app.back_to_selection();
        }
        _ => {}
    }
}

fn handle_result_events(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
        KeyCode::Esc => app.back_to_selection(),
        KeyCode::Tab => app.switch_result_tab(),
        KeyCode::Up => app.result_scroll_up(),
        KeyCode::Down => app.result_scroll_down(),
        _ => {}
    }
}

fn handle_preview_mouse(app: &mut App, event: MouseEvent) {
    match event.kind {
        MouseEventKind::ScrollUp => app.scroll_up(),
        MouseEventKind::ScrollDown => app.scroll_down(),
        _ => {}
    }
}

fn handle_loop_mouse(app: &mut App, event: MouseEvent) {
    match event.kind {
        MouseEventKind::ScrollUp => app.loop_scroll_up(),
        MouseEventKind::ScrollDown => app.loop_scroll_down(),
        _ => {}
    }
}

fn handle_result_mouse(app: &mut App, event: MouseEvent) {
    match event.kind {
        MouseEventKind::ScrollUp => app.result_scroll_up(),
        MouseEventKind::ScrollDown => app.result_scroll_down(),
        _ => {}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::MouseButton;

    fn create_scroll_up_event() -> MouseEvent {
        MouseEvent {
            kind: MouseEventKind::ScrollUp,
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        }
    }

    fn create_scroll_down_event() -> MouseEvent {
        MouseEvent {
            kind: MouseEventKind::ScrollDown,
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        }
    }

    fn create_click_event() -> MouseEvent {
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 10,
            modifiers: KeyModifiers::NONE,
        }
    }

    // Preview screen mouse scroll tests
    #[test]
    fn preview_mouse_scroll_up_decreases_offset() {
        let mut app = App::new();
        app.tasks_scroll_offset = 5;

        handle_preview_mouse(&mut app, create_scroll_up_event());

        assert_eq!(app.tasks_scroll_offset, 4);
    }

    #[test]
    fn preview_mouse_scroll_down_increases_offset() {
        let mut app = App::new();
        app.tasks_scroll_offset = 5;

        handle_preview_mouse(&mut app, create_scroll_down_event());

        assert_eq!(app.tasks_scroll_offset, 6);
    }

    #[test]
    fn preview_mouse_scroll_up_stops_at_zero() {
        let mut app = App::new();
        app.tasks_scroll_offset = 0;

        handle_preview_mouse(&mut app, create_scroll_up_event());

        assert_eq!(app.tasks_scroll_offset, 0);
    }

    #[test]
    fn preview_mouse_scroll_respects_active_tab() {
        use crate::app::PreviewTab;

        let mut app = App::new();
        app.active_tab = PreviewTab::Scenarios;
        app.scenarios_scroll_offset = 3;

        handle_preview_mouse(&mut app, create_scroll_down_event());

        assert_eq!(app.scenarios_scroll_offset, 4);
        assert_eq!(app.tasks_scroll_offset, 0); // Tasks tab unchanged
    }

    #[test]
    fn preview_mouse_ignores_non_scroll_events() {
        let mut app = App::new();
        app.tasks_scroll_offset = 5;

        handle_preview_mouse(&mut app, create_click_event());

        assert_eq!(app.tasks_scroll_offset, 5); // Unchanged
    }

    // Loop execution screen mouse scroll tests
    #[test]
    fn loop_mouse_scroll_up_decreases_offset() {
        let mut app = App::new();
        app.loop_info_scroll = 5;

        handle_loop_mouse(&mut app, create_scroll_up_event());

        assert_eq!(app.loop_info_scroll, 4);
    }

    #[test]
    fn loop_mouse_scroll_down_increases_offset() {
        let mut app = App::new();
        app.loop_info_scroll = 5;

        handle_loop_mouse(&mut app, create_scroll_down_event());

        assert_eq!(app.loop_info_scroll, 6);
    }

    #[test]
    fn loop_mouse_scroll_up_stops_at_zero() {
        let mut app = App::new();
        app.loop_info_scroll = 0;

        handle_loop_mouse(&mut app, create_scroll_up_event());

        assert_eq!(app.loop_info_scroll, 0);
    }

    #[test]
    fn loop_mouse_scroll_respects_active_tab() {
        use crate::app::LoopTab;

        let mut app = App::new();
        app.loop_tab = LoopTab::Agent;
        app.loop_agent_scroll = 3;

        handle_loop_mouse(&mut app, create_scroll_down_event());

        assert_eq!(app.loop_agent_scroll, 4);
        assert_eq!(app.loop_info_scroll, 0); // Info tab unchanged
    }

    #[test]
    fn loop_mouse_ignores_non_scroll_events() {
        let mut app = App::new();
        app.loop_info_scroll = 5;

        handle_loop_mouse(&mut app, create_click_event());

        assert_eq!(app.loop_info_scroll, 5); // Unchanged
    }

    // Result screen mouse scroll tests
    #[test]
    fn result_mouse_scroll_up_decreases_offset() {
        use crate::app::ResultTab;

        let mut app = App::new();
        app.result_tab = ResultTab::ChangedFiles;
        app.result_scroll_offset = 5;

        handle_result_mouse(&mut app, create_scroll_up_event());

        assert_eq!(app.result_scroll_offset, 4);
    }

    #[test]
    fn result_mouse_scroll_down_increases_offset() {
        use crate::app::ResultTab;

        let mut app = App::new();
        app.result_tab = ResultTab::ChangedFiles;
        app.result_scroll_offset = 5;

        handle_result_mouse(&mut app, create_scroll_down_event());

        assert_eq!(app.result_scroll_offset, 6);
    }

    #[test]
    fn result_mouse_scroll_up_stops_at_zero() {
        use crate::app::ResultTab;

        let mut app = App::new();
        app.result_tab = ResultTab::ChangedFiles;
        app.result_scroll_offset = 0;

        handle_result_mouse(&mut app, create_scroll_up_event());

        assert_eq!(app.result_scroll_offset, 0);
    }

    #[test]
    fn result_mouse_ignores_non_scroll_events() {
        use crate::app::ResultTab;

        let mut app = App::new();
        app.result_tab = ResultTab::ChangedFiles;
        app.result_scroll_offset = 5;

        handle_result_mouse(&mut app, create_click_event());

        assert_eq!(app.result_scroll_offset, 5); // Unchanged
    }

    #[test]
    fn result_mouse_scroll_respects_active_tab() {
        use crate::app::ResultTab;

        let mut app = App::new();
        // Default tab is Tasks
        assert_eq!(app.result_tab, ResultTab::Tasks);
        app.result_tasks_scroll = 3;

        handle_result_mouse(&mut app, create_scroll_down_event());

        assert_eq!(app.result_tasks_scroll, 4);
        assert_eq!(app.result_scroll_offset, 0); // ChangedFiles tab unchanged
    }

    #[test]
    fn result_tab_switching_via_handle_result_events() {
        use crate::app::ResultTab;

        let mut app = App::new();
        assert_eq!(app.result_tab, ResultTab::Tasks);

        handle_result_events(&mut app, KeyCode::Tab);
        assert_eq!(app.result_tab, ResultTab::ChangedFiles);

        handle_result_events(&mut app, KeyCode::Tab);
        assert_eq!(app.result_tab, ResultTab::Tasks);
    }

    #[test]
    fn result_scroll_preserves_position_when_switching_tabs() {
        use crate::app::ResultTab;

        let mut app = App::new();
        // Scroll down in Tasks tab
        app.result_tasks_scroll = 5;

        // Switch to ChangedFiles
        handle_result_events(&mut app, KeyCode::Tab);
        assert_eq!(app.result_tab, ResultTab::ChangedFiles);

        // Scroll in ChangedFiles tab
        handle_result_events(&mut app, KeyCode::Down);
        handle_result_events(&mut app, KeyCode::Down);
        assert_eq!(app.result_scroll_offset, 2);

        // Switch back to Tasks
        handle_result_events(&mut app, KeyCode::Tab);
        assert_eq!(app.result_tab, ResultTab::Tasks);

        // Tasks scroll position should be preserved
        assert_eq!(app.result_tasks_scroll, 5);
        // ChangedFiles scroll position should be preserved
        assert_eq!(app.result_scroll_offset, 2);
    }
}
