## Purpose

Progress tracking for learnings during Ralph Loop iterations.

## Requirements

### Requirement: Record learning

The system SHALL record learnings via `ralphtool agent learn "<description>"`.

#### Scenario: Record a learning
- **WHEN** agent calls `ralphtool agent learn "Use IF NOT EXISTS for migrations"`
- **THEN** system stores learning in session state (not immediately written to files)
- **THEN** system returns confirmation JSON

#### Scenario: Learning with task reference
- **WHEN** agent calls `ralphtool agent learn "Pattern X" --task 2.1`
- **THEN** system stores learning with task ID reference
- **THEN** learning is tagged with current story and iteration

### Requirement: Learnings accumulated in session

The system SHALL accumulate learnings in session state until flush.

#### Scenario: Multiple learnings in iteration
- **WHEN** agent records multiple learnings during iteration
- **THEN** all learnings are accumulated in session state
- **THEN** learnings are not written to design.md until session flush

#### Scenario: Learnings flushed to design.md
- **WHEN** orchestrator calls `session flush`
- **THEN** accumulated learnings are appended to design.md under `## Learnings`
- **THEN** each learning includes timestamp and optional task reference

### Requirement: Learnings available in context

The system SHALL include previous learnings in context response.

#### Scenario: Context includes learnings
- **WHEN** agent calls `context`
- **THEN** response includes `learnings` array from all previous iterations in this session

#### Scenario: Learnings from current iteration
- **WHEN** agent records learning then calls `context` again
- **THEN** newly recorded learning is included in `learnings` array

### Requirement: Learning format in design.md

The system SHALL write learnings in consistent format.

#### Scenario: Learning format
- **WHEN** learnings are flushed to design.md
- **THEN** format is:
  ```
  ### [Date] - Story [ID]
  - [Learning text] (Task [ID] if referenced)
  ```
