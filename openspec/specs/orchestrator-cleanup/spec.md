## Purpose

Handles the cleanup lifecycle when the Ralph Loop completes. The orchestrator owns the complete cleanup flow, waiting for user choice (Cleanup/Keep) and executing the appropriate branch operations before signaling completion to the TUI.

## Requirements

### Requirement: Orchestrator updates completed_stories after final story

The orchestrator SHALL update `state.completed_stories` to equal `state.total_stories` when all stories have been completed successfully.

#### Scenario: All stories complete successfully
- **WHEN** the orchestrator finishes processing all stories (no more incomplete stories found)
- **THEN** `state.completed_stories` SHALL equal `state.total_stories`

#### Scenario: Story count displayed on completion screen
- **WHEN** the completion screen displays after all 3 stories complete successfully
- **THEN** the summary SHALL show "Stories: 3/3 completed"

### Requirement: Orchestrator sends AwaitingUserChoice event

The orchestrator SHALL send an `AwaitingUserChoice` event when all story processing is complete (whether by success, max retries, or user stop) and wait for a response before proceeding.

#### Scenario: Successful completion triggers AwaitingUserChoice
- **WHEN** all stories complete successfully
- **THEN** orchestrator SHALL send `AwaitingUserChoice` event with a oneshot sender
- **AND** orchestrator SHALL wait for user choice response

#### Scenario: Max retries exceeded triggers AwaitingUserChoice
- **WHEN** max retries is exceeded for a story
- **THEN** orchestrator SHALL send `AwaitingUserChoice` event after emitting `MaxRetriesExceeded`
- **AND** orchestrator SHALL wait for user choice response

#### Scenario: User stop triggers AwaitingUserChoice
- **WHEN** user requests stop via stop flag
- **THEN** orchestrator SHALL send `AwaitingUserChoice` event
- **AND** orchestrator SHALL wait for user choice response

### Requirement: Orchestrator executes cleanup based on user choice

The orchestrator SHALL execute `checkpoint.cleanup(option)` with the user's choice before sending the `Complete` event.

#### Scenario: User chooses Cleanup
- **WHEN** user selects Cleanup option
- **AND** orchestrator receives the choice via oneshot channel
- **THEN** orchestrator SHALL call `checkpoint.cleanup(CompletionOption::Cleanup)`
- **AND** orchestrator SHALL send `Complete` event after cleanup finishes

#### Scenario: User chooses Keep
- **WHEN** user selects Keep option
- **AND** orchestrator receives the choice via oneshot channel
- **THEN** orchestrator SHALL call `checkpoint.cleanup(CompletionOption::Keep)`
- **AND** orchestrator SHALL send `Complete` event after cleanup finishes

#### Scenario: Oneshot channel dropped (force quit)
- **WHEN** the oneshot sender is dropped without sending a choice
- **THEN** orchestrator SHALL treat this as Keep (preserve current state)
- **AND** orchestrator SHALL send `Complete` event

### Requirement: TUI forwards user choice to Orchestrator

The TUI SHALL forward the user's Cleanup/Keep selection to the Orchestrator via the oneshot channel provided in `AwaitingUserChoice` event.

#### Scenario: User confirms selection on completion screen
- **WHEN** user presses Enter to confirm their selection on completion screen
- **THEN** TUI SHALL send the selected `CompletionOption` via the oneshot sender
- **AND** TUI SHALL display a progress indicator while waiting for cleanup

#### Scenario: TUI receives Complete event after cleanup
- **WHEN** TUI receives `Complete` event after forwarding user choice
- **THEN** TUI SHALL transition to the result screen

### Requirement: Cleanup logic removed from main.rs

The cleanup logic in `main.rs` SHALL be removed since the Orchestrator now handles cleanup.

#### Scenario: Completion screen no longer triggers cleanup in main.rs
- **WHEN** user confirms selection on completion screen
- **THEN** `main.rs` SHALL NOT create a new Checkpoint instance
- **AND** `main.rs` SHALL NOT call `checkpoint.cleanup()`
