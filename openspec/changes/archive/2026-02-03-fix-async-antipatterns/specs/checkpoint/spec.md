## MODIFIED Requirements

### Requirement: Checkpoint save

The system SHALL save the current working directory state before spawning an agent for a story using git stash with untracked files included, using async-safe command execution.

#### Scenario: Save checkpoint before agent spawn

- **WHEN** the orchestrator is about to spawn an agent for a story
- **THEN** the system executes `git stash push -u -m "ralph:{change_name}:{story_id}"` via `async_cmd::run()`
- **AND** the stash includes both tracked and untracked files
- **AND** the operation does not block tokio worker threads

#### Scenario: Stash naming convention

- **WHEN** the system creates a checkpoint for story "story-1" in change "my-feature"
- **THEN** the stash message is "ralph:my-feature:story-1"

### Requirement: Checkpoint revert

The system SHALL restore the working directory to a previously saved checkpoint state using git stash apply, using async-safe command execution.

#### Scenario: Revert to checkpoint on failure

- **WHEN** an agent fails (no COMPLETE signal) and retries are available
- **THEN** the system finds the stash with matching "ralph:{change}:{story}" message
- **AND** executes `git stash apply stash@{n}` via `async_cmd::run()` to restore state
- **AND** the stash is NOT dropped (preserved for potential further retries)
- **AND** the operation does not block tokio worker threads

#### Scenario: Revert cleans working directory

- **WHEN** the system reverts to a checkpoint
- **THEN** all changes made by the failed agent are discarded via async git commands
- **AND** untracked files created by the agent are removed

### Requirement: Checkpoint drop

The system SHALL remove a checkpoint stash when a story completes successfully, using async-safe command execution.

#### Scenario: Drop checkpoint on success

- **WHEN** an agent completes successfully (outputs COMPLETE signal)
- **THEN** the system finds the stash with matching "ralph:{change}:{story}" message
- **AND** executes `git stash drop stash@{n}` via `async_cmd::run()`
- **AND** the operation does not block tokio worker threads

### Requirement: Checkpoint cleanup

The system SHALL provide a cleanup operation that removes all stashes for a change, using async-safe command execution.

#### Scenario: Cleanup all change stashes

- **WHEN** the orchestrator finishes (success or max retries exceeded)
- **THEN** the system drops all stashes matching "ralph:{change_name}:*" via async git commands

#### Scenario: Stash lookup by name

- **WHEN** the system needs to find a specific checkpoint
- **THEN** it parses `git stash list` output (via `async_cmd::run()`) to find the stash index by message pattern
