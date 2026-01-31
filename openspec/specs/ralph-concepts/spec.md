## Purpose

Provides backend-agnostic traits and domain types for Ralph workflow concepts. Adapters implement these traits to provide task, story, and progress data from their respective spec systems (OpenSpec, SpecKit, etc.).

## Requirements

### Requirement: Task domain types
The system SHALL provide domain types representing Ralph task hierarchy: `Epic`, `Story`, and `Task` with their associated metadata.

#### Scenario: Task with completion state
- **WHEN** a Task is created
- **THEN** it SHALL have an id, description, and completion status (complete/incomplete)

#### Scenario: Story contains tasks
- **WHEN** a Story is represented
- **THEN** it SHALL contain zero or more Tasks and have an id and title

#### Scenario: Epic contains stories
- **WHEN** an Epic is represented
- **THEN** it SHALL contain zero or more Stories and have an id and title

### Requirement: TaskSource trait
The system SHALL provide a `TaskSource` trait that adapters implement to provide task data from their backend.

#### Scenario: List all tasks
- **WHEN** `list_tasks()` is called
- **THEN** the adapter SHALL return all tasks in hierarchical form (Epic > Story > Task)

#### Scenario: Get next incomplete task
- **WHEN** `next_task()` is called
- **THEN** the adapter SHALL return the first incomplete task in priority order, or None if all complete

#### Scenario: Mark task complete
- **WHEN** `mark_complete(task_id)` is called
- **THEN** the adapter SHALL update the task's completion state in the source system's native format

### Requirement: Progress domain types
The system SHALL provide domain types for progress tracking: `Learning` (per-task insight) and `Pattern` (reusable codebase pattern).

#### Scenario: Learning with context
- **WHEN** a Learning is created
- **THEN** it SHALL have a description, optional task reference, and timestamp

#### Scenario: Pattern with description
- **WHEN** a Pattern is created
- **THEN** it SHALL have a name and description

### Requirement: ProgressTracker trait
The system SHALL provide a `ProgressTracker` trait for recording learnings and patterns back to the source system.

#### Scenario: Record a learning
- **WHEN** `record_learning(learning)` is called
- **THEN** the adapter SHALL persist the learning to the source system's native format

#### Scenario: Record a pattern
- **WHEN** `record_pattern(pattern)` is called
- **THEN** the adapter SHALL persist the pattern to the source system's native format

#### Scenario: List patterns
- **WHEN** `list_patterns()` is called
- **THEN** the adapter SHALL return all recorded patterns from the source system

### Requirement: Story domain types
The system SHALL provide domain types for user stories with acceptance criteria: `UserStory` with priority and pass/fail state.

#### Scenario: Story with acceptance criteria
- **WHEN** a UserStory is represented
- **THEN** it SHALL have an id, title, description, acceptance criteria list, priority, and pass status

### Requirement: StoryProvider trait
The system SHALL provide a `StoryProvider` trait for accessing user story data.

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

### Requirement: VerificationSource trait
The system SHALL provide a `VerificationSource` trait for accessing verification scenarios.

#### Scenario: Get scenarios for story
- **WHEN** `scenarios_for(story_id)` is called
- **THEN** the adapter SHALL return all verification scenarios associated with that story

#### Scenario: List all scenarios
- **WHEN** `list_scenarios()` is called
- **THEN** the adapter SHALL return all verification scenarios from the source system

### Requirement: No Ralph-specific file creation
The abstraction layer SHALL NOT create Ralph-specific files (prd.json, progress.txt, tasks.md). All persistence SHALL go through adapters to their native formats.

#### Scenario: Write operation delegates to adapter
- **WHEN** any write operation is performed (mark_complete, record_learning, etc.)
- **THEN** the adapter SHALL write to its source system's format, not create new Ralph files
