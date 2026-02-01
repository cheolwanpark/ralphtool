## MODIFIED Requirements

### Requirement: Screen layout
The screen SHALL organize information in a readable layout.

#### Scenario: Tabbed display
- **WHEN** the preview screen is displayed
- **THEN** content SHALL be organized into tabs: Tasks and User Stories

#### Scenario: Tab bar position
- **WHEN** the preview screen is displayed
- **THEN** the tab bar SHALL appear between the header and content area

#### Scenario: Scrollable content per tab
- **WHEN** content in the active tab exceeds the visible area
- **THEN** the user SHALL be able to scroll through the content using arrow keys or Page Up/Down

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

### Requirement: Display user stories with nested scenarios
The User Stories tab SHALL display user stories with their scenarios nested underneath.

#### Scenario: User Stories tab shows story list
- **WHEN** the User Stories tab is active
- **THEN** all user stories SHALL be displayed

#### Scenario: User Stories tab shows story details
- **WHEN** a user story is displayed in the User Stories tab
- **THEN** the story title SHALL be shown
- **AND** the story description SHALL be shown if present

#### Scenario: Scenarios nested under user stories
- **WHEN** a user story is displayed in the User Stories tab
- **THEN** all scenarios belonging to that user story SHALL be displayed underneath it
- **AND** scenarios SHALL be visually indented to show the parent-child relationship

#### Scenario: Scenario structure display
- **WHEN** a scenario is displayed under its user story
- **THEN** the scenario name SHALL be shown
- **AND** GIVEN steps SHALL be listed
- **AND** the WHEN step SHALL be shown
- **AND** THEN steps SHALL be listed

#### Scenario: User story without scenarios
- **WHEN** a user story has no associated scenarios
- **THEN** the user story SHALL still be displayed
- **AND** no scenario section SHALL appear under it
