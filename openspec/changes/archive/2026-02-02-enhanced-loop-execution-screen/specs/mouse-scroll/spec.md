## ADDED Requirements

### Requirement: Mouse scroll support
The TUI SHALL support mouse wheel scrolling on all scrollable screens.

#### Scenario: Enable mouse capture
- **WHEN** the TUI initializes
- **THEN** mouse capture is enabled for the terminal

#### Scenario: Disable mouse capture on exit
- **WHEN** the TUI exits
- **THEN** mouse capture is disabled before restoring terminal

#### Scenario: Scroll up with mouse wheel
- **WHEN** user scrolls mouse wheel up on a scrollable screen
- **THEN** the content scrolls up (earlier content becomes visible)

#### Scenario: Scroll down with mouse wheel
- **WHEN** user scrolls mouse wheel down on a scrollable screen
- **THEN** the content scrolls down (later content becomes visible)

#### Scenario: Mouse scroll on Preview screen
- **WHEN** user scrolls mouse wheel on the Preview screen
- **THEN** the active tab's content scrolls accordingly

#### Scenario: Mouse scroll on Loop Execution screen
- **WHEN** user scrolls mouse wheel on the Loop Execution screen
- **THEN** the active tab's content scrolls accordingly

#### Scenario: Mouse scroll on Result screen
- **WHEN** user scrolls mouse wheel on the Result screen
- **THEN** the content scrolls accordingly
