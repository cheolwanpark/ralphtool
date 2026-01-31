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

### Requirement: Quit on Q key
The application SHALL exit when the user presses 'q' or 'Q'.

#### Scenario: Q key exits application
- **WHEN** the user presses the 'q' key
- **THEN** the main loop exits AND the application terminates cleanly
