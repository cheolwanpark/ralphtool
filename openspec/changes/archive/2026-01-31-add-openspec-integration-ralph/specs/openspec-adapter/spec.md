## ADDED Requirements

### Requirement: OpenSpec CLI integration
The adapter SHALL use OpenSpec CLI commands for change enumeration and status checking.

#### Scenario: List available changes
- **WHEN** `list_changes()` is called
- **THEN** the adapter SHALL execute `openspec list --json` and parse the JSON response

#### Scenario: Check change completion status
- **WHEN** `is_complete(change_name)` is called
- **THEN** the adapter SHALL execute `openspec status --change <name> --json` and return true if `isComplete` is true

#### Scenario: CLI not available
- **WHEN** the `openspec` command is not found in PATH
- **THEN** the adapter SHALL return an error with a helpful message indicating OpenSpec CLI is required

### Requirement: Task parsing from tasks.md
The adapter SHALL parse `tasks.md` files to extract task hierarchy.

#### Scenario: Parse epic groups
- **WHEN** parsing a tasks.md file
- **THEN** lines matching `## N. <title>` SHALL be parsed as Epic entries with id "N" and title "<title>"

#### Scenario: Parse tasks with checkbox
- **WHEN** parsing a tasks.md file
- **THEN** lines matching `- [ ] N.M <description>` SHALL be parsed as incomplete Tasks with id "N.M"
- **AND** lines matching `- [x] N.M <description>` SHALL be parsed as complete Tasks

#### Scenario: Group tasks under epics
- **WHEN** tasks are parsed
- **THEN** each task SHALL be associated with the most recent epic heading based on its id prefix

### Requirement: Scenario parsing from specs
The adapter SHALL parse spec files to extract verification scenarios.

#### Scenario: Parse scenario blocks
- **WHEN** parsing a spec.md file
- **THEN** blocks starting with `#### Scenario: <name>` SHALL be parsed as Scenario entries

#### Scenario: Extract Given/When/Then steps
- **WHEN** parsing a scenario block
- **THEN** lines containing `GIVEN` SHALL populate the `given` field
- **AND** lines containing `WHEN` SHALL populate the `when` field
- **AND** lines containing `THEN` or `AND` after THEN SHALL populate the `then` field

### Requirement: UserStory extraction from specs
The adapter SHALL extract UserStory data from spec requirements.

#### Scenario: Parse requirements as stories
- **WHEN** parsing a spec.md file
- **THEN** blocks starting with `### Requirement: <name>` SHALL be parsed as UserStory entries
- **AND** the requirement description SHALL become the story description
- **AND** associated scenarios SHALL become acceptance criteria

### Requirement: TaskSource trait implementation
The adapter SHALL implement the `TaskSource` trait.

#### Scenario: List tasks returns hierarchy
- **WHEN** `list_tasks()` is called
- **THEN** the adapter SHALL return all tasks organized as Epic > Story > Task hierarchy

#### Scenario: Next task returns first incomplete
- **WHEN** `next_task()` is called
- **THEN** the adapter SHALL return the first task where `complete` is false, in document order
- **AND** return None if all tasks are complete

#### Scenario: Mark complete updates state
- **WHEN** `mark_complete(task_id)` is called
- **THEN** the adapter SHALL update the in-memory task state to complete
- **AND** the change SHALL NOT be persisted to disk (preview mode only)

### Requirement: StoryProvider trait implementation
The adapter SHALL implement the `StoryProvider` trait.

#### Scenario: List stories returns all
- **WHEN** `list_stories()` is called
- **THEN** the adapter SHALL return all UserStory entries extracted from specs

#### Scenario: Next story returns first unpassed
- **WHEN** `next_story()` is called
- **THEN** the adapter SHALL return the first story where `passed` is false
- **AND** return None if all stories have passed

#### Scenario: Mark passed updates state
- **WHEN** `mark_passed(story_id)` is called
- **THEN** the adapter SHALL update the in-memory story state to passed

### Requirement: VerificationSource trait implementation
The adapter SHALL implement the `VerificationSource` trait.

#### Scenario: List scenarios returns all
- **WHEN** `list_scenarios()` is called
- **THEN** the adapter SHALL return all Scenario entries extracted from specs

#### Scenario: Scenarios for story filters correctly
- **WHEN** `scenarios_for(story_id)` is called
- **THEN** the adapter SHALL return only scenarios belonging to the specified story/requirement

### Requirement: ProgressTracker trait implementation
The adapter SHALL implement the `ProgressTracker` trait with no-op persistence.

#### Scenario: Record learning is no-op
- **WHEN** `record_learning(learning)` is called
- **THEN** the adapter SHALL return Ok(()) without persisting

#### Scenario: Record pattern is no-op
- **WHEN** `record_pattern(pattern)` is called
- **THEN** the adapter SHALL return Ok(()) without persisting

#### Scenario: List patterns returns empty
- **WHEN** `list_patterns()` is called
- **THEN** the adapter SHALL return an empty vector
