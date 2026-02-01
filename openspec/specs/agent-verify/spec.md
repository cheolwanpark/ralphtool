## Purpose

Verification commands for agent-driven story verification workflow.

## Requirements

### Requirement: Verification context retrieval
The system SHALL provide a `verify context` command that returns all data needed by a verification agent.

#### Scenario: Get verification context
- **WHEN** `ralphtool agent verify context` is called with valid RALPH_SESSION
- **THEN** system returns JSON containing all user stories from specs
- **THEN** system returns JSON containing all scenarios from specs
- **THEN** system returns JSON containing completed tasks from current session
- **THEN** system returns JSON containing proposal and design content
- **THEN** system returns JSON containing verification commands

#### Scenario: Missing session
- **WHEN** `verify context` is called without RALPH_SESSION env
- **THEN** system returns error with code SESSION_REQUIRED

### Requirement: Mark story as verified
The system SHALL provide a `verify pass` command to mark the current story as verified.

#### Scenario: Mark story passed
- **WHEN** `ralphtool agent verify pass` is called with valid RALPH_SESSION and RALPH_STORY
- **THEN** system marks the story as passed via StorySource::mark_passed
- **THEN** system returns JSON with success status

#### Scenario: Missing story scope
- **WHEN** `verify pass` is called without RALPH_STORY env
- **THEN** system returns error with code STORY_REQUIRED
