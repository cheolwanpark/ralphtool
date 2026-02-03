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

#### Scenario: Task display with checkbox on separate line
- **WHEN** displaying a task
- **THEN** the checkbox and task ID are displayed on one line (e.g., `  ☐ 5.1`)
- **AND** the task description starts on the next line with indentation (e.g., 4 spaces)
- **AND** the description wraps naturally via `Paragraph::wrap()`

### Requirement: Agent tab content
The Agent tab SHALL display full agent messages with role identification and spacing.

#### Scenario: Display messages with role prefix on separate line
- **WHEN** Agent tab is active
- **THEN** "Assistant:" label is displayed on its own line
- **AND** message content starts on the next line with indentation (e.g., 2 spaces)
- **AND** content wraps naturally via `Paragraph::wrap()`

#### Scenario: Display Done result distinctly
- **WHEN** the agent completes (StreamEvent::Done received)
- **THEN** "Done:" label is displayed on its own line
- **AND** response content starts on the next line with indentation
- **AND** usage statistics (turns, tokens, cost) are displayed on a separate line
- **AND** the Done section uses a different color than regular messages

#### Scenario: Message separation with double spacing
- **WHEN** displaying multiple messages
- **THEN** messages are separated by 2 blank lines
- **AND** messages are stacked vertically (newest at bottom)

#### Scenario: Consecutive blank lines compression
- **WHEN** a message contains multiple consecutive blank lines
- **THEN** consecutive blank lines are compressed to a single blank line
- **AND** non-consecutive blank lines are preserved

### Requirement: Scroll snap behavior
The Agent tab SHALL support automatic scrolling to new content when at the bottom.

#### Scenario: Auto-scroll when at bottom
- **WHEN** new content arrives
- **AND** auto_scroll flag is true
- **THEN** the scroll offset is set to show the newest content at the bottom
- **AND** offset is calculated as total_lines.saturating_sub(visible_height)

#### Scenario: Manual scroll up disables auto-scroll
- **WHEN** user scrolls up (keyboard or mouse)
- **AND** auto_scroll flag is true
- **THEN** loop_agent_scroll is set to current max_scroll value before decrementing
- **AND** loop_agent_scroll is decremented by 1
- **AND** auto_scroll flag is set to false
- **AND** the view scrolls up from the current bottom position

#### Scenario: Manual scroll up when not at auto-scroll
- **WHEN** user scrolls up (keyboard or mouse)
- **AND** auto_scroll flag is false
- **THEN** loop_agent_scroll is decremented by 1 (saturating)
- **AND** auto_scroll flag remains false

#### Scenario: Scroll down when at auto-scroll bottom
- **WHEN** user scrolls down (keyboard or mouse)
- **AND** auto_scroll flag is true
- **THEN** no scroll operation is performed (already at bottom)
- **AND** auto_scroll flag remains true

#### Scenario: Scroll to bottom re-enables auto-scroll
- **WHEN** user scrolls down
- **AND** auto_scroll flag is false
- **AND** scroll position reaches the bottom (offset >= max_scroll)
- **THEN** auto_scroll flag is set to true

#### Scenario: Auto-scroll default state
- **WHEN** a new story starts or loop begins
- **THEN** auto_scroll flag is initialized to true

### Requirement: Scroll reset on story change
Scroll positions SHALL reset when the selected story changes.

#### Scenario: Reset scroll on manual story navigation
- **WHEN** user navigates to a different story (left/right arrow)
- **THEN** loop_info_scroll is reset to 0
- **AND** loop_agent_scroll is reset to 0
- **AND** auto_scroll flag is reset to true

#### Scenario: Reset scroll on auto story selection
- **WHEN** a new story is auto-selected (new story starts)
- **THEN** loop_info_scroll is reset to 0
- **AND** loop_agent_scroll is reset to 0
- **AND** auto_scroll flag is reset to true
