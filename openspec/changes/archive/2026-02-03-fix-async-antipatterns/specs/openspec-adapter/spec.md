## MODIFIED Requirements

### Requirement: OpenSpec CLI integration

The adapter SHALL use OpenSpec CLI commands for change enumeration and status checking, using async-safe command execution.

#### Scenario: List available changes

- **WHEN** `list_changes()` is called
- **THEN** the adapter SHALL execute `openspec list --json` via `async_cmd::run()` and parse the JSON response
- **AND** the operation does not block tokio worker threads

#### Scenario: Check change completion status

- **WHEN** `is_complete(change_name)` is called
- **THEN** the adapter SHALL execute `openspec status --change <name> --json` via `async_cmd::run()` and return true if `isComplete` is true
- **AND** the operation does not block tokio worker threads

#### Scenario: CLI not available

- **WHEN** the `openspec` command is not found in PATH
- **THEN** the adapter SHALL return an error with a helpful message indicating OpenSpec CLI is required

#### Scenario: CLI command timeout

- **WHEN** an openspec CLI command does not respond within the timeout period
- **THEN** the adapter SHALL return a timeout error
- **AND** the operation SHALL not block other async tasks indefinitely
