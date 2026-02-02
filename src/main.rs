mod agent;
mod ralph_loop;
mod app;
mod error;
mod event;
mod spec;
mod ui;

use std::io;
use std::panic;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use app::App;
use event::handle_events;
use ui::render;

fn main() -> Result<()> {
    // Run TUI mode
    run_tui()
}

fn run_tui() -> Result<()> {
    // Check if openspec CLI is available
    if let Err(e) = check_openspec_cli() {
        eprintln!("Error: {}", e);
        eprintln!("Please ensure OpenSpec CLI is installed and in your PATH.");
        std::process::exit(1);
    }

    install_panic_hook();

    let mut terminal = init_terminal()?;

    let mut app = App::new();

    // Load available changes on startup
    if let Err(e) = app.load_changes() {
        restore_terminal()?;
        eprintln!("Failed to load changes: {}", e);
        std::process::exit(1);
    }

    while app.running {
        terminal.draw(|frame| render(frame, &app))?;

        // Process loop events when on the loop execution screen
        if app.screen == app::Screen::LoopExecution {
            let completed = app.process_loop_events();

            // Check if we should transition
            if completed {
                // Loop completed normally - show result screen
                let result = app.build_loop_result();
                app.cleanup_loop();
                app.show_loop_result(result);
            } else if !app.loop_state.running && app.is_loop_thread_finished() {
                // Loop was stopped by user - go back to selection
                app.cleanup_loop();
                app.back_to_selection();
            }
        }

        handle_events(&mut app)?;
    }

    restore_terminal()?;
    Ok(())
}

fn check_openspec_cli() -> Result<()> {
    use std::process::Command;
    Command::new("openspec")
        .arg("--version")
        .output()
        .map_err(|_| anyhow::anyhow!("OpenSpec CLI not found"))?;
    Ok(())
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), DisableMouseCapture, LeaveAlternateScreen)?;
    Ok(())
}

fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        original_hook(panic_info);
    }));
}
