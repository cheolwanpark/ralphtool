## MODIFIED Requirements

### Requirement: Mark task complete

The system SHALL mark a task as complete via `ralphtool agent task done <task-id>`.

#### Scenario: Complete a task

- **WHEN** agent calls `ralphtool agent task done 2.1`
- **THEN** system validates task 2.1 exists in current story scope
- **THEN** system calls spec adapter's `mark_complete("2.1")`
- **THEN** system returns JSON with remaining tasks and story completion status

#### Scenario: Task not in current story

- **WHEN** agent calls `task done 3.1` but RALPH_STORY=2
- **THEN** system returns error indicating task is not in current story scope

#### Scenario: Task already complete

- **WHEN** agent calls `task done` for already completed task
- **THEN** system returns success (idempotent operation)

#### Scenario: Invalid task ID

- **WHEN** agent calls `task done` with non-existent task ID
- **THEN** system returns error indicating task not found

### Requirement: Story completion detection

The system SHALL detect when all tasks in current story are complete.

#### Scenario: Last task completed

- **WHEN** agent completes the last remaining task in story
- **THEN** response includes `story_complete: true`
- **THEN** response includes list of scenarios to verify

#### Scenario: Tasks remaining

- **WHEN** agent completes a task but others remain
- **THEN** response includes `story_complete: false`
- **THEN** response includes `remaining` array with incomplete task IDs

### Requirement: Task status query

The system SHALL provide current task status via `ralphtool agent status`.

#### Scenario: Get status

- **WHEN** agent calls `ralphtool agent status`
- **THEN** system calls spec adapter's `get_status()`
- **THEN** system returns JSON with:
  - `story`: { id, tasks_done, tasks_total }
  - `change`: { stories_done, stories_total }
  - `story_complete`: boolean
  - `change_complete`: boolean

### Requirement: Atomic task updates

The system SHALL ensure task updates are atomic and consistent.

#### Scenario: Concurrent task updates

- **WHEN** task update is in progress
- **THEN** spec adapter uses file locking to prevent corruption
- **THEN** update is atomic (no partial writes)

## ADDED Requirements

### Requirement: Task completion through adapter

The task command SHALL use the spec adapter's TaskSource trait, not write files directly.

#### Scenario: No direct file writes

- **WHEN** agent calls `task done`
- **THEN** system calls `adapter.mark_complete(task_id)` to update tasks.md
- **THEN** system does NOT write to tasks.md directly
