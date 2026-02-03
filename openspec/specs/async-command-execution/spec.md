## Purpose

Provides async-safe command execution utilities that wrap blocking `std::process::Command` calls to work correctly within tokio async contexts.

## Requirements

### Requirement: Async command execution

The system SHALL provide an async function to execute external commands without blocking tokio worker threads.

#### Scenario: Execute command asynchronously

- **WHEN** `async_cmd::run(program, args)` is called from an async context
- **THEN** the command SHALL be executed via `tokio::task::spawn_blocking()`
- **AND** the async task SHALL yield properly while the command runs
- **AND** the function SHALL return `Result<Output>`

#### Scenario: Command with timeout

- **WHEN** `async_cmd::run_with_timeout(program, args, duration)` is called
- **THEN** the command SHALL be wrapped in `tokio::time::timeout()`
- **AND** if the command exceeds the timeout, a timeout error SHALL be returned

#### Scenario: Default timeout

- **WHEN** `async_cmd::run(program, args)` is called without explicit timeout
- **THEN** a default timeout of 30 seconds SHALL be applied

### Requirement: Command output parsing

The system SHALL provide helpers for common command output operations.

#### Scenario: Successful command output

- **WHEN** the command exits with status 0
- **THEN** the function SHALL return `Ok(Output)` with stdout and stderr captured

#### Scenario: Failed command output

- **WHEN** the command exits with non-zero status
- **THEN** the function SHALL return an error containing the stderr output

#### Scenario: Command not found

- **WHEN** the specified program is not found in PATH
- **THEN** the function SHALL return a descriptive error indicating the program is not available
