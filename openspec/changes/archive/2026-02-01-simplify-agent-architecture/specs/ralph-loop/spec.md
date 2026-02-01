## MODIFIED Requirements

### Requirement: Loop orchestration
The system SHALL orchestrate autonomous AI development by spawning a single coding agent for an OpenSpec change with a self-contained prompt.

#### Scenario: Start loop for a change
- **WHEN** user starts the Ralph loop for a change
- **THEN** the system generates a prompt with the change location and workflow instructions
- **AND** spawns a single coding agent with that prompt

#### Scenario: Agent autonomy
- **WHEN** the agent is running
- **THEN** the agent reads change files directly (proposal, design, tasks, specs)
- **AND** implements tasks
- **AND** marks tasks complete by editing tasks.md
- **AND** progresses through stories autonomously

#### Scenario: Loop completion
- **WHEN** the agent completes (exits)
- **THEN** the system emits a Complete event

## REMOVED Requirements

### Requirement: Instruction generation
**Reason**: Replaced by simple prompt template. Agent reads files directly instead of receiving generated instructions.
**Migration**: Use new agent-prompt capability for prompt generation.

### Requirement: Session-based task tracking
**Reason**: Session concept removed. Tasks tracked via tasks.md file directly.
**Migration**: Agent edits tasks.md to mark progress. No session state needed.

## MODIFIED Requirements

### Requirement: Loop events
The system SHALL emit events during loop execution to enable TUI updates.

#### Scenario: Event types
- **WHEN** the loop is running
- **THEN** the system emits AgentOutput and Complete events

#### Scenario: TUI subscription
- **WHEN** the TUI is displaying the loop screen
- **THEN** it receives events via a channel and updates the display accordingly

### Requirement: TUI loop integration
The TUI SHALL spawn the orchestrator when the user starts the loop and display agent output.

#### Scenario: Loop startup
- **WHEN** user presses 'R' to start the loop from the preview screen
- **THEN** the system spawns the agent with the generated prompt
- **AND** displays the loop screen with agent output streaming

#### Scenario: Stop signal propagation
- **WHEN** user presses 'q' on the loop screen
- **THEN** the TUI signals the agent to stop
- **AND** transitions back to selection screen after agent exits
