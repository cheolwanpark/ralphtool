## Purpose

Defines the visual rendering requirements for the TUI application's user interface.

## Requirements

### Requirement: Basic UI rendering
The application SHALL render a basic UI frame using ratatui's Terminal::draw method.

#### Scenario: UI renders without error
- **WHEN** the render function is called
- **THEN** a frame is drawn to the terminal without errors

### Requirement: Welcome message display
The application SHALL display a welcome message or title indicating the application is running.

#### Scenario: Welcome message visible
- **WHEN** the application is running
- **THEN** a welcome message or application title is visible in the terminal

### Requirement: Quit instructions display
The application SHALL display instructions for how to quit (press q to exit).

#### Scenario: Quit instructions visible
- **WHEN** the application is running
- **THEN** text indicating "Press q to quit" or similar is visible
