//! CLI definitions using clap derive macros.

use clap::{Parser, Subcommand};

/// Ralph - Task and story management tool.
#[derive(Parser)]
#[command(name = "ralphtool")]
#[command(about = "Ralph workflow management tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<RootCommand>,
}

/// Root-level commands.
#[derive(Subcommand)]
pub enum RootCommand {
    /// Agent CLI for machine-to-machine interaction.
    ///
    /// WARNING: These commands are for coding agents, not humans.
    /// Human users should run `ralphtool` without arguments for the TUI.
    #[command(
        about = "Agent CLI for machine-to-machine interaction",
        long_about = "WARNING: These commands are for coding agents, not humans.\n\n\
                      Human users should run `ralphtool` without arguments for the TUI.\n\n\
                      All commands output JSON and require RALPH_SESSION environment variable.\n\
                      Use the orchestrator to manage sessions properly."
    )]
    Agent {
        #[command(subcommand)]
        command: AgentCommand,
    },
}

/// Agent subcommands for machine-to-machine interaction.
///
/// WARNING: These commands are designed for use by coding agents (Claude, Amp),
/// not for direct human use. Human users should run `ralphtool` without arguments
/// to use the interactive TUI.
///
/// All commands require RALPH_SESSION environment variable to be set.
/// Session management commands (init, flush) are used by the orchestrator.
/// Other commands are used by coding agents during iteration.
#[derive(Subcommand)]
pub enum AgentCommand {
    /// Session lifecycle management (for orchestrator use)
    #[command(subcommand)]
    Session(SessionCommand),

    /// Get context for the current story
    Context,

    /// Task operations
    #[command(subcommand)]
    Task(TaskCommand),

    /// Check current progress status
    Status,

    /// Record a learning
    Learn(LearnArgs),
}

/// Session lifecycle commands (used by orchestrator).
#[derive(Subcommand)]
pub enum SessionCommand {
    /// Initialize a new session for a change
    Init(SessionInitArgs),

    /// Get the next incomplete story
    NextStory,

    /// Flush accumulated state and cleanup
    Flush,
}

/// Arguments for session init command.
#[derive(Parser)]
pub struct SessionInitArgs {
    /// Name of the change to work on
    #[arg(long)]
    pub change: String,
}

/// Task operation commands.
#[derive(Subcommand)]
pub enum TaskCommand {
    /// Mark a task as complete
    Done(TaskDoneArgs),
}

/// Arguments for task done command.
#[derive(Parser)]
pub struct TaskDoneArgs {
    /// Task ID to mark as complete (e.g., "2.1")
    pub task_id: String,
}

/// Arguments for learn command.
#[derive(Parser)]
pub struct LearnArgs {
    /// Learning description
    pub description: String,

    /// Optional task ID this learning relates to
    #[arg(long)]
    pub task: Option<String>,
}
