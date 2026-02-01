## Purpose

Defines the conversion preview screen that displays Ralph domain data (tasks, stories, scenarios) from a selected OpenSpec change.

## Requirements

### Requirement: Display task hierarchy
The screen SHALL display the full task hierarchy from the selected change.

#### Scenario: Show story groupings
- **WHEN** the preview screen is displayed
- **THEN** story titles SHALL be displayed as section headers

#### Scenario: Show tasks under stories
- **WHEN** a story is displayed
- **THEN** all tasks belonging to that story SHALL be listed beneath it
- **AND** each task SHALL show its id, description, and completion status

#### Scenario: Visual completion indicator
- **WHEN** a task is displayed
- **THEN** completed tasks SHALL show a checkmark or [x] indicator
- **AND** incomplete tasks SHALL show an empty checkbox or [ ] indicator

### Requirement: Display user stories
The screen SHALL display user stories extracted from specs.

#### Scenario: Show story list
- **WHEN** the preview screen is displayed
- **THEN** a section showing user stories SHALL be visible

#### Scenario: Show story details
- **WHEN** a user story is displayed
- **THEN** the story title SHALL be shown
- **AND** the story description SHALL be shown
- **AND** acceptance criteria SHALL be listed

### Requirement: Display verification scenarios
The screen SHALL display verification scenarios from specs.

#### Scenario: Show scenario list
- **WHEN** the preview screen is displayed
- **THEN** verification scenarios SHALL be visible

#### Scenario: Show scenario structure
- **WHEN** a scenario is displayed
- **THEN** the scenario name SHALL be shown
- **AND** GIVEN steps SHALL be listed
- **AND** the WHEN step SHALL be shown
- **AND** THEN steps SHALL be listed

### Requirement: Screen layout
The screen SHALL organize information in a readable layout.

#### Scenario: Sectioned display
- **WHEN** the preview screen is displayed
- **THEN** content SHALL be organized into clear sections (Tasks, Stories, Scenarios)

#### Scenario: Scrollable content
- **WHEN** content exceeds the visible area
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
