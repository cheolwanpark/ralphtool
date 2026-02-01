## Purpose

Context retrieval for the current story in a Ralph Loop iteration.

## Requirements

### Requirement: Story context retrieval

The system SHALL return complete context for the current story via `ralphtool agent context`.

#### Scenario: Get context for current story
- **WHEN** agent calls `ralphtool agent context`
- **THEN** system returns JSON containing:
  - `story`: current story object (id, title, description)
  - `tasks`: all tasks in current story with completion status
  - `proposal`: proposal.md content
  - `design`: design.md content
  - `scenarios`: scenarios from delta specs relevant to current story
  - `learnings`: all learnings from previous iterations in this session
  - `patterns`: codebase patterns from specs
  - `verify`: verification commands (checks, tests)

#### Scenario: Context scoped to story
- **WHEN** agent calls `context` with RALPH_STORY=2
- **THEN** system returns only tasks belonging to story 2
- **THEN** system returns only scenarios relevant to story 2

### Requirement: Verification commands

The system SHALL include verification commands in context response.

#### Scenario: Verification commands present
- **WHEN** agent calls `context`
- **THEN** response includes `verify.checks` array with static check commands
- **THEN** response includes `verify.tests` with test command pattern

#### Scenario: Default verification commands
- **WHEN** no project-specific verification config exists
- **THEN** system infers commands from project type (Cargo.toml → `cargo check`, `cargo test`)

### Requirement: Context requires session

The system SHALL require valid session for context retrieval.

#### Scenario: Context without session
- **WHEN** agent calls `context` without RALPH_SESSION
- **THEN** system returns error explaining session requirement

#### Scenario: Context without story scope
- **WHEN** agent calls `context` without RALPH_STORY
- **THEN** system returns error explaining story scope requirement
