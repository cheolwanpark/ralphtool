## MODIFIED Requirements

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

### Requirement: TaskSource trait implementation
The adapter SHALL implement the `TaskSource` trait.

#### Scenario: List tasks returns hierarchy
- **WHEN** `list_tasks()` is called
- **THEN** the adapter SHALL return all tasks organized as Story > Task hierarchy

#### Scenario: Next task returns first incomplete
- **WHEN** `next_task()` is called
- **THEN** the adapter SHALL return the first task where `complete` is false, in document order
- **AND** return None if all tasks are complete

#### Scenario: Mark complete updates state
- **WHEN** `mark_complete(task_id)` is called
- **THEN** the adapter SHALL update the in-memory task state to complete
- **AND** the change SHALL NOT be persisted to disk (preview mode only)

## REMOVED Requirements

### Requirement: Parse epic groups
**Reason**: Epic concept removed. Headings now parsed directly as Story.
**Migration**: `parse_epic_header` becomes `parse_story_header`, Epic struct removed.
