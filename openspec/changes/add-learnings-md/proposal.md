## Why

Currently, each Story in a Ralph Iteration runs independently without knowledge of what previous Stories learned. When Story 1 discovers important patterns, makes design decisions, or encounters gotchas, Story 2 has no way to know this - it must rediscover everything by reading the code. This leads to redundant exploration and potential inconsistencies.

## What Changes

- Add a shared `learnings.md` file at `/tmp/ralphtool/{change}-learnings.md` that persists across Stories and Iterations
- Include learnings content in the Agent prompt so each Story can see what previous Stories discovered
- Provide guidance to Agent on what to record (discoveries, decisions, gotchas)
- File persists across multiple Iteration runs, enabling cumulative learning

## Capabilities

### New Capabilities
- `learnings-file`: Manages creation, reading, and prompt integration of the shared learnings file

### Modified Capabilities
- `agent-prompt`: Add learnings section to prompt when learnings content exists

## Impact

- `src/ralph_loop/orchestrator.rs`: Ensure learnings file exists at iteration start
- `src/agent/prompt.rs`: Read learnings file and include in prompt generation
- Agent prompts will be slightly longer when learnings exist
