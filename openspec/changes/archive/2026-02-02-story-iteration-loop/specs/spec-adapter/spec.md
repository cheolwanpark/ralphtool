## MODIFIED Requirements

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
