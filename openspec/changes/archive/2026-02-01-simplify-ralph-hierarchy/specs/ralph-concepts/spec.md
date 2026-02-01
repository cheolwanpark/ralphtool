## MODIFIED Requirements

### Requirement: Task domain types
The system SHALL provide domain types representing Ralph task hierarchy: `Story` and `Task` with their associated metadata.

#### Scenario: Task with completion state
- **WHEN** a Task is created
- **THEN** it SHALL have an id, description, and completion status (complete/incomplete)

#### Scenario: Story contains tasks
- **WHEN** a Story is represented
- **THEN** it SHALL contain zero or more Tasks and have an id and title

### Requirement: TaskSource trait
The system SHALL provide a `TaskSource` trait that adapters implement to provide task data from their backend.

#### Scenario: List all tasks
- **WHEN** `list_tasks()` is called
- **THEN** the adapter SHALL return all tasks in hierarchical form (Story > Task)

#### Scenario: Get next incomplete task
- **WHEN** `next_task()` is called
- **THEN** the adapter SHALL return the first incomplete task in priority order, or None if all complete

#### Scenario: Mark task complete
- **WHEN** `mark_complete(task_id)` is called
- **THEN** the adapter SHALL update the task's completion state in the source system's native format

## REMOVED Requirements

### Requirement: Epic contains stories
**Reason**: Simplified to 2-level hierarchy. Epic was always 1:1 with Story, adding no value.
**Migration**: Use Story as the top-level container instead of Epic.
