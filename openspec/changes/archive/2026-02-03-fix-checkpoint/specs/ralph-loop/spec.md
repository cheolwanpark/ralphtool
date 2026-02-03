## MODIFIED Requirements

### Requirement: Loop orchestration
The system SHALL orchestrate autonomous AI development by iterating through stories, spawning a coding agent for each incomplete story with a story-specific prompt, with branch-based checkpoint/revert/retry support on failure.

#### Scenario: Start loop for a change
- **WHEN** user starts the Ralph loop for a change
- **THEN** the system initializes the checkpoint branch via `git checkout -B ralph/{change}`
- **AND** creates an "initial state" commit
- **AND** gets the list of stories from the adapter
- **AND** identifies the first incomplete story
- **AND** generates a story-specific prompt
- **AND** spawns a coding agent for that story

#### Scenario: Story iteration
- **WHEN** an agent completes a story (outputs `<promise>COMPLETE</promise>`)
- **THEN** the system creates a checkpoint commit for that story
- **AND** refreshes the story list
- **AND** spawns an agent for the next incomplete story
- **AND** continues until all stories are complete

#### Scenario: Loop completion
- **WHEN** all stories are complete
- **THEN** the system emits a Complete event with completion options
- **AND** waits for user to select cleanup or keep option

#### Scenario: Checkpoint before agent spawn
- **WHEN** the orchestrator is about to spawn an agent for a story
- **THEN** it verifies the checkpoint branch is active (no explicit save needed - last commit is the checkpoint)

#### Scenario: Retry on failure without COMPLETE signal
- **WHEN** an agent finishes without outputting `<promise>COMPLETE</promise>`
- **AND** retry count is less than max retries
- **THEN** the system executes `git reset --hard HEAD` to revert to last checkpoint
- **AND** increments retry count
- **AND** spawns the agent again for the same story

#### Scenario: Retry with failure reason
- **WHEN** an agent outputs `<promise>FAILED: {reason}</promise>`
- **AND** retry count is less than max retries
- **THEN** the system executes `git reset --hard HEAD` to revert to last checkpoint
- **AND** includes the failure reason in the next prompt
- **AND** spawns the agent again for the same story

#### Scenario: Max retries exceeded
- **WHEN** retry count reaches max retries
- **THEN** the system emits an Error event with the failure details
- **AND** shows completion options (cleanup/keep) instead of stopping abruptly

#### Scenario: Abnormal termination retry
- **WHEN** an agent finishes without any promise signal (COMPLETE or FAILED)
- **AND** retry count is less than max retries
- **THEN** the system executes `git reset --hard HEAD` and retries without additional context

## ADDED Requirements

### Requirement: Completion option handling
The system SHALL handle user-selected completion options after loop finishes.

#### Scenario: Handle cleanup option
- **WHEN** loop completes and user selects "cleanup"
- **THEN** the orchestrator calls checkpoint cleanup with "cleanup" option
- **AND** returns to original branch with uncommitted changes

#### Scenario: Handle keep option
- **WHEN** loop completes and user selects "keep"
- **THEN** the orchestrator calls checkpoint cleanup with "keep" option
- **AND** remains on the ralph/{change} branch

#### Scenario: Completion options on error
- **WHEN** max retries exceeded or loop stops due to error
- **THEN** the system still offers completion options (cleanup/keep)
- **AND** user can recover partial work via their selection
