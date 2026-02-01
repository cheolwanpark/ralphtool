## Why

Ralph is designed as an autonomous AI development tool where the agent always runs with full permissions. The current configuration options (`dangerously_skip_permissions` and `allowed_tools`) add unnecessary complexity without providing value—the agent should always skip permission prompts to enable autonomous operation.

## What Changes

- **Remove `dangerously_skip_permissions` field** from `AgentConfig` struct
- **Remove `allowed_tools` field** from `AgentConfig` struct
- **Always pass `--dangerously-skip-permissions`** to Claude CLI (hardcoded behavior)
- **Remove `--allowedTools` flag** from Claude CLI invocation (no tool restrictions)
- **BREAKING**: Simplify `AgentConfig` to only contain `max_turns` and `timeout`

## Capabilities

### New Capabilities

None - this is a simplification change.

### Modified Capabilities

- `coding-agent`: Remove configurable permission/tool settings, always run with full permissions

## Impact

- `src/agent/mod.rs`: Remove `allowed_tools` and `dangerously_skip_permissions` fields from `AgentConfig`
- `src/agent/claude.rs`: Simplify `build_command_args` to always include `--dangerously-skip-permissions` and remove `--allowedTools` logic
- Tests in `src/agent/claude.rs`: Remove tests for conditional flag passing
- `openspec/specs/coding-agent/spec.md`: Update spec to reflect simplified configuration
