## Purpose

RAII-style session wrapper that manages session lifecycle and provides subprocess environment configuration. Ensures proper initialization, lock management, and cleanup for orchestrator-spawned coding agents.

## Requirements

### Requirement: Scoped session initialization

The system SHALL provide a `ScopedSession` that initializes a session and acquires the change lock on construction.

#### Scenario: Create scoped session

- **WHEN** `ScopedSession::init(change_name)` is called with a valid change name
- **THEN** system creates a new session via `session init` logic
- **THEN** system acquires exclusive lock on the change
- **THEN** system returns `Ok(ScopedSession)` with session state

#### Scenario: Create scoped session for locked change

- **WHEN** `ScopedSession::init(change_name)` is called for a change with active session
- **THEN** system returns `Err` indicating change is locked

#### Scenario: Create scoped session for invalid change

- **WHEN** `ScopedSession::init(change_name)` is called with non-existent change
- **THEN** system returns `Err` indicating change not found

### Requirement: Session environment access

The system SHALL provide access to session environment variables for subprocess configuration.

#### Scenario: Get session environment

- **WHEN** `session.env()` is called on an active `ScopedSession`
- **THEN** system returns a `HashMap` containing `RALPH_SESSION` with the session ID
- **THEN** if a story is set, the map also contains `RALPH_STORY` with the story ID

### Requirement: Story iteration

The system SHALL support iterating through incomplete stories.

#### Scenario: Get next incomplete story

- **WHEN** `session.next_story()` is called
- **THEN** system queries for the next incomplete story
- **THEN** system updates internal story state
- **THEN** system returns `Ok(Some(story_id))` with the story ID

#### Scenario: All stories complete

- **WHEN** `session.next_story()` is called and all stories are complete
- **THEN** system returns `Ok(None)`

### Requirement: Session cleanup on drop

The system SHALL release resources when `ScopedSession` is dropped.

#### Scenario: Drop releases lock

- **WHEN** a `ScopedSession` goes out of scope or is dropped
- **THEN** system releases the exclusive lock on the change
- **THEN** system removes the session state file

### Requirement: Explicit flush with learnings

The system SHALL support explicit flush to persist learnings before cleanup.

#### Scenario: Flush with learnings

- **WHEN** `session.flush(learnings)` is called with accumulated learnings
- **THEN** system persists learnings via spec adapter
- **THEN** system releases lock and removes session file
- **THEN** session is consumed (cannot be used after flush)
