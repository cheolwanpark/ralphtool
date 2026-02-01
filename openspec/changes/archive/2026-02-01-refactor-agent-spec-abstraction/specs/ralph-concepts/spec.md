## MODIFIED Requirements

### Requirement: Task domain types

The system SHALL provide domain types representing task hierarchy: `Story` and `Task` with their associated metadata.

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
- **THEN** the adapter SHALL persist the task's completion state to the source system

### Requirement: Story domain types

The system SHALL provide domain types for user stories with acceptance criteria: `UserStory` with priority and pass/fail state.

#### Scenario: Story with acceptance criteria

- **WHEN** a UserStory is represented
- **THEN** it SHALL have an id, title, description, acceptance criteria list, priority, and pass status

### Requirement: StorySource trait

The system SHALL provide a `StorySource` trait for accessing user story data.

#### Scenario: List all stories

- **WHEN** `list_stories()` is called
- **THEN** the adapter SHALL return all user stories with their metadata

#### Scenario: Get next incomplete story

- **WHEN** `next_story()` is called
- **THEN** the adapter SHALL return the highest-priority story where pass=false, or None if all pass

#### Scenario: Mark story passed

- **WHEN** `mark_passed(story_id)` is called
- **THEN** the adapter SHALL update the story's pass status in the source system

### Requirement: Verification domain types

The system SHALL provide domain types for verification scenarios: `Scenario` with Given/When/Then steps.

#### Scenario: Scenario structure

- **WHEN** a Scenario is represented
- **THEN** it SHALL have a name, preconditions (Given), action (When), and expected outcomes (Then)

### Requirement: ScenarioSource trait

The system SHALL provide a `ScenarioSource` trait for accessing verification scenarios.

#### Scenario: Get scenarios for story

- **WHEN** `scenarios_for(story_id)` is called
- **THEN** the adapter SHALL return all verification scenarios associated with that story

#### Scenario: List all scenarios

- **WHEN** `list_scenarios()` is called
- **THEN** the adapter SHALL return all verification scenarios from the source system

### Requirement: No Ralph-specific file creation

The abstraction layer SHALL NOT create Ralph-specific files (prd.json, progress.txt). All persistence SHALL go through adapters to their native formats.

#### Scenario: Write operation delegates to adapter

- **WHEN** any write operation is performed (mark_complete, write_learnings, etc.)
- **THEN** the adapter SHALL write to its source system's format, not create new Ralph files

## REMOVED Requirements

### Requirement: Progress domain types

**Reason**: Learnings and patterns are now owned by agent layer, not spec abstraction
**Migration**: Use agent session state for Learning and Pattern types

### Requirement: ProgressTracker trait

**Reason**: Replaced by SpecWriter trait which agent calls on flush
**Migration**: Use SpecWriter::write_learnings() and SpecWriter::write_patterns() via adapter

## RENAMED Requirements

### Requirement: VerificationSource trait
**FROM**: VerificationSource trait
**TO**: ScenarioSource trait

### Requirement: StoryProvider trait
**FROM**: StoryProvider trait
**TO**: StorySource trait
