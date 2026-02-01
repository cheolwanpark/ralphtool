## 1. Rename ralph module to spec

- [x] 1.1 Rename `src/ralph/` directory to `src/spec/`
- [x] 1.2 Update `src/ralph/mod.rs` to `src/spec/mod.rs` with updated module docs
- [x] 1.3 Update all imports across codebase (ralph:: → spec::)
- [x] 1.4 Verify build passes with `cargo check`

## 2. Refactor spec abstraction traits

- [x] 2.1 Rename `StoryProvider` trait to `StorySource` in traits.rs
- [x] 2.2 Rename `VerificationSource` trait to `ScenarioSource` in traits.rs
- [x] 2.3 Remove `ProgressTracker` trait from traits.rs
- [x] 2.4 Add `SpecWriter` trait with `write_learnings()` and `write_patterns()` methods
- [x] 2.5 Add `ContextProvider` trait with `get_context()` and `get_status()` methods
- [x] 2.6 Add `WorkContext` and `WorkStatus` types to types.rs
- [x] 2.7 Move `Learning` and `Pattern` types to agent layer (session.rs)

## 3. Update OpenSpec adapter

- [x] 3.1 Update trait implementations to use renamed traits (StorySource, ScenarioSource)
- [x] 3.2 Remove ProgressTracker implementation
- [x] 3.3 Implement `SpecWriter::write_learnings()` with file locking
- [x] 3.4 Implement `SpecWriter::write_patterns()`
- [x] 3.5 Implement `ContextProvider::get_context()` reading proposal/design/tasks
- [x] 3.6 Implement `ContextProvider::get_status()` returning WorkStatus
- [x] 3.7 Update `mark_complete()` to persist changes to tasks.md (not in-memory only)

## 4. Refactor agent session module

- [x] 4.1 Add `Learning` struct to session.rs (moved from ralph/types.rs)
- [x] 4.2 Add `Pattern` struct to session.rs
- [x] 4.3 Add `patterns: Vec<Pattern>` field to SessionState
- [x] 4.4 Update `run_flush()` to call `adapter.write_learnings()`
- [x] 4.5 Update `run_flush()` to call `adapter.write_patterns()`
- [x] 4.6 Remove direct file writes to design.md in session.rs

## 5. Refactor agent context module

- [x] 5.1 Replace direct file reads with `adapter.get_context(story_id)`
- [x] 5.2 Read learnings from session state instead of context response
- [x] 5.3 Remove `get_change_dir()` helper (no longer needed)
- [x] 5.4 Remove `infer_verify_commands()` (moved to adapter)

## 6. Refactor agent tasks module

- [x] 6.1 Replace direct tasks.md writes with `adapter.mark_complete(task_id)`
- [x] 6.2 Replace status logic with `adapter.get_status()`
- [x] 6.3 Remove `get_tasks_path()` helper (no longer needed)
- [x] 6.4 Remove `update_task_in_file()` helper (moved to adapter)

## 7. Add pattern command to agent CLI

- [x] 7.1 Add `PatternArgs` struct to cli.rs
- [x] 7.2 Add `Pattern(PatternArgs)` variant to AgentCommand
- [x] 7.3 Implement pattern command in progress.rs storing to session state
- [x] 7.4 Add `PatternResponse` struct

## 8. Update tests

- [x] 8.1 Update spec layer tests for renamed traits
- [x] 8.2 Add tests for SpecWriter trait implementation
- [x] 8.3 Add tests for ContextProvider trait implementation
- [x] 8.4 Update agent layer tests for new flow
- [x] 8.5 Run full test suite and fix any failures
