## Why

The current agent/spec layer architecture has grown complex with 5 traits, 2 parallel type hierarchies (Story/UserStory), duplicate response types, and a separate verification phase. This adds cognitive overhead and maintenance burden without providing proportional value. Simplification is needed to make the codebase more maintainable and easier to extend for future spec systems (SpecKit).

## What Changes

- **BREAKING**: Consolidate 5 traits (`TaskSource`, `StorySource`, `ScenarioSource`, `SpecWriter`, `ContextProvider`) into 1 unified `SpecAdapter` trait
- **BREAKING**: Remove `UserStory` type - keep only `Story` with `Task` children
- **BREAKING**: Remove separate verification phase (`VerifyCommand`, `verify.rs`) - AI outputs `<promise>VERIFIED</promise>` token instead
- **BREAKING**: Replace `anyhow::Error` with custom `ralphtool::Error` enum
- Remove `Pattern` type - keep only `Learning` (simple strings)
- Simplify `Session` state - remove complex buffering, just `{id, change, story_id, learnings[]}`
- Remove duplicate types (`WorkContext`/`ContextResponse`, `StoryStatus`/`StoryContext`)
- Agent layer uses trait objects only - no direct `OpenSpecAdapter` imports
- Add factory function `spec::create_adapter(change) -> Box<dyn SpecAdapter>`
- Keep `Scenario` flat in storage, group by `story_id` in UI layer

## Capabilities

### New Capabilities

- `error-handling`: Custom `ralphtool::Error` enum with machine-readable error codes for agent CLI

### Modified Capabilities

- `agent-cli`: Simplified command set, uses spec layer abstraction
- `spec-adapter`: Single unified trait replacing 5 separate traits

## Impact

- `src/agent/` - Consolidate session.rs, context.rs, tasks.rs, progress.rs, verify.rs into fewer files
- `src/spec/` - Rewrite traits.rs to single trait, simplify types.rs, update openspec.rs
- `src/error.rs` - New file for custom error type
- Agent CLI interface unchanged (backwards compatible commands)
- Estimated ~400 lines reduction
