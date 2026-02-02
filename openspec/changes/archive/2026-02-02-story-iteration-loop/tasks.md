## 1. SpecAdapter Extension

- [x] 1.1 Add `tool_prompt(&self) -> String` method to SpecAdapter trait in `src/spec/mod.rs`
- [x] 1.2 Implement `tool_prompt()` for OpenSpecAdapter in `src/spec/openspec.rs` returning file format and task marking instructions
- [x] 1.3 Add unit test for `tool_prompt()` output

## 2. Prompt Module Refactor

- [x] 2.1 Create `src/agent/prompt.rs` with `PromptBuilder` struct
- [x] 2.2 Implement `PromptBuilder::for_story(adapter, story_id)` that generates story-specific prompt
- [x] 2.3 Include scenario injection with "focus on relevant scenarios" instruction
- [x] 2.4 Include `<promise>COMPLETE</promise>` completion signal instructions
- [x] 2.5 Use `adapter.tool_prompt()` for spec tool usage instructions
- [x] 2.6 Export `PromptBuilder` from `src/agent/mod.rs`
- [x] 2.7 Remove old `src/spec/prompt.rs` and update `src/spec/mod.rs` exports

## 3. Loop Event Updates

- [x] 3.1 Add `StoryProgress { story_id, story_title, current, total }` variant to LoopEvent
- [x] 3.2 Update LoopState to track current story ID

## 4. Orchestrator Rewrite

- [x] 4.1 Refactor `Orchestrator::run()` to iterate through stories using a loop
- [x] 4.2 Add `next_incomplete_story()` helper that queries adapter and returns first incomplete story
- [x] 4.3 Generate story-specific prompt using `PromptBuilder::for_story()`
- [x] 4.4 Detect `<promise>COMPLETE</promise>` in agent output to mark story iteration done
- [x] 4.5 Refresh story list after each iteration by re-querying adapter
- [x] 4.6 Emit `StoryProgress` event when starting each story
- [x] 4.7 Exit loop when all stories are complete

## 5. Integration and Testing

- [x] 5.1 Update TUI loop screen to display current story progress (optional, if time permits)
- [x] 5.2 Run `cargo check` and `cargo clippy` to verify no warnings
- [x] 5.3 Run `cargo test` to verify all tests pass
