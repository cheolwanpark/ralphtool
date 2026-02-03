## Purpose

Loop orchestration for autonomous AI development. Iterates through stories, spawning a coding agent for each incomplete story with a story-specific prompt.

## Requirements

### Requirement: Loop orchestration
The system SHALL orchestrate autonomous AI development by iterating through stories, spawning a coding agent for each incomplete story with a story-specific prompt, with checkpoint/revert/retry support on failure.

#### Scenario: Start loop for a change
- **WHEN** user starts the Ralph loop for a change
- **THEN** the system gets the list of stories from the adapter
- **AND** identifies the first incomplete story
- **AND** generates a story-specific prompt
- **AND** spawns a coding agent for that story

#### Scenario: Story iteration
- **WHEN** an agent completes a story (outputs `<promise>COMPLETE</promise>`)
- **THEN** the system drops the checkpoint for that story
- **AND** refreshes the story list
- **AND** spawns an agent for the next incomplete story
- **AND** continues until all stories are complete

#### Scenario: Loop completion
- **WHEN** all stories are complete
- **THEN** the system cleans up any remaining checkpoints
- **AND** emits a Complete event

#### Scenario: Checkpoint before agent spawn
- **WHEN** the orchestrator is about to spawn an agent for a story
- **THEN** it saves a checkpoint of the current working directory state

#### Scenario: Retry on failure without COMPLETE signal
- **WHEN** an agent finishes without outputting `<promise>COMPLETE</promise>`
- **AND** retry count is less than max retries
- **THEN** the system reverts to the checkpoint
- **AND** increments retry count
- **AND** spawns the agent again for the same story

#### Scenario: Retry with failure reason
- **WHEN** an agent outputs `<promise>FAILED: {reason}</promise>`
- **AND** retry count is less than max retries
- **THEN** the system reverts to the checkpoint
- **AND** includes the failure reason in the next prompt
- **AND** spawns the agent again for the same story

#### Scenario: Max retries exceeded
- **WHEN** retry count reaches max retries
- **THEN** the system emits an Error event with the failure details
- **AND** stops the loop (does not continue to next story)

#### Scenario: Abnormal termination retry
- **WHEN** an agent finishes without any promise signal (COMPLETE or FAILED)
- **AND** retry count is less than max retries
- **THEN** the system reverts and retries without additional context

### Requirement: Loop events
The system SHALL emit events during loop execution to enable TUI updates, including story progress information and full agent output with story association.

#### Scenario: Event types
- **WHEN** the loop is running
- **THEN** the system emits StoryEvent, StoryProgress, Error, and Complete events

#### Scenario: StoryEvent with full message
- **WHEN** the agent emits a StreamEvent::Message
- **THEN** the orchestrator emits a StoryEvent containing the story_id and the full StreamEvent
- **AND** the message content is not truncated

#### Scenario: StoryEvent with Done response
- **WHEN** the agent emits a StreamEvent::Done
- **THEN** the orchestrator emits a StoryEvent containing the story_id and the full StreamEvent
- **AND** the Response includes content, turns, tokens, and cost

#### Scenario: Story progress event
- **WHEN** the orchestrator starts working on a story
- **THEN** it emits a StoryProgress event with current story ID and total count

#### Scenario: TUI subscription
- **WHEN** the TUI is displaying the loop screen
- **THEN** it receives events via a channel and updates the display accordingly
- **AND** stores messages per-story for navigation
- **AND** shows the current story being worked on

### Requirement: Loop control
The system SHALL allow the user to stop the loop, including a force-quit mechanism when graceful shutdown fails.

#### Scenario: User stops loop
- **WHEN** user presses 'q' during loop execution
- **THEN** the system signals the agent to stop
- **AND** preserves any completed work

#### Scenario: Force-quit on repeated q presses
- **WHEN** user presses 'q' three times within 3 seconds
- **THEN** the system SHALL force-quit immediately
- **AND** cleanup SHALL be attempted but not block exit
- **AND** a warning message SHALL be displayed indicating force-quit was triggered

#### Scenario: Graceful stop timeout
- **WHEN** graceful stop is requested but the orchestrator does not respond within 5 seconds
- **THEN** the TUI SHALL display a message suggesting force-quit (press q twice more)

### Requirement: TUI loop integration
The TUI SHALL spawn the orchestrator when the user starts the loop and display agent output with story-based navigation.

#### Scenario: Loop startup
- **WHEN** user presses 'R' to start the loop from the preview screen
- **THEN** the system spawns the agent with the generated prompt
- **AND** displays the loop screen with story navigation and tabbed content

#### Scenario: Stop signal propagation
- **WHEN** user presses 'q' on the loop screen
- **THEN** the TUI signals the agent to stop
- **AND** transitions back to selection screen after agent exits

### Requirement: Configurable max retries
The system SHALL allow configuration of maximum retry attempts via CLI flag.

#### Scenario: CLI flag for max retries
- **WHEN** user starts the loop with `--max-retries 5`
- **THEN** the orchestrator uses 5 as the maximum retry count per story

#### Scenario: Default max retries
- **WHEN** user starts the loop without `--max-retries` flag
- **THEN** the orchestrator uses 3 as the default maximum retry count

### Requirement: Async-safe orchestration
The orchestrator SHALL execute all blocking operations in an async-safe manner.

#### Scenario: Adapter refresh is non-blocking
- **WHEN** the orchestrator refreshes the adapter between stories
- **THEN** the adapter creation SHALL use async command execution
- **AND** other async tasks (like event forwarding) SHALL continue to make progress

#### Scenario: Checkpoint operations are non-blocking
- **WHEN** the orchestrator saves, reverts, or drops checkpoints
- **THEN** the checkpoint operations SHALL use async command execution
- **AND** the stop flag SHALL remain checkable during these operations

### Requirement: Operation timeouts
The orchestrator SHALL apply timeouts to external operations to prevent indefinite hangs.

#### Scenario: Default command timeout
- **WHEN** external commands (git, openspec) are executed
- **THEN** a default timeout of 30 seconds SHALL be applied

#### Scenario: Timeout triggers retry
- **WHEN** a command times out during story execution
- **THEN** the timeout SHALL be treated as a failure
- **AND** the retry mechanism SHALL be triggered if retries are available

#### Scenario: Configurable timeout
- **WHEN** user specifies `--command-timeout <seconds>` CLI flag
- **THEN** the specified timeout SHALL be used for all external commands
