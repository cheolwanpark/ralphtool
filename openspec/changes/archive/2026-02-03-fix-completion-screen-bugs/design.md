## Context

The Ralph Loop TUI has two bugs in the completion flow:

1. **Bug 1 - Story count mismatch**: When all stories complete successfully, the completion screen shows "2/3 completed" instead of "3/3 completed". This happens because `completed_stories` is updated at the start of each story iteration, not after the last story completes.

2. **Bug 2 - Cleanup fails silently**: The "Cleanup" option fails because `main.rs` creates a new `Checkpoint` instance that lacks the `original_branch` information (set during `init()`). The orchestrator's checkpoint has this info, but it's not accessible from `main.rs`.

Current architecture:
- Orchestrator runs in background thread, owns the Checkpoint with `original_branch`
- Orchestrator sends `Complete` event and thread ends
- TUI shows completion screen, user selects Cleanup/Keep
- `main.rs` creates NEW Checkpoint (loses `original_branch`) and calls cleanup

## Goals / Non-Goals

**Goals:**
- Fix story count to show correct "3/3 completed" when all stories succeed
- Make Orchestrator responsible for entire cleanup lifecycle
- Ensure Cleanup option actually works (returns to original branch with uncommitted changes)

**Non-Goals:**
- Changing the user-facing completion screen UI
- Modifying checkpoint commit/revert logic
- Adding new completion options beyond Cleanup/Keep

## Decisions

### Decision 1: Update completed_stories when all stories are done

**Choice**: Add `state.completed_stories = state.total_stories` in the `None` branch (all stories complete case) of the story loop.

**Rationale**: Minimal change, fixes the count at the source. The count is updated when starting each story, but not after the final story completes. This one-line fix ensures consistency.

**Alternatives considered**:
- Update in TUI when `CompletionReason::Success`: Would work but feels like a workaround rather than fixing the source

### Decision 2: Orchestrator waits for user choice and executes cleanup

**Choice**: Orchestrator sends `AwaitingUserChoice` event, waits for response via oneshot channel, then executes cleanup before sending `Complete`.

**Rationale**:
- Orchestrator already owns the Checkpoint with `original_branch`
- Keeps cleanup logic encapsulated in one place
- No need to transfer Checkpoint state across thread boundaries

**Alternatives considered**:
- Pass `original_branch` via `LoopEvent::Complete`: Simpler but spreads cleanup logic across files
- Store Orchestrator's Checkpoint in shared state: Complex synchronization

### Decision 3: Use oneshot channel for user choice response

**Choice**: Create a `tokio::sync::oneshot` channel when sending `AwaitingUserChoice`, include sender in the event.

**Rationale**:
- Oneshot is perfect for single-response patterns
- No need for persistent bidirectional channel
- Clean ownership - sender moves into event, receiver stays with Orchestrator

**Implementation flow**:
```
Orchestrator                         TUI (main thread)
    |                                      |
    |-- AwaitingUserChoice(oneshot_tx) -->|
    |   (waits on oneshot_rx)              |
    |                                      |-- Show completion screen
    |                                      |-- User selects option
    |<-- CompletionOption -----------------| (via oneshot_tx.send())
    |                                      |
    |-- checkpoint.cleanup(option)         |
    |-- Complete ------------------------->|
    |   (thread ends)                      |-- Show result screen
```

## Risks / Trade-offs

**[Risk] Orchestrator thread blocks waiting for user input** → The orchestrator already runs in a dedicated thread, so blocking is acceptable. The stop flag is checked before waiting.

**[Risk] User force-quits during AwaitingUserChoice** → The oneshot sender will be dropped, causing `recv()` to return an error. Orchestrator should handle this gracefully (default to Keep or just exit).

**[Trade-off] Increased coupling between TUI and Orchestrator** → Acceptable because cleanup is inherently tied to the orchestrator's checkpoint state. The alternative (spreading logic across files) is worse.
