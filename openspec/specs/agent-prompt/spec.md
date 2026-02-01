## Purpose

Prompt generation for coding agents. Generates a self-contained prompt that tells the agent how to work on an OpenSpec change autonomously.

## Implementation

Located in `src/spec/prompt.rs`. The `generate_prompt` function creates a markdown prompt from the spec adapter's context.

## Requirements

### Requirement: Agent prompt template
The system SHALL generate a prompt that tells the agent how to work on a change autonomously.

#### Scenario: Prompt contains change location
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL include the path to the change directory (e.g., `openspec/changes/<name>/`)

#### Scenario: Prompt contains workflow instructions
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL instruct the agent to:
  - Read `proposal.md` for motivation
  - Read `design.md` for technical decisions
  - Read `tasks.md` for stories and tasks
  - Read `specs/` for detailed requirements

#### Scenario: Prompt contains task marking instructions
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL instruct the agent to mark tasks complete by editing `tasks.md`
- **AND** changing `[ ]` to `[x]` for completed tasks

#### Scenario: Prompt contains verification instructions
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL include verification commands (e.g., `cargo check`, `cargo test`)

#### Scenario: Prompt contains story progression instructions
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL instruct the agent to complete all tasks in Story 1 before moving to Story 2

### Requirement: Prompt is self-contained
The agent prompt SHALL contain all information needed to work on the change without requiring environment variables or special CLI commands.

#### Scenario: No environment variables required
- **WHEN** an agent receives the prompt
- **THEN** the agent SHALL be able to complete all work using only file operations
- **AND** no `RALPH_*` environment variables are needed

#### Scenario: No special CLI commands required
- **WHEN** an agent receives the prompt
- **THEN** the agent SHALL NOT need to run `ralphtool agent *` commands
- **AND** all state is managed through direct file operations
