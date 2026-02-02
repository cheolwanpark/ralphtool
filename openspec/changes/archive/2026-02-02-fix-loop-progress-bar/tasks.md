## 1. Update Event Structure

- [x] 1.1 Add `completed: usize` field to `LoopEvent::StoryProgress` in `src/ralph_loop/mod.rs`

## 2. Update Orchestrator

- [x] 2.1 Include `completed: state.completed_stories` when emitting `StoryProgress` event in `src/ralph_loop/orchestrator.rs`

## 3. Update App Event Handler

- [x] 3.1 Destructure `completed` field and update `self.loop_state.completed_stories` in `process_loop_events()` in `src/app.rs`

## 4. Update Tests

- [x] 4.1 Update any tests that construct `LoopEvent::StoryProgress` to include the new `completed` field
