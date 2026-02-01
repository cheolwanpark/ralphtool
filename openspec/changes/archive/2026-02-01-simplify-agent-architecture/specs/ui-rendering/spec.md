## MODIFIED Requirements

### Requirement: Loop execution screen
The loop execution screen SHALL display agent output and allow user to stop the loop.

#### Scenario: Display agent output
- **WHEN** the loop execution screen is active
- **THEN** the screen SHALL stream agent stdout to the log area

#### Scenario: Display completion status
- **WHEN** the agent completes
- **THEN** the screen SHALL show "Loop completed" message
- **AND** allow user to view results or return to selection

## REMOVED Requirements

### Requirement: Session-based progress display
**Reason**: Session state removed. Progress comes from file watching or agent output.
**Migration**: TUI watches tasks.md or parses agent output for progress indication.

### Requirement: Story/task counters from session
**Reason**: No session to track counts. Agent manages its own progress.
**Migration**: If needed, TUI can parse tasks.md to count completed tasks.
