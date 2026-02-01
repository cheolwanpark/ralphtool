## Why

The current agent architecture requires complex session management (session files, environment variables, special CLI commands) that doesn't work with Claude Code's Bash tool because environment variables aren't inherited by subprocesses. This causes agent failures and unnecessary complexity. Removing the session layer and letting agents work directly with files is simpler and more robust.

## What Changes

- **BREAKING**: Remove entire session module (`src/session/`) including CLI commands (`agent session init/flush/next-story`, `agent task done`, `agent status`, `agent context`, `agent learn`)
- **BREAKING**: Remove lock file mechanism (`.ralph/locks/`)
- Simplify orchestrator to just spawn agent with a prompt containing change location
- Agent reads OpenSpec files directly (`proposal.md`, `design.md`, `tasks.md`, `specs/`)
- Agent marks tasks complete by directly editing `tasks.md` (`[ ]` → `[x]`)
- TUI watches file changes or streams agent output for progress display

## Capabilities

### New Capabilities
- `agent-prompt`: Defines the prompt template that tells agent how to work on a change autonomously

### Modified Capabilities
- `ralph-loop`: Remove session-based orchestration, simplify to single agent spawn with file watching
- `ui-rendering`: Update loop execution screen to work without session state
- `agent-cli`: **REMOVED** - Entire agent CLI subcommand tree deleted
- `agent-session`: **REMOVED** - Session management no longer needed
- `agent-context`: **REMOVED** - Agent reads files directly
- `agent-tasks`: **REMOVED** - Agent edits tasks.md directly
- `agent-progress`: **REMOVED** - No session-based progress tracking
- `scoped-session`: **REMOVED** - ScopedSession wrapper deleted

## Impact

- **Code removed**: `src/session/` module (~600 lines), agent CLI commands
- **Code simplified**: `src/ralph_loop/orchestrator.rs`, `src/app.rs`
- **Files removed**: `.ralph/locks/`, `.ralph/sessions/`, `/tmp/ralph/sessions/`
- **Dependencies**: No new dependencies; may remove `fs2` (file locking) and `uuid` if unused elsewhere
