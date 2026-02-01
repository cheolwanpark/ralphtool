## Why

The agent layer currently bypasses the Ralph abstraction layer, writing directly to OpenSpec files (`tasks.md`, `design.md`). This violates layer boundaries and duplicates OpenSpec file format knowledge. Additionally, the "ralph" naming is confusing—the abstraction should be about spec operations, not Ralph branding.

## What Changes

- **BREAKING**: Rename `src/ralph/` to `src/spec/` (module rename)
- **BREAKING**: Remove `ProgressTracker` trait from spec abstraction (learnings/patterns move to agent layer)
- Move learnings and patterns accumulation entirely to agent layer's `SessionState`
- Add `SpecWriter` trait for agent to call on flush (persist learnings/patterns)
- Add `ContextProvider` trait for unified context retrieval
- Agent layer stops writing directly to OpenSpec files—uses abstraction only
- OpenSpec adapter implements persistence via CLI (best effort) or file fallback

## Capabilities

### New Capabilities

- `spec-writer`: Trait for persisting learnings and patterns to the spec system on session flush
- `context-provider`: Trait for retrieving unified work context (story, tasks, proposal, design, scenarios, verify commands)

### Modified Capabilities

- `ralph-concepts`: Rename to spec abstraction, remove ProgressTracker (learnings/patterns move to agent), add SpecWriter and ContextProvider traits
- `openspec-adapter`: Implement new SpecWriter and ContextProvider traits, use CLI-first approach with file fallback
- `agent-session`: Own learnings/patterns buffer, call spec adapter on flush to persist
- `agent-context`: Use adapter's `get_context()` instead of reading files directly
- `agent-tasks`: Use adapter's `mark_complete()` instead of writing tasks.md directly
- `agent-progress`: Buffer learnings/patterns in session only (remove direct file writes)

## Impact

- **Code**: `src/ralph/` → `src/spec/`, all imports updated
- **Traits**: `ProgressTracker` removed from spec layer, `SpecWriter` and `ContextProvider` added
- **Agent module**: All file operations go through spec adapter
- **Tests**: Unit tests for spec layer need updating for new trait structure
