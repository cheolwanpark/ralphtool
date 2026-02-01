## Purpose

Agent CLI layer abstraction and session state management for machine-to-machine interaction.

## Requirements

### Requirement: Agent layer abstraction

The agent layer SHALL use only the `SpecAdapter` trait and factory function. It SHALL NOT import concrete adapter types.

#### Scenario: No OpenSpec imports in agent layer
- **WHEN** compiling the agent module
- **THEN** there are no imports from `crate::spec::openspec`

#### Scenario: Adapter creation via factory
- **WHEN** agent commands need a spec adapter
- **THEN** they call `spec::create_adapter(change_name)`

### Requirement: Simplified session state

Session state SHALL contain only essential fields: `id`, `change`, `story_id`, and `learnings`.

#### Scenario: Session structure
- **WHEN** a session is created
- **THEN** it contains only `{id, change, story_id: Option, learnings: Vec<String>}`

#### Scenario: Learning storage
- **WHEN** a learning is recorded
- **THEN** it is stored as a plain string without metadata

### Requirement: Simplified learn command

The learn command SHALL accept a description and store it as a plain string.

#### Scenario: Record learning
- **WHEN** `ralph agent learn "discovered X"` is called
- **THEN** the string is appended to session.learnings
- **THEN** response includes success status and count

### Requirement: No separate verification commands

The system SHALL NOT have separate verify commands. Verification is indicated by AI outputting `<promise>VERIFIED</promise>`.

#### Scenario: Verification token
- **WHEN** AI verifies a story
- **THEN** it outputs `<promise>VERIFIED</promise>` in its response
