## Why

The orchestrator runs blocking operations (`std::process::Command::output()`) directly inside async functions. This is a well-documented tokio antipattern that blocks worker threads, preventing other async tasks from making progress. When external commands hang or take too long, the entire TUI becomes unresponsive - users cannot even press 'q' to quit.

## What Changes

- Wrap all blocking command executions in `tokio::task::spawn_blocking()` or replace with `tokio::process::Command`
- Add timeout mechanisms to external command calls to prevent indefinite hangs
- Add a force-quit mechanism when graceful shutdown fails

## Capabilities

### New Capabilities

- `async-command-execution`: Safe async wrappers for executing external commands (openspec CLI, git) that don't block tokio worker threads

### Modified Capabilities

- `checkpoint`: Update git stash operations to use async-safe command execution
- `openspec-adapter`: Update openspec CLI calls to use async-safe command execution
- `ralph-loop`: Add timeout handling and force-quit mechanism for hung operations

## Impact

- `src/checkpoint/mod.rs`: All git command calls need async wrappers
- `src/spec/openspec.rs`: `run_openspec_command()` needs async wrapper
- `src/ralph_loop/orchestrator.rs`: Add timeout and cancellation support
- `src/event.rs`: Add force-quit after repeated 'q' presses
- `Cargo.toml`: May need `tokio` feature flags for `process` and `time`
