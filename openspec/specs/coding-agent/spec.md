## Purpose

Coding agent abstraction and Claude Code integration for autonomous AI development.

## Requirements

### Requirement: CodingAgent trait
The system SHALL define a CodingAgent trait that abstracts AI coding backends.

#### Scenario: Trait definition
- **WHEN** implementing a new coding agent backend
- **THEN** the implementation provides a `run` method that takes a Prompt and returns an AgentStream

### Requirement: Claude Code integration
The system SHALL provide a ClaudeAgent implementation of the CodingAgent trait.

#### Scenario: Run with Claude CLI
- **WHEN** ClaudeAgent::run is called with a Prompt
- **THEN** it invokes `claude -p <prompt> --output-format stream-json`
- **AND** returns an AgentStream that yields StreamEvent items

#### Scenario: Always skip permissions
- **WHEN** ClaudeAgent::run is called
- **THEN** the agent always passes `--dangerously-skip-permissions` to Claude CLI

### Requirement: Error handling
The system SHALL handle agent errors gracefully.

#### Scenario: Claude CLI not found
- **WHEN** the claude command is not available
- **THEN** the system returns an error with a helpful message

#### Scenario: Agent timeout
- **WHEN** the agent exceeds the configured timeout
- **THEN** the system terminates the process and returns a timeout error

#### Scenario: Invalid output
- **WHEN** the agent returns non-JSON output
- **THEN** the system returns an error with the raw output for debugging
