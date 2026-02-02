## Purpose

Provides a centered container layout for the TUI that constrains content width and centers it both horizontally and vertically within the terminal. This improves readability on wide terminals and creates consistent visual presentation across all screens.

## Requirements

### Requirement: Centered container with max-width constraint
The TUI SHALL render all content within a centered container that does not exceed 100 columns in width. When the terminal is wider than 100 columns, the container SHALL be horizontally centered with equal padding on both sides.

#### Scenario: Wide terminal centering
- **WHEN** the terminal width is 150 columns
- **THEN** the content container is 100 columns wide, centered with 25 columns of padding on each side

#### Scenario: Narrow terminal no padding
- **WHEN** the terminal width is 80 columns
- **THEN** the content container uses the full 80 columns with no horizontal padding

### Requirement: Vertical centering with max-height
The TUI SHALL vertically center the content container when the terminal height exceeds the maximum content height. The maximum content height SHALL be 40 lines or the actual content height, whichever is smaller.

#### Scenario: Tall terminal vertical centering
- **WHEN** the terminal height is 50 lines and content requires 40 lines
- **THEN** the content container is vertically centered with 5 lines of padding at top and bottom

#### Scenario: Short terminal no vertical padding
- **WHEN** the terminal height is 30 lines
- **THEN** the content container uses the full 30 lines with no vertical padding

### Requirement: Centering applies to all screens
The centered container layout SHALL be applied consistently to all TUI screens: Change Selection, Preview, Loop Execution, and Result.

#### Scenario: Selection screen is centered
- **WHEN** the Selection screen is rendered on a 150x50 terminal
- **THEN** the content is displayed within a centered 100-column container

#### Scenario: Preview screen is centered
- **WHEN** the Preview screen is rendered on a 150x50 terminal
- **THEN** the content is displayed within a centered 100-column container

#### Scenario: Loop screen is centered
- **WHEN** the Loop Execution screen is rendered on a 150x50 terminal
- **THEN** the content is displayed within a centered 100-column container

#### Scenario: Result screen is centered
- **WHEN** the Result screen is rendered on a 150x50 terminal
- **THEN** the content is displayed within a centered 100-column container
