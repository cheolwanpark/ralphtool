## MODIFIED Requirements

### Requirement: Loop orchestration
The system SHALL orchestrate autonomous AI development by iterating through stories, spawning a coding agent for each incomplete story with a story-specific prompt.

#### Scenario: Start loop for a change
- **WHEN** user starts the Ralph loop for a change
- **THEN** the system gets the list of stories from the adapter
- **AND** identifies the first incomplete story
- **AND** generates a story-specific prompt
- **AND** spawns a coding agent for that story

#### Scenario: Story iteration
- **WHEN** an agent completes a story (outputs `<promise>COMPLETE</promise>`)
- **THEN** the system refreshes the story list
- **AND** spawns an agent for the next incomplete story
- **AND** continues until all stories are complete

#### Scenario: Loop completion
- **WHEN** all stories are complete
- **THEN** the system emits a Complete event

### Requirement: Loop events
The system SHALL emit events during loop execution to enable TUI updates, including story progress information.

#### Scenario: Event types
- **WHEN** the loop is running
- **THEN** the system emits AgentOutput, StoryProgress, Error, and Complete events

#### Scenario: Story progress event
- **WHEN** the orchestrator starts working on a story
- **THEN** it emits a StoryProgress event with current story ID and total count

#### Scenario: TUI subscription
- **WHEN** the TUI is displaying the loop screen
- **THEN** it receives events via a channel and updates the display accordingly
- **AND** shows the current story being worked on
