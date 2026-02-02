## Purpose

Provides percentage-based responsive width and height calculations for TUI layout, ensuring content dimensions adapt to terminal size while maintaining usability and readability through minimum and maximum bounds.

## Requirements

### Requirement: Percentage-based responsive width calculation

The TUI layout system SHALL calculate content width as a percentage of the available terminal width, with minimum and maximum bounds to ensure usability and readability.

#### Scenario: Standard terminal responsive width
- **WHEN** the terminal width is 120 columns
- **THEN** the content width SHALL be 102 columns (85% of 120)

#### Scenario: Wide terminal capped at maximum
- **WHEN** the terminal width is 200 columns
- **THEN** the content width SHALL be 140 columns (maximum bound)

#### Scenario: Narrow terminal at minimum
- **WHEN** the terminal width is 60 columns
- **THEN** the content width SHALL be 60 columns (minimum bound)

#### Scenario: Very narrow terminal uses minimum
- **WHEN** the terminal width is 50 columns
- **THEN** the content width SHALL be 60 columns (minimum bound, may overflow)

### Requirement: Responsive width bounds

The responsive width calculation SHALL enforce minimum width of 60 columns and maximum width of 140 columns.

#### Scenario: Width never below minimum
- **WHEN** the calculated 85% width would be less than 60 columns
- **THEN** the content width SHALL be exactly 60 columns

#### Scenario: Width never above maximum
- **WHEN** the calculated 85% width would exceed 140 columns
- **THEN** the content width SHALL be exactly 140 columns

### Requirement: Horizontal centering with responsive width

The content container SHALL be horizontally centered within the terminal using the responsive width calculation.

#### Scenario: Centered on standard terminal
- **WHEN** the terminal width is 120 columns and content width is 102 columns
- **THEN** the content SHALL be centered with 9 columns padding on each side

#### Scenario: Centered on wide terminal
- **WHEN** the terminal width is 200 columns and content width is 140 columns
- **THEN** the content SHALL be centered with 30 columns padding on each side

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
