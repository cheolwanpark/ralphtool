## ADDED Requirements

### Requirement: Responsive height function

The TUI layout system SHALL provide a `responsive_height` function that calculates content height as 90% of terminal height, clamped to [20, 50] rows.

#### Scenario: responsive_height on 40-row terminal
- **WHEN** `responsive_height(40)` is called
- **THEN** it SHALL return 36

#### Scenario: responsive_height on tall terminal
- **WHEN** `responsive_height(80)` is called
- **THEN** it SHALL return 50 (clamped to maximum)

#### Scenario: responsive_height on short terminal
- **WHEN** `responsive_height(18)` is called
- **THEN** it SHALL return 20 (clamped to minimum)
