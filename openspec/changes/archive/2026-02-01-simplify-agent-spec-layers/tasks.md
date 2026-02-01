## 1. Error Type Foundation

- [x] 1.1 Create `src/error.rs` with custom Error enum and Result type alias
- [x] 1.2 Implement Display, std::error::Error, and From traits for Error
- [x] 1.3 Add `code()` method returning machine-readable error strings
- [x] 1.4 Update `src/main.rs` to export error module

## 2. Simplified Spec Types

- [x] 2.1 Rewrite `src/spec/types.rs` with simplified Story, Task, Scenario, Context structs
- [x] 2.2 Add `Story::is_complete()` and `Story::next_task()` helper methods
- [x] 2.3 Remove UserStory, Priority, WorkContext, WorkStatus types

## 3. Unified SpecAdapter Trait

- [x] 3.1 Rewrite `src/spec/mod.rs` with single SpecAdapter trait
- [x] 3.2 Add `create_adapter(change_name)` factory function
- [x] 3.3 Remove old trait exports (TaskSource, StorySource, etc.)

## 4. OpenSpec Adapter Update

- [x] 4.1 Update `src/spec/openspec.rs` to implement new SpecAdapter trait
- [x] 4.2 Simplify scenario parsing - store story_id directly in Scenario
- [x] 4.3 Remove UserStory parsing, keep only Story/Task from tasks.md
- [x] 4.4 Update mark_done and append_learnings to use new Error type
- [x] 4.5 Remove scenario_to_story HashMap

## 5. Simplified Session

- [x] 5.1 Rewrite `src/agent/session.rs` with minimal Session struct
- [x] 5.2 Simplify load/save/delete functions using new Error type
- [x] 5.3 Remove SessionLearning, SessionPattern types
- [x] 5.4 Remove pattern-related code

## 6. Agent Commands Consolidation

- [x] 6.1 Update `src/agent/mod.rs` to use factory and trait objects
- [x] 6.2 Consolidate context.rs, tasks.rs, progress.rs logic into mod.rs
- [x] 6.3 Remove verify.rs and VerifyCommand
- [x] 6.4 Update cli.rs to remove Pattern and Verify commands
- [x] 6.5 Delete unused files: context.rs, tasks.rs, progress.rs, verify.rs

## 7. Cleanup and Verification

- [x] 7.1 Run cargo check and fix any compilation errors
- [x] 7.2 Run cargo clippy and address warnings
- [x] 7.3 Run cargo test and fix any failing tests
- [x] 7.4 Verify agent CLI commands still work: init, next-story, context, task done, learn, status, flush
