## ADDED Requirements

### Requirement: Native scroll with Paragraph::scroll()
All scrollable content SHALL use Ratatui's native `Paragraph::scroll((y, x))` method instead of manual line skipping.

#### Scenario: Preview screen uses native scroll
- **WHEN** rendering Preview screen content (Tasks or Scenarios tab)
- **THEN** the Paragraph widget uses `.scroll((offset, 0))` for vertical scrolling
- **AND** manual `skip().collect()` is NOT used

#### Scenario: Loop Info tab uses native scroll
- **WHEN** rendering Loop Execution Info tab content
- **THEN** the Paragraph widget uses `.scroll((offset, 0))` for vertical scrolling
- **AND** manual `skip().collect()` is NOT used

#### Scenario: Loop Agent tab uses native scroll
- **WHEN** rendering Loop Execution Agent tab content
- **THEN** the Paragraph widget uses `.scroll((offset, 0))` for vertical scrolling
- **AND** manual `skip().collect()` is NOT used

#### Scenario: Result screen uses native scroll
- **WHEN** rendering Result screen content (Tasks or Changed Files tab)
- **THEN** the Paragraph widget uses `.scroll((offset, 0))` for vertical scrolling
- **AND** the List widget is NOT used for scrollable content

### Requirement: Scroll bounds calculation with line_count()
Scroll position calculations SHALL use `Paragraph::line_count(width)` to account for wrapped lines.

#### Scenario: Calculate rendered line count
- **WHEN** determining scroll bounds for content with Wrap enabled
- **THEN** `Paragraph::line_count(area.width)` is used to get actual rendered line count
- **AND** the count reflects lines after wrap is applied

#### Scenario: Maximum scroll calculation
- **WHEN** calculating maximum scroll position
- **THEN** max_scroll = total_lines.saturating_sub(visible_height)
- **AND** scroll offset is clamped to this maximum
