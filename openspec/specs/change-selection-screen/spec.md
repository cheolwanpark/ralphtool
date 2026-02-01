## Purpose

Defines the change selection screen where users browse and select completed OpenSpec changes for implementation via the Ralph loop.

## Requirements

### Requirement: Display completed changes list
The screen SHALL display a list of completed OpenSpec changes available for implementation.

#### Scenario: Show completed changes only
- **WHEN** the change selection screen is displayed
- **THEN** only changes where `isComplete` is true SHALL be shown

#### Scenario: Display change metadata
- **WHEN** a change is listed
- **THEN** the change name SHALL be displayed
- **AND** the last modified timestamp SHALL be displayed
- **AND** the task count (if available) SHALL be displayed

#### Scenario: Empty state when no completed changes
- **WHEN** no completed changes exist
- **THEN** a message "No completed changes available" SHALL be displayed

### Requirement: Keyboard navigation
The screen SHALL support keyboard navigation for selecting changes.

#### Scenario: Arrow key navigation
- **WHEN** the user presses Up or Down arrow keys
- **THEN** the selection highlight SHALL move to the previous or next change

#### Scenario: Wrap around navigation
- **WHEN** the user presses Down on the last item
- **THEN** the selection SHALL wrap to the first item
- **AND** pressing Up on the first item SHALL wrap to the last item

#### Scenario: Enter key selects change
- **WHEN** the user presses Enter with a change highlighted
- **THEN** the selected change SHALL be loaded
- **AND** the screen SHALL transition to the conversion preview

#### Scenario: Quit from selection
- **WHEN** the user presses 'q' on the selection screen
- **THEN** the application SHALL exit

### Requirement: Visual feedback
The screen SHALL provide visual feedback for the current selection.

#### Scenario: Highlight current selection
- **WHEN** a change is selected (highlighted)
- **THEN** it SHALL be visually distinct from other items (e.g., different background color or marker)

#### Scenario: Show selection instructions
- **WHEN** the selection screen is displayed
- **THEN** instructions for navigation SHALL be visible in the header keybindings section
- **AND** no separate footer help section SHALL be rendered
