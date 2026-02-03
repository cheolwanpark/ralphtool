## 1. Async Command Module

- [x] 1.1 Create `src/async_cmd.rs` module with `run()` function that wraps `std::process::Command` in `tokio::task::spawn_blocking()`
- [x] 1.2 Add `run_with_timeout()` function that wraps execution in `tokio::time::timeout()` with configurable duration
- [x] 1.3 Add default timeout constant (30 seconds) and apply it in `run()`
- [x] 1.4 Add proper error types for timeout, command not found, and execution failure
- [x] 1.5 Add unit tests for async command execution

## 2. Checkpoint Async Migration

- [x] 2.1 Update `Checkpoint` struct to use async methods (`save`, `revert`, `drop`, `cleanup`, `find_stash`)
- [x] 2.2 Replace all `Command::new().output()` calls with `async_cmd::run()` in checkpoint module
- [x] 2.3 Update orchestrator to await checkpoint operations
- [x] 2.4 Update checkpoint tests to use `#[tokio::test]`

## 3. OpenSpec Adapter Async Migration

- [x] 3.1 Convert `run_openspec_command()` to async using `async_cmd::run()`
- [x] 3.2 Update `OpenSpecAdapter::new()` to be async
- [x] 3.3 Update `OpenSpecAdapter::get_status()` and `list_changes()` to be async
- [x] 3.4 Update orchestrator's `spec::create_adapter()` call to await
- [x] 3.5 Update adapter tests to use `#[tokio::test]`

## 4. Force-Quit Mechanism

- [x] 4.1 Add `quit_press_count` and `last_quit_time` fields to App state
- [x] 4.2 Update `handle_loop_events()` to track consecutive 'q' presses within 3 seconds
- [x] 4.3 Implement force-quit on third 'q' press: call `std::process::exit(1)` after attempting cleanup
- [x] 4.4 Display force-quit hint message after first 'q' press when loop is running

## 5. CLI and Configuration

- [x] 5.1 Add `--command-timeout` CLI flag with default of 30 seconds
- [x] 5.2 Pass timeout configuration through App to orchestrator
- [x] 5.3 Update orchestrator to use configured timeout for all async commands

## 6. Integration Testing

- [x] 6.1 Add integration test: orchestrator handles command timeout gracefully
- [x] 6.2 Add integration test: force-quit works when orchestrator is hung
- [x] 6.3 Verify TUI remains responsive during long-running commands
