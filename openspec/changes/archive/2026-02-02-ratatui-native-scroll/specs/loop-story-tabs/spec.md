## MODIFIED Requirements

### Requirement: Scroll snap behavior
The Agent tab SHALL support automatic scrolling to new content when at the bottom.

#### Scenario: Auto-scroll when at bottom
- **WHEN** new content arrives
- **AND** auto_scroll flag is true
- **THEN** the scroll offset is set to show the newest content at the bottom
- **AND** offset is calculated as total_lines.saturating_sub(visible_height)

#### Scenario: Manual scroll up disables auto-scroll
- **WHEN** user scrolls up (keyboard or mouse)
- **THEN** auto_scroll flag is set to false
- **AND** the view remains at the user's scroll position

#### Scenario: Scroll to bottom re-enables auto-scroll
- **WHEN** user scrolls down
- **AND** scroll position reaches the bottom (offset >= max_scroll)
- **THEN** auto_scroll flag is set to true

#### Scenario: Auto-scroll default state
- **WHEN** a new story starts or loop begins
- **THEN** auto_scroll flag is initialized to true

## ADDED Requirements

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
