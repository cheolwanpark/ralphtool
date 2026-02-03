## 1. Remove Dead Code

- [x] 1.1 Remove `Checkpoint::original_branch()` method from `src/checkpoint/mod.rs`
- [x] 1.2 Remove `Orchestrator::cleanup()`, `original_branch()`, `ralph_branch()` methods from `src/ralph_loop/orchestrator.rs`
- [x] 1.3 Remove `App::confirm_completion_option()` method from `src/app.rs`

## 2. Add MaxRetriesExceeded Event

- [x] 2.1 Add `MaxRetriesExceeded { story_id: String }` variant to `LoopEvent` enum in `src/ralph_loop/mod.rs`
- [x] 2.2 Update orchestrator to emit `MaxRetriesExceeded` event when retry count reaches max_retries (3 locations in `src/ralph_loop/orchestrator.rs`)

## 3. Handle MaxRetriesExceeded in App/Main

- [x] 3.1 Add `max_retries_exceeded_story: Option<String>` field to `App` struct in `src/app.rs`
- [x] 3.2 Handle `LoopEvent::MaxRetriesExceeded` in `process_loop_events()` to store the story_id
- [x] 3.3 Reset the field in `start_loop()` method
- [x] 3.4 Update `main.rs` to check `max_retries_exceeded_story` and use `CompletionReason::MaxRetries` when set

## 4. Verify

- [x] 4.1 Run `cargo build` and confirm no dead_code warnings remain
- [x] 4.2 Run `cargo test` and ensure all tests pass
