mod agent;
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
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use agent::cli::{Cli, RootCommand};
use app::App;
use event::handle_events;
use ui::render;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // If agent subcommand is present, run agent mode
    if let Some(RootCommand::Agent { command }) = cli.command {
        return agent::run(command);
    }

    // Otherwise, run TUI mode
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
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        original_hook(panic_info);
    }));
}
