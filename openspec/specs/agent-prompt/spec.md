## Purpose

Prompt generation for coding agents. Generates a story-specific prompt that tells the agent how to work on one story of an OpenSpec change.

## Implementation

Located in `src/agent/prompt.rs`. The prompt generation creates a markdown prompt from the spec adapter's context for a specific story.

## Requirements

### Requirement: Agent prompt template
The system SHALL generate a story-specific prompt that tells the agent how to work on one story of a change, including completion and failure signal instructions, and shared learnings when available.

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
  - If verification fails and cannot be fixed, output `<promise>FAILED: {reason}</promise>`
  - The agent MUST output one of these signals

#### Scenario: Prompt contains failure signal instructions
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL explain the FAILED signal:
  - Use `<promise>FAILED: {reason}</promise>` when unable to complete
  - Include a brief description of what went wrong in the reason
  - This allows the orchestrator to retry with context

#### Scenario: Prompt contains tool usage instructions
- **WHEN** the orchestrator generates an agent prompt
- **THEN** the prompt SHALL include spec tool usage instructions from the adapter's `tool_prompt()`
- **AND** these instructions explain how to mark tasks complete in tasks.md

#### Scenario: Prompt contains shared learnings when available
- **WHEN** the orchestrator generates an agent prompt
- **AND** the learnings file exists and contains content beyond the initial template
- **THEN** the prompt SHALL include a "Shared Learnings" section with:
  - Instructions on what to record (discoveries, decisions, gotchas)
  - The path to the learnings file
  - The existing learnings content

#### Scenario: Prompt omits learnings section when empty
- **WHEN** the orchestrator generates an agent prompt
- **AND** the learnings file does not exist or contains only the initial template
- **THEN** the prompt SHALL NOT include a learnings section

### Requirement: Prompt is self-contained
The agent prompt SHALL contain all information needed to work on the story without requiring environment variables or special CLI commands.

#### Scenario: No environment variables required
- **WHEN** an agent receives the prompt
- **THEN** the agent SHALL be able to complete all work using only file operations

#### Scenario: Story-scoped work
- **WHEN** an agent receives the prompt
- **THEN** the agent works only on the specified story
- **AND** does not proceed to the next story (that's the orchestrator's job)

### Requirement: Retry context in prompt
The system SHALL include previous failure information in retry prompts when available.

#### Scenario: Include failure reason on retry
- **WHEN** the orchestrator generates a prompt for a retry attempt
- **AND** the previous attempt output `<promise>FAILED: {reason}</promise>`
- **THEN** the prompt SHALL include a section:
  - Header indicating this is a retry
  - The failure reason from the previous attempt
  - Instruction to try a different approach

#### Scenario: No extra context for abnormal termination
- **WHEN** the orchestrator generates a prompt for a retry attempt
- **AND** the previous attempt had no promise signal (abnormal termination)
- **THEN** the prompt SHALL NOT include a retry context section
