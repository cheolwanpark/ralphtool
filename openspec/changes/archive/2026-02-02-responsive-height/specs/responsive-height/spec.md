## ADDED Requirements

### Requirement: Percentage-based responsive height calculation

The TUI layout system SHALL calculate content height as a percentage of the available terminal height, with minimum and maximum bounds to ensure usability and readability.

#### Scenario: Standard terminal responsive height
- **WHEN** the terminal height is 40 rows
- **THEN** the content height SHALL be 36 rows (90% of 40)

#### Scenario: Tall terminal capped at maximum
- **WHEN** the terminal height is 80 rows
- **THEN** the content height SHALL be 50 rows (maximum bound)

#### Scenario: Short terminal at minimum
- **WHEN** the terminal height is 20 rows
- **THEN** the content height SHALL be 20 rows (minimum bound)

#### Scenario: Very short terminal uses minimum
- **WHEN** the terminal height is 15 rows
- **THEN** the content height SHALL be 20 rows (minimum bound, may overflow)

### Requirement: Responsive height bounds

The responsive height calculation SHALL enforce minimum height of 20 rows and maximum height of 50 rows.

#### Scenario: Height never below minimum
- **WHEN** the calculated 90% height would be less than 20 rows
- **THEN** the content height SHALL be exactly 20 rows

#### Scenario: Height never above maximum
- **WHEN** the calculated 90% height would exceed 50 rows
- **THEN** the content height SHALL be exactly 50 rows

### Requirement: Vertical centering with responsive height

The content container SHALL be vertically centered within the terminal using the responsive height calculation.

#### Scenario: Centered on standard terminal
- **WHEN** the terminal height is 40 rows and content height is 36 rows
- **THEN** the content SHALL be centered with 2 rows padding on top and bottom

#### Scenario: Centered on tall terminal
- **WHEN** the terminal height is 80 rows and content height is 50 rows
- **THEN** the content SHALL be centered with 15 rows padding on top and bottom
