## Purpose

Coding agent abstraction and Claude Code integration for autonomous AI development.

## Requirements

### Requirement: CodingAgent trait
The system SHALL define a CodingAgent trait that abstracts AI coding backends.

#### Scenario: Trait definition
- **WHEN** implementing a new coding agent backend
- **THEN** the implementation provides a `run` method that takes a prompt and config and returns an AgentOutput

#### Scenario: Agent configuration
- **WHEN** spawning an agent
- **THEN** the config specifies allowed_tools, max_turns, and timeout

### Requirement: Claude Code integration
The system SHALL provide a ClaudeAgent implementation of the CodingAgent trait.

#### Scenario: Run with Claude CLI
- **WHEN** ClaudeAgent::run is called with a prompt
- **THEN** it invokes `claude -p <prompt> --output-format json`
- **AND** parses the JSON response into AgentOutput

#### Scenario: Tool restrictions
- **WHEN** config.allowed_tools is specified
- **THEN** the agent passes `--allowedTools <tools>` to Claude CLI

#### Scenario: Turn limit
- **WHEN** config.max_turns is specified
- **THEN** the agent passes `--max-turns <n>` to Claude CLI

#### Scenario: Parse output
- **WHEN** Claude CLI returns JSON output
- **THEN** the agent extracts result, session_id, and usage from the response

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
