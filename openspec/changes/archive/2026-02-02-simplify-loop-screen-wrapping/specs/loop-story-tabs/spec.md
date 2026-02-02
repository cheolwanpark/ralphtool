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

#### Scenario: Task display with checkbox on separate line
- **WHEN** displaying a task
- **THEN** the checkbox and task ID are displayed on one line (e.g., `  ☐ 5.1`)
- **AND** the task description starts on the next line with indentation (e.g., 4 spaces)
- **AND** the description wraps naturally via `Paragraph::wrap()`

### Requirement: Agent tab content
The Agent tab SHALL display full agent messages with role identification and spacing.

#### Scenario: Display messages with role prefix on separate line
- **WHEN** Agent tab is active
- **THEN** "Assistant:" label is displayed on its own line
- **AND** message content starts on the next line with indentation (e.g., 2 spaces)
- **AND** content wraps naturally via `Paragraph::wrap()`

#### Scenario: Display Done result distinctly
- **WHEN** the agent completes (StreamEvent::Done received)
- **THEN** "Done:" label is displayed on its own line
- **AND** response content starts on the next line with indentation
- **AND** usage statistics (turns, tokens, cost) are displayed on a separate line
- **AND** the Done section uses a different color than regular messages

#### Scenario: Message separation with double spacing
- **WHEN** displaying multiple messages
- **THEN** messages are separated by 2 blank lines
- **AND** messages are stacked vertically (newest at bottom)

#### Scenario: Consecutive blank lines compression
- **WHEN** a message contains multiple consecutive blank lines
- **THEN** consecutive blank lines are compressed to a single blank line
- **AND** non-consecutive blank lines are preserved
