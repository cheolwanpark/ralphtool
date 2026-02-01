## ADDED Requirements

### Requirement: Run loop action

The Preview screen SHALL support starting the Ralph loop for the selected change.

#### Scenario: R key triggers loop start
- **WHEN** the user presses 'R' or 'r' on the Preview screen
- **THEN** the screen SHALL transition to LoopExecution
- **AND** the selected change SHALL be passed to the loop

## MODIFIED Requirements

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

#### Scenario: Unified header
- **WHEN** the preview screen is displayed
- **THEN** the header SHALL use the shared header component
- **AND** the header SHALL display change name and summary counts as context
- **AND** no separate footer help section SHALL be rendered

### Requirement: Navigation back to selection

The screen SHALL support returning to the change selection screen.

#### Scenario: Escape returns to selection
- **WHEN** the user presses Escape on the preview screen
- **THEN** the screen SHALL transition back to the change selection screen
- **AND** the previously selected change highlight SHALL be preserved

#### Scenario: Show navigation hint
- **WHEN** the preview screen is displayed
- **THEN** the Esc keybinding SHALL be visible in the header keybindings section
