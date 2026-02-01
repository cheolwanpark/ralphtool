## MODIFIED Requirements

### Requirement: Context includes scenarios
The system SHALL include all scenarios in the context response for implementation reference.

#### Scenario: Context returns all scenarios
- **WHEN** `ralphtool agent context` is called
- **THEN** system returns all scenarios from the change's specs directory
- **THEN** scenarios are not filtered by story ID
