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
