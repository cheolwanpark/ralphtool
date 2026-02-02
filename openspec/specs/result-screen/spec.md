## Purpose

Result screen displayed after loop execution completes, showing completion summary, task status, and changed files in a tabbed interface.

## Requirements

### Requirement: Result screen displays accurate completion summary

The result screen SHALL display accurate counts of completed stories and tasks based on the actual tasks.md file state after loop execution.

The Summary section SHALL show:
- Stories completed vs total (e.g., "Stories: 3/5 completed")
- Tasks completed vs total (e.g., "Tasks: 12/15 completed")

#### Scenario: Summary shows actual completion counts
- **WHEN** loop execution completes and result screen is displayed
- **THEN** Summary shows the number of stories where all tasks are marked complete out of total stories
- **AND** Summary shows the number of tasks marked [x] out of total tasks

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
- **AND** added files are shown in green
- **AND** modified files are shown in yellow
- **AND** deleted files are shown in red

### Requirement: Tab key switches between tabs

The user SHALL be able to switch between Tasks and Changed Files tabs using the Tab key.

#### Scenario: Switch from Tasks to Changed Files
- **WHEN** user is on Tasks tab and presses Tab key
- **THEN** Changed Files tab becomes active
- **AND** content area updates to show changed files

#### Scenario: Switch from Changed Files to Tasks
- **WHEN** user is on Changed Files tab and presses Tab key
- **THEN** Tasks tab becomes active
- **AND** content area updates to show task list

### Requirement: Each tab maintains independent scroll position

Each tab SHALL maintain its own scroll offset so that switching tabs preserves the user's scroll position within each tab.

#### Scenario: Scroll position preserved when switching tabs
- **WHEN** user scrolls down in Tasks tab
- **AND** user switches to Changed Files tab
- **AND** user switches back to Tasks tab
- **THEN** Tasks tab shows the same scroll position as before switching

### Requirement: Keybindings display includes Tab instruction

The keybindings display at the bottom of the result screen SHALL include the Tab key instruction for switching tabs.

#### Scenario: Keybindings show Tab instruction
- **WHEN** result screen is displayed
- **THEN** keybindings show "Tab Switch" along with other keybindings
