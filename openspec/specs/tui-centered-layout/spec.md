## Purpose

Provides a centered container layout for the TUI that uses responsive width calculation and centers content horizontally within the terminal. This improves readability on wide terminals and creates consistent visual presentation across all screens.

## Requirements

### Requirement: Centered container with max-width constraint
The TUI SHALL render all content within a centered container using responsive width calculation. The width SHALL be 85% of terminal width, clamped between 60 and 140 columns. When the terminal is wider than the calculated width, the container SHALL be horizontally centered with equal padding on both sides.

#### Scenario: Wide terminal responsive centering
- **WHEN** the terminal width is 150 columns
- **THEN** the content container is 127 columns wide (85% of 150), centered with 11-12 columns of padding on each side

#### Scenario: Very wide terminal maximum width
- **WHEN** the terminal width is 200 columns
- **THEN** the content container is 140 columns wide (maximum), centered with 30 columns of padding on each side

#### Scenario: Standard terminal responsive width
- **WHEN** the terminal width is 100 columns
- **THEN** the content container is 85 columns wide (85% of 100), centered with 7-8 columns of padding on each side

#### Scenario: Narrow terminal minimum width
- **WHEN** the terminal width is 70 columns
- **THEN** the content container is 60 columns wide (minimum), centered with 5 columns of padding on each side

### Requirement: Centering applies to all screens
The centered container layout SHALL be applied consistently to all TUI screens: Change Selection, Preview, Loop Execution, and Result.

#### Scenario: Selection screen is centered
- **WHEN** the Selection screen is rendered on a 150x50 terminal
- **THEN** the content is displayed within a centered responsive-width container

#### Scenario: Preview screen is centered
- **WHEN** the Preview screen is rendered on a 150x50 terminal
- **THEN** the content is displayed within a centered responsive-width container

#### Scenario: Loop screen is centered
- **WHEN** the Loop Execution screen is rendered on a 150x50 terminal
- **THEN** the content is displayed within a centered responsive-width container

#### Scenario: Result screen is centered
- **WHEN** the Result screen is rendered on a 150x50 terminal
- **THEN** the content is displayed within a centered responsive-width container
