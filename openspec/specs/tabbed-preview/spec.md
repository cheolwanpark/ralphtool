## Purpose

Defines the tabbed interface behavior for the preview screen, including tab navigation, state management, and per-tab scroll positions.

## Requirements

### Requirement: Tab state management
The preview screen SHALL maintain an active tab state that determines which content is displayed.

#### Scenario: Initial tab is Tasks
- **WHEN** the preview screen is first displayed
- **THEN** the Tasks tab SHALL be active

#### Scenario: Tab state persists during session
- **WHEN** the user switches tabs and scrolls
- **THEN** the active tab SHALL remain selected until explicitly changed

### Requirement: Tab switching with keyboard
The preview screen SHALL support keyboard navigation between tabs.

#### Scenario: Tab key switches to next tab
- **WHEN** the user presses the Tab key on the preview screen
- **THEN** the active tab SHALL switch to the next tab in order
- **AND** if on the last tab, it SHALL wrap to the first tab

#### Scenario: Shift+Tab switches to previous tab
- **WHEN** the user presses Shift+Tab on the preview screen
- **THEN** the active tab SHALL switch to the previous tab in order
- **AND** if on the first tab, it SHALL wrap to the last tab

### Requirement: Tab bar display
The preview screen SHALL display a tab bar showing available tabs.

#### Scenario: Tab bar shows all tabs
- **WHEN** the preview screen is displayed
- **THEN** a tab bar SHALL be visible below the header
- **AND** all tab names SHALL be displayed

#### Scenario: Active tab is visually indicated
- **WHEN** a tab is active
- **THEN** the active tab name SHALL be enclosed in brackets (e.g., `[Tasks]`)
- **AND** inactive tabs SHALL be displayed without brackets

#### Scenario: Tab bar format
- **WHEN** the tab bar is rendered
- **THEN** tabs SHALL be separated by ` | ` delimiter

### Requirement: Per-tab scroll position
The preview screen SHALL maintain separate scroll positions for each tab.

#### Scenario: Scroll position preserved on tab switch
- **WHEN** the user scrolls in one tab and switches to another tab
- **AND** then switches back to the original tab
- **THEN** the scroll position SHALL be restored to where it was

#### Scenario: Independent scroll positions
- **WHEN** the user scrolls in the Tasks tab
- **THEN** the User Stories tab scroll position SHALL NOT be affected

### Requirement: Tab-aware help text
The help bar SHALL include tab switching instructions.

#### Scenario: Help shows tab navigation
- **WHEN** the preview screen is displayed
- **THEN** the help bar SHALL include "Tab/Shift+Tab Switch" or equivalent hint
