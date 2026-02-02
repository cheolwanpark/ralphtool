## MODIFIED Requirements

### Requirement: Info tab content
The Info tab SHALL display story details and task list for the selected story.

#### Scenario: Display story information
- **WHEN** Info tab is active
- **THEN** the story title is displayed
- **AND** the story ID is visible

#### Scenario: Display task list with status
- **WHEN** Info tab is active
- **THEN** all tasks for the selected story are listed
- **AND** completed tasks show a checked box (☑)
- **AND** incomplete tasks show an unchecked box (☐)

#### Scenario: Task description line wrapping
- **WHEN** a task description is too long to fit on one line
- **THEN** the description wraps to multiple lines
- **AND** continuation lines are indented to align with the description start position (after `☐ {task.id} `)

### Requirement: Agent tab content
The Agent tab SHALL display full agent messages with role identification and spacing.

#### Scenario: Display messages with role prefix
- **WHEN** Agent tab is active
- **THEN** each message is prefixed with "Assistant:"
- **AND** messages are displayed without truncation

#### Scenario: Display Done result distinctly
- **WHEN** the agent completes (StreamEvent::Done received)
- **THEN** the final message is prefixed with "Done:"
- **AND** usage statistics (turns, tokens, cost) are displayed
- **AND** the Done section uses a different color than regular messages

#### Scenario: Message separation
- **WHEN** displaying multiple messages
- **THEN** messages are separated by visual spacing or dividers
- **AND** messages are stacked vertically (newest at bottom)

#### Scenario: Consecutive blank lines compression
- **WHEN** a message contains multiple consecutive blank lines
- **THEN** consecutive blank lines are compressed to a single blank line
- **AND** non-consecutive blank lines are preserved
