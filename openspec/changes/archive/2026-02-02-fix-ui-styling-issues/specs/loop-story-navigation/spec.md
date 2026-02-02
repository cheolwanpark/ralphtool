## MODIFIED Requirements

### Requirement: Story indicator display
The loop execution screen SHALL display a story indicator showing the current position among started stories.

#### Scenario: Display up to five story indicators
- **WHEN** there are 5 or fewer started stories
- **THEN** all story numbers are displayed in the indicator

#### Scenario: Sliding window for many stories
- **WHEN** there are more than 5 started stories
- **THEN** a window of 5 story numbers is displayed
- **AND** the selected story is centered in the window when possible
- **AND** no ellipsis indicators are shown

#### Scenario: Current story visual distinction
- **WHEN** displaying story indicators
- **THEN** the currently executing story (in progress) is shown in green color
- **AND** completed stories are shown in default color
- **AND** the selected story has an underline
