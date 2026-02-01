## MODIFIED Requirements

### Requirement: CodingAgent trait
The system SHALL define a CodingAgent trait that abstracts AI coding backends.

#### Scenario: Trait definition
- **WHEN** implementing a new coding agent backend
- **THEN** the implementation provides a `run` method that takes a prompt and config and returns an AgentOutput

#### Scenario: Agent configuration
- **WHEN** spawning an agent
- **THEN** the config specifies max_turns and timeout only

### Requirement: Claude Code integration
The system SHALL provide a ClaudeAgent implementation of the CodingAgent trait.

#### Scenario: Run with Claude CLI
- **WHEN** ClaudeAgent::run is called with a prompt
- **THEN** it invokes `claude -p <prompt> --output-format json`
- **AND** parses the JSON response into AgentOutput

#### Scenario: Turn limit
- **WHEN** config.max_turns is specified
- **THEN** the agent passes `--max-turns <n>` to Claude CLI

#### Scenario: Always skip permissions
- **WHEN** ClaudeAgent::run is called
- **THEN** the agent always passes `--dangerously-skip-permissions` to Claude CLI

#### Scenario: Parse output
- **WHEN** Claude CLI returns JSON output
- **THEN** the agent extracts result, session_id, and usage from the response

## REMOVED Requirements

### Requirement: Skip permissions configuration
**Reason**: Ralph always operates autonomously and requires skip permissions. Configuration is unnecessary.
**Migration**: No migration needed - the flag is now always enabled.

### Requirement: Tool restrictions
**Reason**: Ralph requires full tool access for autonomous operation. Restrictions would break the workflow.
**Migration**: No migration needed - tool restrictions are removed entirely.
