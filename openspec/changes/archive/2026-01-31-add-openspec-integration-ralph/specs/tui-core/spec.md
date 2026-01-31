## ADDED Requirements

### Requirement: Screen state management
The application SHALL maintain a current screen state to support multiple screens.

#### Scenario: Initial screen is change selection
- **WHEN** the application starts
- **THEN** the current screen SHALL be set to ChangeSelection

#### Scenario: Screen transitions update state
- **WHEN** a screen transition occurs (e.g., selection to preview)
- **THEN** the current screen state SHALL be updated accordingly

### Requirement: Screen-aware event handling
The application SHALL route keyboard events based on the current screen.

#### Scenario: Events dispatched to current screen
- **WHEN** a keyboard event is received
- **THEN** the event SHALL be handled by the current screen's event handler

#### Scenario: Escape key handling
- **WHEN** the Escape key is pressed
- **THEN** the current screen's back navigation SHALL be triggered (if applicable)

### Requirement: Screen-aware rendering
The application SHALL render the appropriate UI based on the current screen.

#### Scenario: Render dispatches to current screen
- **WHEN** the render function is called
- **THEN** the UI for the current screen SHALL be rendered

## MODIFIED Requirements

### Requirement: Main event loop
The application SHALL run a main loop that polls for events, updates state, and renders the UI.

#### Scenario: Loop processes events
- **WHEN** the main loop is running
- **THEN** keyboard events are polled with a timeout AND the UI is rendered each iteration

#### Scenario: Loop dispatches to screen handlers
- **WHEN** an event is received in the main loop
- **THEN** the event SHALL be dispatched to the current screen's handler
- **AND** screen transitions SHALL be processed

### Requirement: Quit on Q key
The application SHALL exit when the user presses 'q' or 'Q'.

#### Scenario: Q key exits application
- **WHEN** the user presses the 'q' key
- **THEN** the main loop exits AND the application terminates cleanly

#### Scenario: Q key respects screen context
- **WHEN** the user presses 'q' on the change selection screen
- **THEN** the application SHALL exit
- **AND** pressing 'q' on other screens MAY have different behavior based on screen requirements
