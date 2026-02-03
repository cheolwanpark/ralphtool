## MODIFIED Requirements

### Requirement: Checkpoint save
The system SHALL save the current working directory state by creating a commit on the checkpoint branch, using async-safe command execution.

#### Scenario: Initialize checkpoint branch at loop start
- **WHEN** the orchestrator starts the loop for a change
- **THEN** the system stores the current branch name as `original_branch`
- **AND** executes `git checkout -B ralph/{change_name}` via async command
- **AND** executes `git add -A && git commit --allow-empty -m "initial state"` via async command
- **AND** the operation does not block tokio worker threads

#### Scenario: Save checkpoint after story completion
- **WHEN** an agent completes a story successfully (outputs COMPLETE signal)
- **THEN** the system executes `git add -A && git commit -m "checkpoint: {story_id}"` via async command
- **AND** the commit includes all tracked and untracked file changes
- **AND** the operation does not block tokio worker threads

### Requirement: Checkpoint revert
The system SHALL restore the working directory to the last checkpoint state using git reset, using async-safe command execution.

#### Scenario: Revert to checkpoint on failure
- **WHEN** an agent fails (no COMPLETE signal) and retries are available
- **THEN** the system executes `git reset --hard HEAD` via async command
- **AND** all changes made by the failed agent are discarded
- **AND** the working directory is restored to the last successful checkpoint commit
- **AND** the operation does not block tokio worker threads

### Requirement: Checkpoint cleanup
The system SHALL provide cleanup operations for when the loop completes, using async-safe command execution.

#### Scenario: Cleanup option selected
- **WHEN** user selects "cleanup" after loop completion
- **THEN** the system executes `git checkout {original_branch}` via async command
- **AND** executes `git merge --squash ralph/{change_name}` to bring all changes as staged
- **AND** executes `git reset HEAD` to make changes unstaged
- **AND** executes `git branch -D ralph/{change_name}` to delete the checkpoint branch
- **AND** the working directory contains all changes from the loop as uncommitted modifications

#### Scenario: Keep option selected
- **WHEN** user selects "keep" after loop completion
- **THEN** the system remains on the `ralph/{change_name}` branch
- **AND** all checkpoint commits are preserved
- **AND** no cleanup is performed

#### Scenario: Store original branch
- **WHEN** the checkpoint system initializes
- **THEN** it stores the current branch name for later restoration during cleanup

## REMOVED Requirements

### Requirement: Checkpoint drop
**Reason**: No longer needed - commits persist and don't need explicit dropping like stashes
**Migration**: Story completion now creates a new checkpoint commit instead of dropping a stash

### Requirement: Stash naming convention
**Reason**: Replaced by branch + commit approach - no longer using git stash
**Migration**: Branch is named `ralph/{change_name}`, commits use message "checkpoint: {story_id}"

### Requirement: Stash lookup by name
**Reason**: Replaced by branch + commit approach - no longer need to search stash list
**Migration**: Checkpoints are commits on the ralph branch, accessed via standard git operations
