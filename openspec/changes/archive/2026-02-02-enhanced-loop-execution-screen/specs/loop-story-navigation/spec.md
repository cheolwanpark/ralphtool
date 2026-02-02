## ADDED Requirements

### Requirement: Story navigation in loop screen
The loop execution screen SHALL allow users to navigate between started stories using keyboard controls.

#### Scenario: Navigate to previous story
- **WHEN** user presses left arrow key on the loop screen
- **AND** current selected story is not the first started story
- **THEN** the selected story index decreases by one
- **AND** the display updates to show the newly selected story's content

#### Scenario: Navigate to next story
- **WHEN** user presses right arrow key on the loop screen
- **AND** current selected story is not the last started story
- **THEN** the selected story index increases by one
- **AND** the display updates to show the newly selected story's content

#### Scenario: Cannot navigate to unstarted stories
- **WHEN** user attempts to navigate beyond started stories
- **THEN** the navigation is ignored
- **AND** the selected story remains unchanged

#### Scenario: Auto-select new story on start
- **WHEN** a new story starts (StoryProgress event received)
- **THEN** the story is added to the list of started stories
- **AND** the selected story changes to the newly started story

### Requirement: Story indicator display
The loop execution screen SHALL display a story indicator showing the current position among started stories.

#### Scenario: Display up to five story indicators
- **WHEN** there are 5 or fewer started stories
- **THEN** all story numbers are displayed in the indicator

#### Scenario: Sliding window for many stories
- **WHEN** there are more than 5 started stories
- **THEN** a window of 5 story numbers is displayed
- **AND** the selected story is centered in the window when possible

#### Scenario: Current story visual distinction
- **WHEN** displaying story indicators
- **THEN** the currently executing story (in progress) is shown in green color
- **AND** completed stories are shown in default color
- **AND** the selected story has an underline

### Requirement: Progress bar display
The loop execution screen SHALL display a progress bar showing overall story completion.

#### Scenario: Progress bar with change name
- **WHEN** the loop is running
- **THEN** a progress bar is displayed showing completed/total stories ratio
- **AND** the change name is displayed alongside the progress bar

#### Scenario: Progress updates on story completion
- **WHEN** a story completes
- **THEN** the progress bar updates to reflect the new completion ratio
