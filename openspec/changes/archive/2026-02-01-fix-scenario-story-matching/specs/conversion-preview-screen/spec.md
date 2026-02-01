## MODIFIED Requirements

### Requirement: Display scenarios grouped by capability

The Scenarios tab SHALL display scenarios organized by their source capability.

#### Scenario: Scenarios tab shows capability groupings

- **WHEN** the Scenarios tab is active
- **THEN** capability names SHALL be displayed as section headers
- **AND** capabilities SHALL be listed in alphabetical order

#### Scenario: Scenarios listed under capability

- **WHEN** a capability section is displayed
- **THEN** all scenarios from that capability SHALL be listed beneath it
- **AND** scenarios SHALL be grouped by their requirement within the capability

#### Scenario: Requirement sub-headers

- **WHEN** scenarios are displayed under a capability
- **THEN** requirement names SHALL appear as sub-headers
- **AND** scenarios belonging to each requirement SHALL be indented under their requirement

#### Scenario: Scenario structure display

- **WHEN** a scenario is displayed under its requirement
- **THEN** the scenario name SHALL be shown
- **AND** GIVEN steps SHALL be listed
- **AND** the WHEN step SHALL be shown
- **AND** THEN steps SHALL be listed

## REMOVED Requirements

### Requirement: Display user stories with nested scenarios

**Reason**: This requirement attempted to nest scenarios under user stories (task stories from tasks.md), but scenarios have no relationship to task stories. Scenarios belong to requirements within capabilities (spec files), not to numbered implementation phases.

**Migration**: The new "Display scenarios grouped by capability" requirement provides correct grouping based on actual data relationships.

### Requirement: Unmatched scenarios section

**Reason**: The "Unmatched Scenarios" section existed because scenario↔story matching always failed (incompatible ID namespaces). With capability-based grouping, all scenarios have a home and no "unmatched" concept is needed.

**Migration**: No migration needed - this was displaying a bug, not a feature.
