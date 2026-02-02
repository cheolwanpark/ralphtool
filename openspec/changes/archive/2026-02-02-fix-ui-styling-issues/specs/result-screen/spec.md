## MODIFIED Requirements

### Requirement: Result screen has tabbed interface for Tasks and Changed Files

The result screen SHALL provide a tabbed interface with two tabs:
- Tasks tab: Shows story titles and their tasks with completion checkboxes
- Changed Files tab: Shows git diff output with color-coded status

The currently active tab SHALL be visually highlighted.

#### Scenario: User views Tasks tab
- **WHEN** result screen is displayed
- **THEN** Tasks tab is selected by default
- **AND** content area shows stories with their tasks
- **AND** each task shows [x] if completed or [ ] if not completed

#### Scenario: User views Changed Files tab
- **WHEN** user is on result screen with Changed Files tab selected
- **THEN** content area shows list of changed files from git diff
- **AND** each line shows status character followed by filename
- **AND** only the status character is colored (not the filename)
- **AND** added files have green status character (A)
- **AND** modified files have yellow status character (M)
- **AND** deleted files have red status character (D)
- **AND** filenames are displayed in default color
