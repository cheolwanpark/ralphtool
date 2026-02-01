## Purpose

Defines the conversion preview screen that displays Ralph domain data (tasks, stories, scenarios) from a selected OpenSpec change.

## Requirements

### Requirement: Display task hierarchy
The Tasks tab SHALL display the full task hierarchy from the selected change.

#### Scenario: Tasks tab shows story groupings
- **WHEN** the Tasks tab is active
- **THEN** story titles SHALL be displayed as section headers

#### Scenario: Tasks tab shows tasks under stories
- **WHEN** the Tasks tab is active and a story is displayed
- **THEN** all tasks belonging to that story SHALL be listed beneath it
- **AND** each task SHALL show its id, description, and completion status

#### Scenario: Visual completion indicator
- **WHEN** a task is displayed in the Tasks tab
- **THEN** completed tasks SHALL show a checkmark or [x] indicator
- **AND** incomplete tasks SHALL show an empty checkbox or [ ] indicator

### Requirement: Display scenarios grouped by capability

The Scenarios tab SHALL display scenarios organized by their source capability.

#### Scenario: Scenarios tab shows capability groupings

- **WHEN** the Scenarios tab is active
- **THEN** capability names SHALL be displayed as section headers
- **AND** capabilities SHALL be listed in alphabetical order

#### Scenario: Scenarios listed under capability

- **WHEN** a capability section is displayed
- **THEN** all scenarios from that capability SHALL be listed beneath it
- **AND** scenarios SHALL be grouped by their requirement within the capability

#### Scenario: Requirement sub-headers

- **WHEN** scenarios are displayed under a capability
- **THEN** requirement names SHALL appear as sub-headers
- **AND** scenarios belonging to each requirement SHALL be indented under their requirement

#### Scenario: Scenario structure display

- **WHEN** a scenario is displayed under its requirement
- **THEN** the scenario name SHALL be shown
- **AND** GIVEN steps SHALL be listed
- **AND** the WHEN step SHALL be shown
- **AND** THEN steps SHALL be listed

### Requirement: Screen layout
The screen SHALL organize information in a readable layout.

#### Scenario: Tabbed display
- **WHEN** the preview screen is displayed
- **THEN** content SHALL be organized into tabs: Tasks and Scenarios

#### Scenario: Tab bar position
- **WHEN** the preview screen is displayed
- **THEN** the tab bar SHALL appear between the header and content area

#### Scenario: Scrollable content per tab
- **WHEN** content in the active tab exceeds the visible area
- **THEN** the user SHALL be able to scroll through the content using arrow keys or Page Up/Down

### Requirement: Navigation back to selection
The screen SHALL support returning to the change selection screen.

#### Scenario: Escape returns to selection
- **WHEN** the user presses Escape on the preview screen
- **THEN** the screen SHALL transition back to the change selection screen
- **AND** the previously selected change highlight SHALL be preserved

#### Scenario: Show navigation hint
- **WHEN** the preview screen is displayed
- **THEN** a hint showing "Esc: Back to selection" SHALL be visible

### Requirement: Change context display
The screen SHALL show context about the selected change.

#### Scenario: Show change name
- **WHEN** the preview screen is displayed
- **THEN** the name of the selected change SHALL be shown in the header

#### Scenario: Show summary counts
- **WHEN** the preview screen is displayed
- **THEN** summary counts SHALL be shown (e.g., "5 tasks, 3 stories, 8 scenarios")
