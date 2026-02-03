## MODIFIED Requirements

### Requirement: Loop control

The system SHALL allow the user to stop the loop, including a force-quit mechanism when graceful shutdown fails.

#### Scenario: User stops loop

- **WHEN** user presses 'q' during loop execution
- **THEN** the system signals the agent to stop
- **AND** preserves any completed work

#### Scenario: Force-quit on repeated q presses

- **WHEN** user presses 'q' three times within 3 seconds
- **THEN** the system SHALL force-quit immediately
- **AND** cleanup SHALL be attempted but not block exit
- **AND** a warning message SHALL be displayed indicating force-quit was triggered

#### Scenario: Graceful stop timeout

- **WHEN** graceful stop is requested but the orchestrator does not respond within 5 seconds
- **THEN** the TUI SHALL display a message suggesting force-quit (press q twice more)

## ADDED Requirements

### Requirement: Async-safe orchestration

The orchestrator SHALL execute all blocking operations in an async-safe manner.

#### Scenario: Adapter refresh is non-blocking

- **WHEN** the orchestrator refreshes the adapter between stories
- **THEN** the adapter creation SHALL use async command execution
- **AND** other async tasks (like event forwarding) SHALL continue to make progress

#### Scenario: Checkpoint operations are non-blocking

- **WHEN** the orchestrator saves, reverts, or drops checkpoints
- **THEN** the checkpoint operations SHALL use async command execution
- **AND** the stop flag SHALL remain checkable during these operations

### Requirement: Operation timeouts

The orchestrator SHALL apply timeouts to external operations to prevent indefinite hangs.

#### Scenario: Default command timeout

- **WHEN** external commands (git, openspec) are executed
- **THEN** a default timeout of 30 seconds SHALL be applied

#### Scenario: Timeout triggers retry

- **WHEN** a command times out during story execution
- **THEN** the timeout SHALL be treated as a failure
- **AND** the retry mechanism SHALL be triggered if retries are available

#### Scenario: Configurable timeout

- **WHEN** user specifies `--command-timeout <seconds>` CLI flag
- **THEN** the specified timeout SHALL be used for all external commands
