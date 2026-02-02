## Purpose

Story-by-story iteration for autonomous AI development. The orchestrator iterates through stories, spawning an agent for each incomplete story with a completion signal protocol.

## Requirements

### Requirement: Story iteration loop
The orchestrator SHALL iterate through stories one at a time, spawning an agent for each incomplete story.

#### Scenario: Iterate through incomplete stories
- **WHEN** the orchestrator runs
- **THEN** it gets the list of stories from the adapter
- **AND** for each incomplete story in order, spawns an agent with a story-specific prompt
- **AND** waits for the agent to complete before proceeding to the next story

#### Scenario: Refresh state between iterations
- **WHEN** an agent completes a story
- **THEN** the orchestrator refreshes the story list from the adapter
- **AND** checks which stories are now complete
- **AND** continues with the next incomplete story

#### Scenario: All stories complete
- **WHEN** all stories are marked complete
- **THEN** the orchestrator emits a Complete event
- **AND** exits the loop

### Requirement: Completion signal
The agent SHALL output `<promise>COMPLETE</promise>` when a story is done and verified.

#### Scenario: Completion after verification
- **WHEN** the agent completes all tasks in a story
- **AND** runs verification commands successfully
- **THEN** the agent outputs `<promise>COMPLETE</promise>`

#### Scenario: No completion on verification failure
- **WHEN** verification commands fail
- **THEN** the agent SHALL NOT output `<promise>COMPLETE</promise>`
- **AND** attempts to fix issues and re-verify

#### Scenario: Orchestrator detects completion
- **WHEN** the orchestrator receives `<promise>COMPLETE</promise>` in agent output
- **THEN** it marks the current story iteration as complete
- **AND** proceeds to refresh state and check for next story

### Requirement: Progress tracking
The orchestrator SHALL track which story is currently being worked on.

#### Scenario: Current story tracking
- **WHEN** the orchestrator spawns an agent for a story
- **THEN** it updates its internal state to reflect the current story
- **AND** emits events that include the current story ID

#### Scenario: Story progress in events
- **WHEN** the loop is running
- **THEN** loop events include the current story ID and total story count
