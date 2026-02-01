## Purpose

Provides a trait for persisting learnings and patterns to the spec system. This is a write-only interface that adapters implement to flush accumulated session data to their native format.

## Requirements

### Requirement: SpecWriter trait

The spec abstraction SHALL provide a `SpecWriter` trait for persisting learnings and patterns to the spec system.

#### Scenario: Write learnings

- **WHEN** `write_learnings(learnings)` is called with a list of learnings
- **THEN** the adapter SHALL persist all learnings to the spec system's native format (e.g., design.md)

#### Scenario: Write patterns

- **WHEN** `write_patterns(patterns)` is called with a list of patterns
- **THEN** the adapter SHALL persist all patterns to the spec system's native format

#### Scenario: Empty learnings list

- **WHEN** `write_learnings([])` is called with empty list
- **THEN** the adapter SHALL return Ok(()) without modifying any files

### Requirement: Learning persistence format

The spec writer SHALL persist learnings in a structured markdown format.

#### Scenario: Learnings section creation

- **WHEN** learnings are written and no `## Learnings` section exists in design.md
- **THEN** the adapter SHALL create the section before appending learnings

#### Scenario: Learnings format

- **WHEN** learnings are written
- **THEN** each learning SHALL be formatted as `### <date> - Story <story_id>` with bullet points for each learning
