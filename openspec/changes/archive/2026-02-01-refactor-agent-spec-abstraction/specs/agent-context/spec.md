## MODIFIED Requirements

### Requirement: Story context retrieval

The system SHALL return complete context for the current story via `ralphtool agent context`.

#### Scenario: Get context for current story

- **WHEN** agent calls `ralphtool agent context`
- **THEN** system calls spec adapter's `get_context(story_id)`
- **THEN** system returns JSON containing:
  - `story`: current story object (id, title, description)
  - `tasks`: all tasks in current story with completion status
  - `proposal`: proposal.md content
  - `design`: design.md content
  - `scenarios`: scenarios from delta specs relevant to current story
  - `learnings`: all learnings from session state
  - `patterns`: codebase patterns from session state
  - `verify`: verification commands (checks, tests)

#### Scenario: Context scoped to story

- **WHEN** agent calls `context` with RALPH_STORY=2
- **THEN** spec adapter returns only tasks belonging to story 2
- **THEN** spec adapter returns only scenarios relevant to story 2

### Requirement: Verification commands

The system SHALL include verification commands in context response.

#### Scenario: Verification commands present

- **WHEN** agent calls `context`
- **THEN** response includes `verify.checks` array with static check commands
- **THEN** response includes `verify.tests` with test command pattern

#### Scenario: Default verification commands

- **WHEN** no project-specific verification config exists
- **THEN** spec adapter infers commands from project type (Cargo.toml → `cargo check`, `cargo test`)

### Requirement: Context requires session

The system SHALL require valid session for context retrieval.

#### Scenario: Context without session

- **WHEN** agent calls `context` without RALPH_SESSION
- **THEN** system returns error explaining session requirement

#### Scenario: Context without story scope

- **WHEN** agent calls `context` without RALPH_STORY
- **THEN** system returns error explaining story scope requirement

## ADDED Requirements

### Requirement: Context uses spec adapter

The context command SHALL use the spec adapter's ContextProvider trait, not read files directly.

#### Scenario: No direct file reads

- **WHEN** agent calls `context`
- **THEN** system calls `adapter.get_context(story_id)` to retrieve proposal, design, and tasks
- **THEN** system does NOT read proposal.md or design.md directly

### Requirement: Learnings from session

The context SHALL include learnings from session state, not from spec adapter.

#### Scenario: Session learnings included

- **WHEN** agent calls `context`
- **THEN** system reads learnings from session state
- **THEN** learnings from all previous iterations in this session are included
