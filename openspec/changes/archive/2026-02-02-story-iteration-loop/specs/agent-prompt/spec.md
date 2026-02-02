## MODIFIED Requirements

### Requirement: Agent prompt template
The system SHALL generate a story-specific prompt that tells the agent how to work on one story of a change.

#### Scenario: Prompt contains story context
- **WHEN** the orchestrator generates an agent prompt for story N
- **THEN** the prompt SHALL include:
  - The target story ID and title
  - The tasks belonging to that story
  - Instructions to complete only tasks in this story

#### Scenario: Prompt contains change context
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL include the path to the change directory
- **AND** reference to proposal.md and design.md for context

#### Scenario: Prompt contains scenarios
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL include all scenarios from the specs
- **AND** instruct the agent to focus on scenarios relevant to the current story's tasks

#### Scenario: Prompt contains verification instructions
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL include verification commands
- **AND** instruct the agent to run verification after completing tasks

#### Scenario: Prompt contains completion signal instructions
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL instruct the agent:
  - After completing all tasks in this story, run verification
  - If verification passes, output `<promise>COMPLETE</promise>`
  - If verification fails, fix issues and re-verify before signaling

#### Scenario: Prompt contains tool usage instructions
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL include spec tool usage instructions from the adapter's `tool_prompt()`
- **AND** these instructions explain how to mark tasks complete in tasks.md

### Requirement: Prompt is self-contained
The agent prompt SHALL contain all information needed to work on the story without requiring environment variables or special CLI commands.

#### Scenario: No environment variables required
- **WHEN** an agent receives the prompt
- **THEN** the agent SHALL be able to complete all work using only file operations

#### Scenario: Story-scoped work
- **WHEN** an agent receives the prompt
- **THEN** the agent works only on the specified story
- **AND** does not proceed to the next story (that's the orchestrator's job)
