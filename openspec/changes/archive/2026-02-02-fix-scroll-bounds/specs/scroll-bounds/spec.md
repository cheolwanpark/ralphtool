## ADDED Requirements

### Requirement: Scroll position clamped to content bounds

The system SHALL clamp scroll position to valid content bounds at render time. The maximum scroll position SHALL be calculated as `total_lines - visible_height`, where `total_lines` accounts for text wrapping at the current viewport width.

#### Scenario: Scroll at content end
- **WHEN** user scrolls down while already at the last line of content
- **THEN** scroll position does not increase and content remains at the same position

#### Scenario: Content shorter than viewport
- **WHEN** content has fewer lines than the viewport height
- **THEN** max scroll is 0 and content cannot be scrolled

#### Scenario: Window resize shrinks content
- **WHEN** window is resized causing current scroll position to exceed new max scroll
- **THEN** scroll position is clamped to new max scroll on next render

### Requirement: Consistent scroll clamping across all scrollable screens

The system SHALL apply scroll clamping to all screens with scrollable Paragraph content:
- Preview screen: Tasks tab and Scenarios tab
- Loop screen: Info tab (Agent tab already has clamping)
- Result screen: Tasks tab and Changed Files tab

#### Scenario: Preview Tasks tab scroll bounds
- **WHEN** user scrolls in Preview Tasks tab
- **THEN** scroll cannot exceed content bounds

#### Scenario: Preview Scenarios tab scroll bounds
- **WHEN** user scrolls in Preview Scenarios tab
- **THEN** scroll cannot exceed content bounds

#### Scenario: Loop Info tab scroll bounds
- **WHEN** user scrolls in Loop Info tab
- **THEN** scroll cannot exceed content bounds

#### Scenario: Result Tasks tab scroll bounds
- **WHEN** user scrolls in Result Tasks tab
- **THEN** scroll cannot exceed content bounds

#### Scenario: Result Changed Files tab scroll bounds
- **WHEN** user scrolls in Result Changed Files tab
- **THEN** scroll cannot exceed content bounds
