## MODIFIED Requirements

### Requirement: Record learning

The system SHALL record learnings via `ralphtool agent learn "<description>"`.

#### Scenario: Record a learning

- **WHEN** agent calls `ralphtool agent learn "Use IF NOT EXISTS for migrations"`
- **THEN** system stores learning in session state's learnings array
- **THEN** system returns confirmation JSON

#### Scenario: Learning with task reference

- **WHEN** agent calls `ralphtool agent learn "Pattern X" --task 2.1`
- **THEN** system stores learning with task ID reference in session state
- **THEN** learning is tagged with current story ID and timestamp

### Requirement: Learnings accumulated in session

The system SHALL accumulate learnings in session state until flush.

#### Scenario: Multiple learnings in iteration

- **WHEN** agent records multiple learnings during iteration
- **THEN** all learnings are accumulated in session state's learnings array
- **THEN** learnings are not written to design.md until session flush

#### Scenario: Learnings flushed via adapter

- **WHEN** orchestrator calls `session flush`
- **THEN** session calls spec adapter's `write_learnings(session.learnings)`
- **THEN** adapter appends learnings to design.md under `## Learnings`

### Requirement: Learnings available in context

The system SHALL include previous learnings in context response.

#### Scenario: Context includes learnings

- **WHEN** agent calls `context`
- **THEN** response includes `learnings` array from session state

#### Scenario: Learnings from current iteration

- **WHEN** agent records learning then calls `context` again
- **THEN** newly recorded learning is included from session state

## ADDED Requirements

### Requirement: Patterns accumulated in session

The system SHALL accumulate patterns in session state until flush.

#### Scenario: Record a pattern

- **WHEN** agent calls `ralphtool agent pattern "Repository pattern" "Use repositories for data access"`
- **THEN** system stores pattern in session state's patterns array
- **THEN** system returns confirmation JSON

#### Scenario: Patterns flushed via adapter

- **WHEN** orchestrator calls `session flush`
- **THEN** session calls spec adapter's `write_patterns(session.patterns)`

### Requirement: No direct file writes for progress

The progress commands SHALL NOT write directly to spec files.

#### Scenario: Learn command buffers only

- **WHEN** agent calls `learn`
- **THEN** system only updates session state
- **THEN** system does NOT write to design.md

#### Scenario: Pattern command buffers only

- **WHEN** agent calls `pattern`
- **THEN** system only updates session state
- **THEN** system does NOT write to specs/

## REMOVED Requirements

### Requirement: Learning format in design.md

**Reason**: Format is now the responsibility of SpecWriter implementation in adapter
**Migration**: See openspec-adapter spec for write_learnings() format
