## Why

The current orchestrator spawns a single agent for the entire change without tracking progress. The agent is told to "complete all stories in order" but there's no iteration, no progress tracking, and no story-specific scenario injection. This makes it hard to resume work, track progress in the UI, and ensure verification is properly scoped to the current story.

## What Changes

- **Story-level iteration**: Orchestrator loops through stories one at a time, spawning agent per story
- **Progress tracking**: Orchestrator tracks which stories are complete, refreshes state after each iteration
- **Story-specific prompts**: Each iteration gets a prompt tailored to the target story with relevant scenarios
- **Completion signal**: Agent must output `<promise>COMPLETE</promise>` only after verification passes
- **SpecAdapter extension**: Add `tool_prompt()` method for spec-tool-specific usage instructions
- **Prompt module move**: `src/spec/prompt.rs` → `src/agent/prompt.rs` (prompt building is agent concern)

## Capabilities

### New Capabilities
- `story-iteration`: Story-level iteration loop with progress tracking and completion signals

### Modified Capabilities
- `ralph-loop`: Now iterates per-story instead of single agent spawn
- `agent-prompt`: Generates story-specific prompts with scenario injection and completion signal instructions
- `spec-adapter`: Add `tool_prompt()` method for spec tool usage instructions

## Impact

- `src/ralph_loop/orchestrator.rs` - Major rewrite for story iteration
- `src/spec/prompt.rs` → `src/agent/prompt.rs` - Move and refactor for story-specific generation
- `src/spec/mod.rs` - Add `tool_prompt()` to SpecAdapter trait
- `src/spec/openspec.rs` - Implement `tool_prompt()` for OpenSpec
- TUI loop screen may need updates for progress display
