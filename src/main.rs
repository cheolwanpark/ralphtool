mod agent;
mod async_cmd;
mod checkpoint;
mod ralph_loop;
mod app;
mod error;
mod event;
mod spec;
mod ui;

use std::io;
use std::panic;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use app::App;
use event::handle_events;
use ralph_loop::{DEFAULT_MAX_RETRIES, DEFAULT_COMMAND_TIMEOUT_SECS};
use ui::render;

/// Ralph Loop - Autonomous AI development orchestrator
#[derive(Parser, Debug)]
#[command(name = "ralphtool")]
#[command(about = "TUI for running the Ralph Loop with OpenSpec changes")]
struct Cli {
    /// Maximum number of retries per story when agent fails
    #[arg(long, default_value_t = DEFAULT_MAX_RETRIES)]
    max_retries: usize,

    /// Timeout in seconds for external commands (git, openspec)
    #[arg(long, default_value_t = DEFAULT_COMMAND_TIMEOUT_SECS)]
    command_timeout: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    run_tui(cli.max_retries, cli.command_timeout)
}

fn run_tui(max_retries: usize, command_timeout: u64) -> Result<()> {
    // Check if openspec CLI is available
    if let Err(e) = check_openspec_cli() {
        eprintln!("Error: {}", e);
        eprintln!("Please ensure OpenSpec CLI is installed and in your PATH.");
        std::process::exit(1);
    }

    install_panic_hook();

    let mut terminal = init_terminal()?;

    let mut app = App::new()
        .with_max_retries(max_retries)
        .with_command_timeout(command_timeout);

    // Load available changes on startup
    if let Err(e) = app.load_changes() {
        restore_terminal()?;
        eprintln!("Failed to load changes: {}", e);
        std::process::exit(1);
    }

    while app.running {
        terminal.draw(|frame| render(frame, &mut app))?;

        // Process loop events when on the loop execution screen
        if app.screen == app::Screen::LoopExecution {
            let completed = app.process_loop_events();

            // Check if we should transition
            if completed {
                // Loop completed normally - show result screen
                let result = app.build_loop_result();
                app.cleanup_loop();
                app.reset_quit_counter();
                app.show_loop_result(result);
            } else if !app.loop_state.running && app.is_loop_thread_finished() {
                // Loop was stopped by user - go back to selection
                app.cleanup_loop();
                app.reset_quit_counter();
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
