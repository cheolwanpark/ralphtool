## Purpose

Provides a unified header section component for the TUI that displays the application logo, screen title, description, and keybindings in a consistent vertical layout. Supports responsive behavior with compact mode for small terminals.

## Requirements

### Requirement: Header section with ASCII logo
The TUI SHALL display a header section containing an ASCII art logo reading "RALPH" using block characters. The logo SHALL use the Slim Block style spanning 2 lines.

#### Scenario: Logo is displayed
- **WHEN** the TUI renders on a terminal with height >= 24 lines
- **THEN** the header section displays the RALPH ASCII logo at the top

#### Scenario: Logo format
- **WHEN** the logo is rendered
- **THEN** the logo displays as two lines: "█▀█ ▄▀█ █   █▀█ █ █" and "█▀▄ █▀█ █▄▄ █▀▀ █▀█"

### Requirement: Header section includes screen title
The TUI header section SHALL display the current screen title with a diamond icon prefix (◆). The title SHALL clearly identify which screen the user is viewing.

#### Scenario: Selection screen title
- **WHEN** the Selection screen is displayed
- **THEN** the header shows "◆ Change Selection"

#### Scenario: Preview screen title
- **WHEN** the Preview screen is displayed
- **THEN** the header shows "◆ Preview"

#### Scenario: Loop screen title
- **WHEN** the Loop Execution screen is displayed
- **THEN** the header shows "◆ Loop Execution"

#### Scenario: Result screen title
- **WHEN** the Result screen is displayed
- **THEN** the header shows "◆ Result"

### Requirement: Header section includes description
The TUI header section SHALL display a brief description of the current screen's purpose or context below the title.

#### Scenario: Selection screen description
- **WHEN** the Selection screen is displayed
- **THEN** the header shows a description like "Select a change to preview and run"

#### Scenario: Preview screen description with context
- **WHEN** the Preview screen is displayed for change "my-change"
- **THEN** the header shows context including the change name and counts

### Requirement: Header section includes keybindings
The TUI header section SHALL display available keyboard shortcuts at the bottom of the header area.

#### Scenario: Keybindings are displayed
- **WHEN** any screen is rendered
- **THEN** the header section shows relevant keybindings for that screen

#### Scenario: Selection keybindings
- **WHEN** the Selection screen is displayed
- **THEN** keybindings include navigation (↑↓), selection (Enter), and quit (q)

### Requirement: Header occupies 20% of height
The header section SHALL occupy approximately 20% of the available vertical space when the logo is displayed.

#### Scenario: Header height on standard terminal
- **WHEN** the terminal height is 40 lines
- **THEN** the header section is approximately 8 lines tall

### Requirement: Compact header for small terminals
The TUI SHALL display a compact single-line header when the terminal height is less than 24 lines. The compact header SHALL hide the logo and combine title with keybindings.

#### Scenario: Compact mode activation
- **WHEN** the terminal height is 20 lines
- **THEN** the logo is hidden and the header is a single line

#### Scenario: Compact header format
- **WHEN** compact mode is active
- **THEN** the header displays format like "◆ Selection │ ↑↓ Navigate  Enter Select  q Quit"

### Requirement: Consistent header across screens
The header section layout and styling SHALL be consistent across all TUI screens, using the same logo, positioning, and visual hierarchy.

#### Scenario: Header consistency
- **WHEN** navigating from Selection to Preview to Loop to Result screens
- **THEN** the header section maintains the same layout structure and styling
