## MODIFIED Requirements

### Requirement: Unified SpecAdapter trait

The system SHALL provide a single `SpecAdapter` trait that combines read-only spec operations.

#### Scenario: Trait definition
- **WHEN** implementing a spec adapter
- **THEN** one trait provides: `stories()`, `scenarios()`, `context()`, `verify_commands()`

## REMOVED Requirements

### Requirement: mark_done method
**Reason**: Agent edits tasks.md directly instead of calling a method. This removes the need for file locking code and the fs2 dependency.
**Migration**: Agent edits tasks.md directly, changing `[ ]` to `[x]`.

### Requirement: append_learnings method
**Reason**: Learnings feature was rarely used. Agent can add notes directly to design.md if needed.
**Migration**: None - feature removed. Agent can edit design.md directly if learnings are needed.
