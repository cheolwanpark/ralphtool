## MODIFIED Requirements

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

## ADDED Requirements

### Requirement: Configurable max retries
The system SHALL allow configuration of maximum retry attempts via CLI flag.

#### Scenario: CLI flag for max retries
- **WHEN** user starts the loop with `--max-retries 5`
- **THEN** the orchestrator uses 5 as the maximum retry count per story

#### Scenario: Default max retries
- **WHEN** user starts the loop without `--max-retries` flag
- **THEN** the orchestrator uses 3 as the default maximum retry count
