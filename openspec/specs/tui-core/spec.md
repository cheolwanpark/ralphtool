## Purpose

Defines core TUI lifecycle management including terminal initialization, restoration, event loop, and graceful shutdown.

## Requirements

### Requirement: Terminal initialization
The application SHALL initialize the terminal for TUI mode by enabling raw mode and entering the alternate screen.

#### Scenario: Terminal enters TUI mode on start
- **WHEN** the application starts
- **THEN** raw mode is enabled AND the alternate screen is entered

### Requirement: Terminal restoration on exit
The application SHALL restore the terminal to its original state on normal exit by disabling raw mode and leaving the alternate screen.

#### Scenario: Clean exit restores terminal
- **WHEN** the application exits normally (user quits)
- **THEN** raw mode is disabled AND the alternate screen is exited AND the cursor is visible

### Requirement: Terminal restoration on panic
The application SHALL install a panic hook that restores the terminal before the panic unwinds.

#### Scenario: Panic restores terminal
- **WHEN** the application panics
- **THEN** the terminal is restored before the panic message is printed

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
