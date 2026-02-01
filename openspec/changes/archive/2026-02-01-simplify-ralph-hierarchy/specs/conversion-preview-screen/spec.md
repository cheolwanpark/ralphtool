## MODIFIED Requirements

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

## REMOVED Requirements

### Requirement: Show epic groupings
**Reason**: Epic concept removed. Stories are now the top-level grouping.
**Migration**: Replace Epic rendering with Story rendering.
