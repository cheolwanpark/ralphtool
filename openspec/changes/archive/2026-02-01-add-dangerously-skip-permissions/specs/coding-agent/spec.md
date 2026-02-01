## MODIFIED Requirements

### Requirement: CodingAgent trait
The system SHALL define a CodingAgent trait that abstracts AI coding backends.

#### Scenario: Trait definition
- **WHEN** implementing a new coding agent backend
- **THEN** the implementation provides a `run` method that takes a prompt and config and returns an AgentOutput

#### Scenario: Agent configuration
- **WHEN** spawning an agent
- **THEN** the config specifies allowed_tools, max_turns, timeout, and dangerously_skip_permissions

#### Scenario: Skip permissions configuration
- **WHEN** config.dangerously_skip_permissions is set
- **THEN** the config field defaults to false for safe operation

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

#### Scenario: Skip permissions flag
- **WHEN** config.dangerously_skip_permissions is true
- **THEN** the agent passes `--dangerously-skip-permissions` to Claude CLI

#### Scenario: Safe default for permissions
- **WHEN** config.dangerously_skip_permissions is false or unset
- **THEN** the agent does NOT pass the skip permissions flag

#### Scenario: Parse output
- **WHEN** Claude CLI returns JSON output
- **THEN** the agent extracts result, session_id, and usage from the response
