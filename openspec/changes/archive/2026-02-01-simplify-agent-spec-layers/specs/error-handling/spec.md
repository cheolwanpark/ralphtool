## ADDED Requirements

### Requirement: Custom error type

The system SHALL use a custom `ralphtool::Error` enum instead of `anyhow::Error` for all spec and agent layer operations.

#### Scenario: Error with machine-readable code
- **WHEN** an error occurs in the agent CLI
- **THEN** the JSON error response includes a `code` field with a machine-readable string

#### Scenario: Session required error
- **WHEN** a command requires RALPH_SESSION but it is not set
- **THEN** error code is `SESSION_REQUIRED`

#### Scenario: Story required error
- **WHEN** a command requires RALPH_STORY but it is not set
- **THEN** error code is `STORY_REQUIRED`

#### Scenario: Change not found error
- **WHEN** a change name does not exist
- **THEN** error code is `CHANGE_NOT_FOUND`

#### Scenario: Task not found error
- **WHEN** a task ID does not exist or is out of scope
- **THEN** error code is `TASK_NOT_FOUND`

#### Scenario: Change locked error
- **WHEN** a change is locked by another session
- **THEN** error code is `CHANGE_LOCKED`

### Requirement: Error type conversions

The error type SHALL implement `From` for common error sources.

#### Scenario: IO error conversion
- **WHEN** a `std::io::Error` occurs
- **THEN** it is convertible to `ralphtool::Error::Io`

#### Scenario: JSON error conversion
- **WHEN** a `serde_json::Error` occurs
- **THEN** it is convertible to `ralphtool::Error::Json`
