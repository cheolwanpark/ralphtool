## ADDED Requirements

### Requirement: Preview to LoopExecution transition

The application SHALL support transitioning from Preview screen to LoopExecution screen.

#### Scenario: R key starts loop from preview
- **WHEN** the user presses 'R' or 'r' on the Preview screen
- **THEN** the application SHALL transition to the LoopExecution screen
- **AND** the loop state SHALL be initialized with the selected change name
- **AND** the loop log SHALL be cleared

#### Scenario: Transition requires selected change
- **WHEN** the user presses 'R' on the Preview screen
- **AND** a change is selected
- **THEN** the LoopExecution screen SHALL display the selected change name
