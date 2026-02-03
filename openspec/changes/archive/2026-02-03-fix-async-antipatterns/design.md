## Context

The orchestrator (`src/ralph_loop/orchestrator.rs`) runs in an async context using tokio. Currently, it calls external commands synchronously:

1. `spec::create_adapter()` → calls `openspec status --json` via `std::process::Command::output()`
2. `checkpoint.save/revert/drop()` → calls git commands via `std::process::Command::output()`

These blocking calls inside `async fn run()` violate tokio's cooperative scheduling model. When any of these commands hang or take too long, the worker thread is blocked, preventing:
- Other async tasks (like the event bridge) from running
- Stop flag checks from happening
- TUI becoming unresponsive to user input

## Goals / Non-Goals

**Goals:**
- Make all external command executions non-blocking in async context
- Add timeout protection to prevent indefinite hangs
- Allow users to force-quit when graceful shutdown fails

**Non-Goals:**
- Changing the overall orchestration architecture
- Adding async support to the TUI main loop (it uses std::sync::mpsc intentionally)
- Optimizing command execution performance beyond fixing the blocking issue

## Decisions

### Decision 1: Use `tokio::task::spawn_blocking()` for external commands

**Choice**: Wrap `std::process::Command` calls in `spawn_blocking()` rather than converting to `tokio::process::Command`.

**Rationale**:
- Simpler change - keeps existing command-building logic intact
- `spawn_blocking()` runs on a dedicated thread pool, freeing async workers
- `tokio::process::Command` would require more extensive refactoring and async propagation

**Alternatives considered**:
- `tokio::process::Command`: More idiomatic but requires converting entire call chains to async
- Keep sync but add threads: Adds complexity without leveraging existing tokio runtime

### Decision 2: Add command timeout with `tokio::time::timeout()`

**Choice**: Wrap command execution in `tokio::time::timeout()` with a configurable default (30 seconds).

**Rationale**:
- Prevents indefinite hangs from blocking the orchestrator forever
- 30 seconds is generous enough for slow commands but catches true hangs
- Timeout errors trigger retry logic naturally

### Decision 3: Force-quit on repeated 'q' presses

**Choice**: Track consecutive 'q' presses. First sets stop flag (graceful), third forces immediate exit.

**Rationale**:
- Maintains graceful shutdown as primary behavior
- Gives users an escape hatch when orchestrator is hung
- Simple to implement in event handler

**Alternatives considered**:
- Single 'q' always force-quits: Too aggressive, loses graceful shutdown benefits
- Separate key for force-quit (Ctrl+C): Already handled by terminal, but app may not respond

### Decision 4: Create async wrapper module

**Choice**: Create `src/async_cmd.rs` with `run_command()` async function that handles spawn_blocking + timeout.

**Rationale**:
- Single place to manage async command execution
- Easy to add logging, metrics, or additional error handling later
- Clean API: `async_cmd::run("git", &["stash", "list"]).await?`

## Risks / Trade-offs

**[Risk] spawn_blocking thread pool exhaustion** → Mitigated by timeouts preventing long-running commands from accumulating

**[Risk] Timeout too aggressive for slow systems** → Mitigated by making timeout configurable via CLI flag

**[Risk] Force-quit may leave git in inconsistent state** → Acceptable trade-off; user chose to force-quit, cleanup() attempts to restore

**[Trade-off] Added complexity** → Necessary complexity to fix real user-facing hang issues
