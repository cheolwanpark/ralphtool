## Why

When running Ralph in fully automated mode, the Claude CLI prompts for permission on every tool invocation. This blocks autonomous execution and requires manual intervention. Adding the `--dangerously-skip-permissions` flag allows the agent to run without permission prompts in trusted environments.

## What Changes

- Add `dangerously_skip_permissions` boolean field to `AgentConfig`
- Pass `--dangerously-skip-permissions` flag to Claude CLI when enabled
- Default to `false` to maintain safe behavior

## Capabilities

### New Capabilities

None - this is a modification to existing agent configuration.

### Modified Capabilities

- `coding-agent`: Adding a new configuration option to skip permission prompts during autonomous execution

## Impact

- **Code**: `src/agent/mod.rs` (AgentConfig), `src/agent/claude.rs` (CLI invocation)
- **Behavior**: When enabled, agent runs without permission prompts (use with caution)
- **Safety**: Only for trusted, automated environments
