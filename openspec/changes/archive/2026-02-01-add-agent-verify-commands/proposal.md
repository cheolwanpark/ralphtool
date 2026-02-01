## Why

The Ralph Loop requires separate implementation and verification agents. Currently, the agent CLI provides context for implementation but lacks support for verification agents. Specifically:
- The `context` command returns empty scenarios (ID namespace mismatch bug)
- No way to get all user stories/requirements for verification
- No command to mark a story as verified/passed

## What Changes

- Fix `context` command to return ALL scenarios (not filtered by broken ID mapping)
- Add `verify context` command returning all requirements, scenarios, and completed tasks
- Add `verify pass` command to mark current story as verified
- Expose `UserStory` data via the verification context

## Capabilities

### New Capabilities
- `agent-verify`: Verification context and commands for the verification agent phase of Ralph Loop

### Modified Capabilities
- `agent-context`: Fix scenario retrieval to return all scenarios instead of empty array

## Impact

- `src/agent/` - New verify.rs module, CLI updates
- `src/spec/openspec.rs` - Fix scenarios_for to return all scenarios
- Agent CLI interface - New subcommands under `verify`
