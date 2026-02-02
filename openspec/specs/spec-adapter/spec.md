## Purpose

Unified spec adapter trait and types for abstracting spec format implementations.

## Requirements

### Requirement: Unified SpecAdapter trait

The system SHALL provide a single `SpecAdapter` trait that combines all spec operations including tool-specific prompt generation.

#### Scenario: Trait definition
- **WHEN** implementing a spec adapter
- **THEN** one trait provides: `stories()`, `scenarios()`, `context()`, `verify_commands()`, `tool_prompt()`

#### Scenario: Tool prompt method
- **WHEN** calling `adapter.tool_prompt()`
- **THEN** returns a string with spec-tool-specific usage instructions
- **AND** these instructions explain the file format and conventions

### Requirement: OpenSpec tool prompt

The OpenSpecAdapter SHALL provide tool usage instructions specific to OpenSpec.

#### Scenario: OpenSpec file instructions
- **WHEN** calling `tool_prompt()` on OpenSpecAdapter
- **THEN** the prompt includes instructions for:
  - Reading proposal.md, design.md, tasks.md, specs/
  - Marking tasks complete by editing tasks.md (change `[ ]` to `[x]`)
  - Understanding the Given/When/Then scenario format

#### Scenario: Verification command inclusion
- **WHEN** calling `tool_prompt()` on OpenSpecAdapter
- **THEN** the prompt includes verification commands from `verify_commands()`

### Requirement: Story type

A `Story` SHALL represent a numbered section from tasks.md with its child tasks.

#### Scenario: Story structure
- **WHEN** parsing tasks.md
- **THEN** `## N. Title` becomes Story with id "N", title "Title", and tasks Vec

#### Scenario: Story completion check
- **WHEN** checking if a story is complete
- **THEN** `story.is_complete()` returns true if all tasks are done

### Requirement: Task type

A `Task` SHALL represent a checkbox item from tasks.md.

#### Scenario: Task structure
- **WHEN** parsing tasks.md
- **THEN** `- [ ] N.M Description` becomes Task with id "N.M", description, done=false

#### Scenario: Completed task
- **WHEN** parsing tasks.md
- **THEN** `- [x] N.M Description` becomes Task with done=true

### Requirement: Scenario type with story link

A `Scenario` SHALL include a `story_id` field for grouping.

#### Scenario: Scenario structure
- **WHEN** parsing specs
- **THEN** Scenario has: name, story_id, given, when, then

#### Scenario: UI grouping
- **WHEN** displaying scenarios in TUI
- **THEN** group by story_id field

### Requirement: Context type

A `Context` SHALL contain all information needed to work on a story.

#### Scenario: Context contents
- **WHEN** getting context for a story
- **THEN** includes: story, proposal text, design text, scenarios, verify commands

### Requirement: Factory function

The system SHALL provide `create_adapter(change_name)` that returns `Box<dyn SpecAdapter>`.

#### Scenario: Adapter creation
- **WHEN** calling `spec::create_adapter("my-change")`
- **THEN** returns a boxed trait object implementing SpecAdapter

#### Scenario: Future extensibility
- **WHEN** SpecKit support is added
- **THEN** factory can return SpecKitAdapter without changing callers
