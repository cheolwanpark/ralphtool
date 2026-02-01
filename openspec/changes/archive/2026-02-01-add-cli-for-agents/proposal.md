# Proposal: Add CLI for Agents

## Why

Coding agents (Claude, Amp) in the Ralph Loop currently interact with state files (tasks.md, design.md) through raw file manipulation. This is unreliable—agents may write incorrectly, skip updates, or corrupt formats. We need a structured CLI interface that provides validated, logged, and scoped access to Ralph state, with session-level context management so each Ralph Loop run is isolated.

## What Changes

- Add `ralphtool agent` subcommand for coding agent operations (not for human use)
- Keep `ralphtool` (no subcommand) as TUI for human users
- Provide session-scoped context management (each Ralph Loop gets isolated state)
- Support story-level context (each iteration works on one story with all its tasks)
- Add task management commands (context, complete, status)
- Add progress tracking commands (learnings)
- All output in JSON format for agent consumption
- Require `RALPH_SESSION` environment variable (fail without it)

## Capabilities

### New Capabilities

- `agent-session`: Session lifecycle management for orchestrator. Initialize session for a change, get next story for iteration, flush pending writes. Sessions stored in OS temp directory.

- `agent-context`: Context retrieval for current story. Returns story details, all tasks in story, proposal, design, scenarios, learnings from previous iterations, and verification commands.

- `agent-tasks`: Task completion within current story. Mark individual tasks complete, get remaining tasks, detect story completion.

- `agent-progress`: Progress tracking scoped to session. Record learnings during iteration, accumulated and flushed to design.md at session end.

### Modified Capabilities

(none - this is additive)

## Impact

- **Code**: New `agent` module with subcommands, session state management
- **Binary**: Single `ralphtool` binary with TUI (default) and agent modes
- **Dependencies**: clap needs to be added for CLI parsing
- **Existing**: No changes to TUI or OpenSpecAdapter (agent mode uses same adapter)
- **Agent prompts**: Will reference `ralphtool agent` commands instead of file edits
