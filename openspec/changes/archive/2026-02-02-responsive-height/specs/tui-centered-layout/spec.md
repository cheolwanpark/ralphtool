## MODIFIED Requirements

### Requirement: Centered container with max-width constraint
The TUI SHALL render all content within a centered container using responsive width and height calculations. The width SHALL be 85% of terminal width, clamped between 60 and 140 columns. The height SHALL be 90% of terminal height, clamped between 20 and 50 rows. The container SHALL be centered both horizontally and vertically within the terminal.

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

#### Scenario: Tall terminal responsive height centering
- **WHEN** the terminal height is 60 rows
- **THEN** the content container is 50 rows tall (maximum), centered with 5 rows of padding on top and bottom

#### Scenario: Standard terminal responsive height centering
- **WHEN** the terminal height is 40 rows
- **THEN** the content container is 36 rows tall (90% of 40), centered with 2 rows of padding on top and bottom

#### Scenario: Short terminal minimum height
- **WHEN** the terminal height is 24 rows
- **THEN** the content container is 21 rows tall (90% of 24, within bounds), centered with 1-2 rows of padding on top and bottom
