## Context

The codebase has compiler warnings for unused code:
1. `Checkpoint::original_branch()` - only called by `Orchestrator::original_branch()` which is itself unused
2. `Orchestrator::cleanup()`, `original_branch()`, `ralph_branch()` - never called; main.rs uses `Checkpoint` directly
3. `App::confirm_completion_option()` - never called
4. `CompletionReason::MaxRetries` - defined but never constructed

The MaxRetries issue is a bug: when max retries is exceeded, orchestrator emits `LoopEvent::Error` and then `LoopEvent::Complete`, causing main.rs to show `CompletionReason::Success`. The UI rendering logic for MaxRetries exists but is never reached.

## Goals / Non-Goals

**Goals:**
- Eliminate all dead_code compiler warnings
- Make `CompletionReason::MaxRetries` work correctly when max retries is exceeded
- Maintain existing CLI `--max-retries` functionality

**Non-Goals:**
- Changing retry logic behavior
- Modifying the completion screen UI layout
- Adding new features

## Decisions

### Decision 1: Add `LoopEvent::MaxRetriesExceeded` variant

**Choice**: Add a new event variant instead of repurposing `LoopEvent::Error`.

**Rationale**:
- Explicit event type makes intent clear
- `Error` events are for logging/display; `MaxRetriesExceeded` triggers completion flow
- Keeps event handling separate: errors can still be logged while MaxRetriesExceeded triggers state transition

**Alternative considered**: Parse error message to detect max retries - rejected as fragile.

### Decision 2: Track max retries failure in App state

**Choice**: Add `max_retries_exceeded_story: Option<String>` field to `App` struct.

**Rationale**:
- Simple field to capture which story exceeded max retries
- Checked in main.rs when determining `CompletionReason`
- Cleared on loop restart

**Alternative considered**: Add completion_reason field directly - rejected as it duplicates existing flow.

### Decision 3: Delete unused code completely

**Choice**: Remove all unused methods entirely rather than marking with `#[allow(dead_code)]`.

**Rationale**:
- Code was added speculatively but never integrated
- Keeping dead code adds maintenance burden
- If needed later, can be re-added with proper integration

## Risks / Trade-offs

**[Risk]** Removing Orchestrator helper methods removes indirection → **Mitigation**: main.rs already uses Checkpoint directly; no functional change.

**[Risk]** Adding new LoopEvent variant requires handling everywhere → **Mitigation**: Only two places handle LoopEvent (app.rs process_loop_events, tests); straightforward addition.
