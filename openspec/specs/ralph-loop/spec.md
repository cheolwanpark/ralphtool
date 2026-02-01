## Purpose

Loop orchestration for autonomous AI development, iterating through stories in an OpenSpec change.

## Requirements

### Requirement: Loop orchestration
The system SHALL orchestrate autonomous AI development by iterating through stories in an OpenSpec change, spawning a coding agent for each story until all stories are complete.

#### Scenario: Start loop for a change
- **WHEN** user starts the Ralph loop for a change
- **THEN** the system loads stories from the change's tasks.md
- **AND** begins processing the first incomplete story

#### Scenario: Story iteration
- **WHEN** a story is being processed
- **THEN** the system generates instructions from the story context
- **AND** spawns a coding agent with those instructions
- **AND** waits for the agent to complete

#### Scenario: Story completion
- **WHEN** the agent completes a story (all tasks marked done)
- **THEN** the system proceeds to the next incomplete story
- **AND** emits a StoryCompleted event

#### Scenario: Loop completion
- **WHEN** all stories in the change are complete
- **THEN** the system flushes accumulated learnings to design.md
- **AND** emits a Complete event

### Requirement: Loop events
The system SHALL emit events during loop execution to enable TUI updates.

#### Scenario: Event types
- **WHEN** the loop is running
- **THEN** the system emits StoryStarted, TaskCompleted, StoryCompleted, AgentOutput, Error, and Complete events

#### Scenario: TUI subscription
- **WHEN** the TUI is displaying the loop screen
- **THEN** it receives events via a channel and updates the display accordingly

### Requirement: Loop control
The system SHALL allow the user to stop the loop.

#### Scenario: User stops loop
- **WHEN** user presses 'q' during loop execution
- **THEN** the system stops after the current agent completes
- **AND** preserves any completed work

### Requirement: Instruction generation
The session module SHALL generate AI instructions from spec layer context.

#### Scenario: Generate instructions for a story
- **WHEN** the loop requests instructions for a story
- **THEN** the session module builds a markdown prompt from proposal, design, story tasks, scenarios, and verify commands

### Requirement: TUI loop integration
The TUI SHALL spawn the orchestrator when the user starts the loop and update the display based on orchestrator events.

#### Scenario: Loop startup with real counts
- **WHEN** user presses 'R' to start the loop from the preview screen
- **THEN** the system loads stories from the spec adapter
- **AND** initializes LoopState with accurate story and task counts
- **AND** spawns the orchestrator in a background thread
- **AND** displays the loop screen with correct initial counts

#### Scenario: Real-time progress updates
- **WHEN** the orchestrator emits a LoopEvent
- **THEN** the TUI receives the event via channel
- **AND** updates the LoopState accordingly
- **AND** re-renders the loop screen

#### Scenario: Stop signal propagation
- **WHEN** user presses 'q' on the loop screen
- **THEN** the TUI sets the orchestrator's stop flag
- **AND** the orchestrator stops after completing the current agent run
- **AND** the TUI transitions back to selection screen