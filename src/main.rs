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

use app::{App, Screen};
use checkpoint::{Checkpoint, CompletionOption};
use event::handle_events;
use ralph_loop::{DEFAULT_MAX_RETRIES, DEFAULT_COMMAND_TIMEOUT_SECS};
use ui::{render, CompletionReason};

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

    // Create tokio runtime for async checkpoint operations
    let rt = tokio::runtime::Runtime::new()?;

    while app.running {
        terminal.draw(|frame| render(frame, &mut app))?;

        // Process loop events when on the loop execution screen
        if app.screen == Screen::LoopExecution {
            let completed = app.process_loop_events();

            // Check if we should transition to completion screen
            if completed {
                // Loop completed - show completion screen
                let change_name = app.selected_change_name.clone().unwrap_or_default();
                let ralph_branch = format!("ralph/{}", change_name);

                // Get original branch from current git state (before any cleanup)
                let original_branch = get_original_branch().unwrap_or_else(|| "main".to_string());

                // Determine completion reason based on max_retries_exceeded_story
                let reason = if let Some(story_id) = app.max_retries_exceeded_story.clone() {
                    CompletionReason::MaxRetries { story_id }
                } else {
                    CompletionReason::Success
                };

                app.reset_quit_counter();
                app.show_completion_screen(reason, original_branch, ralph_branch);
            } else if !app.loop_state.running && app.is_loop_thread_finished() {
                // Loop was stopped by user - show completion screen with UserStop reason
                let change_name = app.selected_change_name.clone().unwrap_or_default();
                let ralph_branch = format!("ralph/{}", change_name);
                let original_branch = get_original_branch().unwrap_or_else(|| "main".to_string());

                app.reset_quit_counter();
                app.show_completion_screen(CompletionReason::UserStop, original_branch, ralph_branch);
            }
        }

        // Handle completion screen async operations
        if app.screen == Screen::LoopCompletion && app.completion_data.in_progress {
            let option = app.completion_data.selected_completion_option();
            let change_name = app.selected_change_name.clone().unwrap_or_default();

            // Update progress message
            app.completion_data.progress_message = Some(match option {
                CompletionOption::Cleanup => format!("Returning to {}...", app.completion_data.original_branch),
                CompletionOption::Keep => format!("Staying on {}...", app.completion_data.ralph_branch),
            });

            // Redraw to show progress
            terminal.draw(|frame| render(frame, &mut app))?;

            // Run the async cleanup operation
            let result = rt.block_on(async {
                let checkpoint = Checkpoint::new(&change_name);
                checkpoint.cleanup(option).await
            });

            match result {
                Ok(()) => {
                    // Cleanup succeeded - transition to result screen
                    app.cleanup_loop();
                    app.finish_completion();
                }
                Err(e) => {
                    // Cleanup failed - log error and still transition
                    eprintln!("Cleanup error: {}", e);
                    app.cleanup_loop();
                    app.finish_completion();
                }
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

/// Gets the original branch name by checking what branch was active before ralph branch.
/// This is a heuristic - we check if we're on a ralph/ branch and try to determine
/// what branch was used before. Falls back to reading from git symbolic-ref.
fn get_original_branch() -> Option<String> {
    use std::process::Command;

    // Get current branch
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let current = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // If we're on a ralph/ branch, the original branch info is stored in the checkpoint
    // For now, return "main" as a fallback - the real original branch is stored in
    // the Checkpoint struct which we'd need to persist somehow
    if current.starts_with("ralph/") {
        // Try to find the default branch (main or master)
        let default_output = Command::new("git")
            .args(["symbolic-ref", "refs/remotes/origin/HEAD", "--short"])
            .output()
            .ok()?;

        if default_output.status.success() {
            let default_ref = String::from_utf8_lossy(&default_output.stdout).trim().to_string();
            // Extract branch name from "origin/main" -> "main"
            return default_ref.strip_prefix("origin/").map(String::from);
        }

        // Fallback to checking if main or master exists
        let main_exists = Command::new("git")
            .args(["rev-parse", "--verify", "main"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if main_exists {
            return Some("main".to_string());
        }

        return Some("master".to_string());
    }

    // Not on a ralph branch - return current
    Some(current)
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
