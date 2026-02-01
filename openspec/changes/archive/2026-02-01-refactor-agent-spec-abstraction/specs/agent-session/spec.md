## MODIFIED Requirements

### Requirement: Session initialization

The system SHALL create an isolated session when `ralphtool agent session init` is called with a change name.

#### Scenario: Initialize new session

- **WHEN** orchestrator calls `ralphtool agent session init --change <name>` with `RALPH_SESSION` env set
- **THEN** system creates session state file in OS temp directory (`temp_dir()/ralph/sessions/<session_id>.json`)
- **THEN** system acquires exclusive lock on the change
- **THEN** system returns JSON with session metadata and list of stories

#### Scenario: Session already exists for change

- **WHEN** orchestrator calls `session init` for a change that has an active session
- **THEN** system returns error indicating change is locked by another session

#### Scenario: Invalid change name

- **WHEN** orchestrator calls `session init` with non-existent change
- **THEN** system returns error indicating change not found

### Requirement: Story iteration

The system SHALL provide the next incomplete story for iteration via `ralphtool agent session next-story`.

#### Scenario: Get next story

- **WHEN** orchestrator calls `ralphtool agent session next-story`
- **THEN** system queries spec adapter for next incomplete story
- **THEN** system returns JSON with next incomplete story ID
- **THEN** system sets internal story scope for subsequent agent commands

#### Scenario: All stories complete

- **WHEN** orchestrator calls `session next-story` and all stories are complete
- **THEN** system returns JSON with `{ "complete": true }`

### Requirement: Session flush

The system SHALL persist accumulated state via spec adapter when `ralphtool agent session flush` is called.

#### Scenario: Flush with learnings

- **WHEN** orchestrator calls `ralphtool agent session flush`
- **THEN** system calls spec adapter's `write_learnings()` with accumulated learnings
- **THEN** system releases change lock
- **THEN** system removes session state file

#### Scenario: Flush with patterns

- **WHEN** orchestrator calls `session flush` with accumulated patterns
- **THEN** system calls spec adapter's `write_patterns()` with accumulated patterns

#### Scenario: Flush empty session

- **WHEN** orchestrator calls `session flush` with no accumulated learnings or patterns
- **THEN** system releases lock and removes session file without calling write methods

### Requirement: Session required

The system SHALL require `RALPH_SESSION` environment variable for all agent commands.

#### Scenario: Missing session environment

- **WHEN** any `ralphtool agent` command is called without `RALPH_SESSION` env
- **THEN** system returns error with message explaining session requirement
- **THEN** system suggests using orchestrator to manage sessions

### Requirement: Session state storage

The system SHALL store session state in OS-agnostic temporary directory.

#### Scenario: Session file location

- **WHEN** session is initialized
- **THEN** session file is created at `std::env::temp_dir()/ralph/sessions/<session_id>.json`

#### Scenario: Session state contents

- **WHEN** session is active
- **THEN** session file contains: session_id, change_name, current_story_id, created_at, learnings, patterns, completed_tasks

## ADDED Requirements

### Requirement: Session owns learnings

The session layer SHALL own the learnings buffer, not the spec abstraction.

#### Scenario: Learnings stored in session

- **WHEN** agent records a learning
- **THEN** learning is stored in session state's `learnings` array
- **THEN** learning includes description, optional task_id, story_id, and timestamp

### Requirement: Session owns patterns

The session layer SHALL own the patterns buffer, not the spec abstraction.

#### Scenario: Patterns stored in session

- **WHEN** agent records a pattern
- **THEN** pattern is stored in session state's `patterns` array
- **THEN** pattern includes name and description
