## Purpose

Defines the OpenSpec adapter that implements Ralph traits by reading OpenSpec change data (tasks.md, specs/*.md) and providing it as Ralph domain types.

## Requirements

### Requirement: OpenSpec CLI integration

The adapter SHALL use OpenSpec CLI commands for change enumeration and status checking, using async-safe command execution.

#### Scenario: List available changes

- **WHEN** `list_changes()` is called
- **THEN** the adapter SHALL execute `openspec list --json` via `async_cmd::run()` and parse the JSON response
- **AND** the operation does not block tokio worker threads

#### Scenario: Check change completion status

- **WHEN** `is_complete(change_name)` is called
- **THEN** the adapter SHALL execute `openspec status --change <name> --json` via `async_cmd::run()` and return true if `isComplete` is true
- **AND** the operation does not block tokio worker threads

#### Scenario: CLI not available

- **WHEN** the `openspec` command is not found in PATH
- **THEN** the adapter SHALL return an error with a helpful message indicating OpenSpec CLI is required

#### Scenario: CLI command timeout

- **WHEN** an openspec CLI command does not respond within the timeout period
- **THEN** the adapter SHALL return a timeout error
- **AND** the operation SHALL not block other async tasks indefinitely

### Requirement: Task parsing from tasks.md

The adapter SHALL parse `tasks.md` files to extract task hierarchy.

#### Scenario: Parse story groups

- **WHEN** parsing a tasks.md file
- **THEN** lines matching `## N. <title>` SHALL be parsed as Story entries with id "N" and title "<title>"

#### Scenario: Parse tasks with checkbox

- **WHEN** parsing a tasks.md file
- **THEN** lines matching `- [ ] N.M <description>` SHALL be parsed as incomplete Tasks with id "N.M"
- **AND** lines matching `- [x] N.M <description>` SHALL be parsed as complete Tasks

#### Scenario: Group tasks under stories

- **WHEN** tasks are parsed
- **THEN** each task SHALL be associated with the most recent story heading based on its id prefix

### Requirement: Scenario parsing from specs

The adapter SHALL parse spec files to extract verification scenarios with capability tracking.

#### Scenario: Parse scenario blocks

- **WHEN** parsing a spec.md file in `specs/<capability>/`
- **THEN** blocks starting with `#### Scenario: <name>` SHALL be parsed as Scenario entries
- **AND** the `capability` field SHALL be set to the spec folder name (e.g., `"ralph-loop"`)

#### Scenario: Extract Given/When/Then steps

- **WHEN** parsing a scenario block
- **THEN** lines containing `GIVEN` SHALL populate the `given` field
- **AND** lines containing `WHEN` SHALL populate the `when` field
- **AND** lines containing `THEN` or `AND` after THEN SHALL populate the `then` field

#### Scenario: Derive requirement ID from requirement name

- **WHEN** parsing a scenario under a requirement heading
- **THEN** the `requirement_id` field SHALL be set to the slugified requirement name
- **AND** the slugification SHALL convert to lowercase and replace spaces with hyphens

### Requirement: UserStory extraction from specs

The adapter SHALL extract UserStory data from spec requirements.

#### Scenario: Parse requirements as stories

- **WHEN** parsing a spec.md file
- **THEN** blocks starting with `### Requirement: <name>` SHALL be parsed as UserStory entries
- **AND** the requirement description SHALL become the story description
- **AND** associated scenarios SHALL become acceptance criteria

### Requirement: TaskSource trait implementation

The adapter SHALL implement the `TaskSource` trait with persistence.

#### Scenario: List tasks returns hierarchy

- **WHEN** `list_tasks()` is called
- **THEN** the adapter SHALL return all tasks organized as Story > Task hierarchy

#### Scenario: Next task returns first incomplete

- **WHEN** `next_task()` is called
- **THEN** the adapter SHALL return the first task where `complete` is false, in document order
- **AND** return None if all tasks are complete

#### Scenario: Mark complete persists to tasks.md

- **WHEN** `mark_complete(task_id)` is called
- **THEN** the adapter SHALL update tasks.md, changing `- [ ] <id>` to `- [x] <id>`
- **AND** the adapter SHALL use file locking for atomic updates

### Requirement: StorySource trait implementation

The adapter SHALL implement the `StorySource` trait.

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

### Requirement: ScenarioSource trait implementation

The adapter SHALL implement the `ScenarioSource` trait.

#### Scenario: List scenarios returns all

- **WHEN** `list_scenarios()` is called
- **THEN** the adapter SHALL return all Scenario entries extracted from specs
- **AND** each scenario SHALL include `capability` and `requirement_id` fields

#### Scenario: Scenarios for capability filters correctly

- **WHEN** `scenarios_for_capability(capability)` is called
- **THEN** the adapter SHALL return only scenarios where `capability` matches the specified value

### Requirement: SpecWriter trait implementation

The adapter SHALL implement the `SpecWriter` trait for persisting learnings and patterns.

#### Scenario: Write learnings to design.md

- **WHEN** `write_learnings(learnings)` is called
- **THEN** the adapter SHALL append learnings to design.md under `## Learnings` section
- **AND** create the section if it does not exist

#### Scenario: Write patterns to design.md

- **WHEN** `write_patterns(patterns)` is called
- **THEN** the adapter SHALL append patterns to design.md under `## Patterns` section

### Requirement: ContextProvider trait implementation

The adapter SHALL implement the `ContextProvider` trait for context retrieval.

#### Scenario: Get context assembles from files

- **WHEN** `get_context(story_id)` is called
- **THEN** the adapter SHALL read proposal.md and design.md content
- **AND** the adapter SHALL return story, tasks, scenarios filtered to the story
- **AND** the adapter SHALL infer verify commands from project type

#### Scenario: Get status from task state

- **WHEN** `get_status()` is called
- **THEN** the adapter SHALL return task/story completion counts from parsed tasks.md
