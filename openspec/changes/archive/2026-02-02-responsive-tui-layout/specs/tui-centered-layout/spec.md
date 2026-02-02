## MODIFIED Requirements

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

## REMOVED Requirements

### Requirement: Vertical centering with max-height

**Reason**: The max-height parameter was never effectively used - all callers passed the full area height. Vertical centering adds complexity without benefit for this TUI application where content should fill available vertical space.

**Migration**: Remove max_height parameter from centered_rect() calls. Content uses full available height.
