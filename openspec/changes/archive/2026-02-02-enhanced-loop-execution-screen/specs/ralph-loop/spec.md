## MODIFIED Requirements

### Requirement: Loop events
The system SHALL emit events during loop execution to enable TUI updates, including story progress information and full agent output with story association.

#### Scenario: Event types
- **WHEN** the loop is running
- **THEN** the system emits StoryEvent, StoryProgress, Error, and Complete events

#### Scenario: StoryEvent with full message
- **WHEN** the agent emits a StreamEvent::Message
- **THEN** the orchestrator emits a StoryEvent containing the story_id and the full StreamEvent
- **AND** the message content is not truncated

#### Scenario: StoryEvent with Done response
- **WHEN** the agent emits a StreamEvent::Done
- **THEN** the orchestrator emits a StoryEvent containing the story_id and the full StreamEvent
- **AND** the Response includes content, turns, tokens, and cost

#### Scenario: Story progress event
- **WHEN** the orchestrator starts working on a story
- **THEN** it emits a StoryProgress event with current story ID and total count

#### Scenario: TUI subscription
- **WHEN** the TUI is displaying the loop screen
- **THEN** it receives events via a channel and updates the display accordingly
- **AND** stores messages per-story for navigation
- **AND** shows the current story being worked on

### Requirement: TUI loop integration
The TUI SHALL spawn the orchestrator when the user starts the loop and display agent output with story-based navigation.

#### Scenario: Loop startup
- **WHEN** user presses 'R' to start the loop from the preview screen
- **THEN** the system spawns the agent with the generated prompt
- **AND** displays the loop screen with story navigation and tabbed content

#### Scenario: Stop signal propagation
- **WHEN** user presses 'q' on the loop screen
- **THEN** the TUI signals the agent to stop
- **AND** transitions back to selection screen after agent exits
