## ADDED Requirements

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
