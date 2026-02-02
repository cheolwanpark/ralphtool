## Purpose

Dual-tab interface (Info/Agent) for the loop execution screen. Allows users to view story details and agent messages in separate tabs with independent scroll positions.

## Requirements

### Requirement: Tab switching in loop screen
The loop execution screen SHALL support switching between Info and Agent tabs.

#### Scenario: Switch tabs with Tab key
- **WHEN** user presses Tab key on the loop screen
- **THEN** the active tab toggles between Info and Agent

#### Scenario: Tab indicator display
- **WHEN** displaying the story section
- **THEN** the current tab is visually indicated (e.g., brackets around active tab name)
- **AND** both tab names are visible

#### Scenario: Independent scroll positions
- **WHEN** user switches between tabs
- **THEN** each tab maintains its own scroll position
- **AND** returning to a tab restores its previous scroll position

### Requirement: Info tab content
The Info tab SHALL display story details and task list for the selected story.

#### Scenario: Display story information
- **WHEN** Info tab is active
- **THEN** the story title is displayed
- **AND** the story ID is visible

#### Scenario: Display task list with status
- **WHEN** Info tab is active
- **THEN** all tasks for the selected story are listed
- **AND** completed tasks show a checked box (☑)
- **AND** incomplete tasks show an unchecked box (☐)

### Requirement: Agent tab content
The Agent tab SHALL display full agent messages with role identification and spacing.

#### Scenario: Display messages with role prefix
- **WHEN** Agent tab is active
- **THEN** each message is prefixed with "Assistant:"
- **AND** messages are displayed without truncation

#### Scenario: Display Done result distinctly
- **WHEN** the agent completes (StreamEvent::Done received)
- **THEN** the final message is prefixed with "Done:"
- **AND** usage statistics (turns, tokens, cost) are displayed
- **AND** the Done section uses a different color than regular messages

#### Scenario: Message separation
- **WHEN** displaying multiple messages
- **THEN** messages are separated by visual spacing or dividers
- **AND** messages are stacked vertically (newest at bottom)

### Requirement: Scroll snap behavior
The Agent tab SHALL support automatic scrolling to new content when at the bottom.

#### Scenario: Auto-scroll when at bottom
- **WHEN** new content arrives
- **AND** the scroll position is at the bottom of content
- **THEN** the view automatically scrolls to show new content

#### Scenario: Manual scroll disables auto-scroll
- **WHEN** user scrolls up manually
- **THEN** auto-scroll is disabled
- **AND** the view remains at the user's scroll position

#### Scenario: Restore auto-scroll at bottom
- **WHEN** user scrolls back to the bottom of content
- **THEN** auto-scroll is re-enabled
