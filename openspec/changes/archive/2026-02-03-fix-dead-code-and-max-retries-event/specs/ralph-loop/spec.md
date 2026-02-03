## MODIFIED Requirements

### Requirement: Loop events
The system SHALL emit events during loop execution to enable TUI updates, including story progress information, full agent output with story association, and max retries exceeded notification.

#### Scenario: Event types
- **WHEN** the loop is running
- **THEN** the system emits StoryEvent, StoryProgress, Error, MaxRetriesExceeded, and Complete events

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

#### Scenario: Max retries exceeded event
- **WHEN** retry count reaches max retries for a story
- **THEN** the orchestrator emits a MaxRetriesExceeded event with the story_id
- **AND** the TUI tracks this event to determine completion reason
- **AND** the completion screen shows CompletionReason::MaxRetries with the failed story ID
