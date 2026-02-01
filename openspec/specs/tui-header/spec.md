## Purpose

Defines a shared header component for all TUI screens that provides consistent branding, keybinding display, and screen context.

## Requirements

### Requirement: Unified header layout

The header component SHALL render a three-column layout within a bordered block.

#### Scenario: Header contains ASCII art branding
- **WHEN** any screen renders its header
- **THEN** the left column SHALL display the RalphTool ASCII art logo
- **AND** the ASCII art SHALL fit within 3 lines and approximately 18 characters width

#### Scenario: Header contains screen title
- **WHEN** any screen renders its header
- **THEN** the center column SHALL display the current screen name
- **AND** screen-specific context information MAY be displayed below the title

#### Scenario: Header contains keybindings
- **WHEN** any screen renders its header
- **THEN** the right column SHALL display screen-specific keybindings
- **AND** keybindings SHALL be formatted as "Key Action" (e.g., "↑↓ Navigate")

### Requirement: Screen-specific keybinding display

Each screen SHALL provide its own set of keybindings to the header component.

#### Scenario: Selection screen keybindings
- **WHEN** the Selection screen is displayed
- **THEN** the header SHALL show: ↑↓ Navigate, Enter Select, q Quit

#### Scenario: Preview screen keybindings
- **WHEN** the Preview screen is displayed
- **THEN** the header SHALL show: ↑↓ Scroll, Tab Switch, R Run, Esc Back, q Quit

#### Scenario: LoopExecution screen keybindings
- **WHEN** the LoopExecution screen is displayed
- **THEN** the header SHALL show: q Stop

#### Scenario: Result screen keybindings
- **WHEN** the Result screen is displayed
- **THEN** the header SHALL show: ↑↓ Scroll, Esc Back, q Quit

### Requirement: ASCII art specification

The ASCII art logo SHALL use Unicode box-drawing characters for terminal compatibility.

#### Scenario: Logo renders correctly
- **WHEN** the header is rendered
- **THEN** the logo SHALL display "Ralph" using box-drawing characters
- **AND** the logo SHALL be exactly 3 lines tall

### Requirement: Footer removal

Screens SHALL NOT render separate footer sections for help text.

#### Scenario: Help text consolidated in header
- **WHEN** any screen is rendered
- **THEN** keybinding information SHALL appear only in the header
- **AND** no separate footer help section SHALL be rendered
