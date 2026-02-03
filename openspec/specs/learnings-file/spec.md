## Purpose

Learnings file management for Ralph Loop iterations. Provides persistent storage for discoveries, decisions, and gotchas that agents can share across iterations.

## Implementation

Located in `src/ralph_loop/learnings.rs`.

## Requirements

### Requirement: Learnings file path
The system SHALL use `/tmp/ralphtool/{change_name}-learnings.md` as the learnings file path, where `{change_name}` is the name of the current change.

#### Scenario: Path follows convention
- **WHEN** the orchestrator needs the learnings file path for change "add-user-auth"
- **THEN** the path SHALL be `/tmp/ralphtool/add-user-auth-learnings.md`

### Requirement: Ensure learnings file exists
The system SHALL ensure the learnings file exists at the start of each iteration, creating it with initial content if it does not exist.

#### Scenario: Create learnings file when missing
- **WHEN** the orchestrator starts an iteration
- **AND** the learnings file does not exist
- **THEN** the system SHALL create the directory `/tmp/ralphtool/` if needed
- **AND** create the learnings file with initial template content

#### Scenario: Preserve existing learnings file
- **WHEN** the orchestrator starts an iteration
- **AND** the learnings file already exists
- **THEN** the system SHALL NOT modify the existing file

### Requirement: Learnings file initial content
The system SHALL create new learnings files with a markdown template containing a header and guidance comment.

#### Scenario: Initial content structure
- **WHEN** the system creates a new learnings file
- **THEN** the file SHALL contain:
  - A markdown header "# Learnings"
  - A comment block explaining what to record (discoveries, decisions, gotchas)

### Requirement: No file deletion
The system SHALL NOT delete the learnings file at iteration end, allowing it to persist across multiple iteration runs.

#### Scenario: File persists after iteration
- **WHEN** an iteration completes (success or failure)
- **THEN** the learnings file SHALL remain at its location
- **AND** be available for the next iteration run
