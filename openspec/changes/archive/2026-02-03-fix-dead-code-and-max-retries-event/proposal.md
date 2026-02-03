## Why

Compiler warnings indicate unused code that should be removed, and `CompletionReason::MaxRetries` is defined but never constructed - the orchestrator emits a generic `LoopEvent::Error` on max retries exceeded instead of a dedicated event, causing the UI to show "Success" even when a story failed due to max retries.

## What Changes

- Remove unused methods: `Checkpoint::original_branch()`, `Orchestrator::cleanup()`, `Orchestrator::original_branch()`, `Orchestrator::ralph_branch()`, `App::confirm_completion_option()`
- Add `LoopEvent::MaxRetriesExceeded { story_id }` variant to signal max retries failure
- Update orchestrator to emit `MaxRetriesExceeded` event when retry limit is reached
- Update app/main to handle the new event and use `CompletionReason::MaxRetries`
- Ensure all compiler warnings (dead_code) are resolved

## Capabilities

### New Capabilities

(none - this is cleanup and bug fix for existing capabilities)

### Modified Capabilities

- `ralph-loop`: Add `MaxRetriesExceeded` event variant and emit it when max retries exceeded, connecting the existing retry logic to the completion reason UI

## Impact

- `src/checkpoint/mod.rs`: Remove `original_branch()` method
- `src/ralph_loop/orchestrator.rs`: Remove `cleanup()`, `original_branch()`, `ralph_branch()` methods; emit new event on max retries
- `src/ralph_loop/mod.rs`: Add `MaxRetriesExceeded` variant to `LoopEvent` enum
- `src/app.rs`: Remove `confirm_completion_option()` method; handle new event and track max retries failure
- `src/main.rs`: Use `CompletionReason::MaxRetries` when max retries event was received
