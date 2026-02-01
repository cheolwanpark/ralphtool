## ADDED Requirements

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
