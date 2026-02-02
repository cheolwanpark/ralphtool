## Context

The loop execution screen displays a progress bar showing story completion. The orchestrator correctly calculates `completed_stories` by counting stories where all tasks are done, but this value only exists in the orchestrator's local `LoopState`. The TUI's `loop_state.completed_stories` remains at 0 because the `LoopEvent::StoryProgress` event doesn't include this field.

Current data flow:
```
Orchestrator                           App (TUI)
┌─────────────────────┐               ┌─────────────────────┐
│ state.completed = 2 │──StoryProgress──▶│ loop_state.completed = 0 │
│ state.total = 5     │   {total: 5}  │ loop_state.total = 5     │
└─────────────────────┘               └─────────────────────────┘
                                      ❌ completed never set!
```

## Goals / Non-Goals

**Goals:**
- Progress bar accurately reflects completed story count
- Minimal change to existing event structure

**Non-Goals:**
- Changing the progress calculation logic (it's already correct)
- Adding percentage display (current ratio-based display is fine)
- Real-time task-level progress (story-level granularity is sufficient)

## Decisions

### Decision 1: Add `completed` field to `LoopEvent::StoryProgress`

**Chosen approach**: Extend the existing event with a new `completed: usize` field.

**Rationale**: This is the simplest fix that directly addresses the gap. The orchestrator already calculates the value; we just need to transmit it.

**Alternatives considered**:
- Separate `LoopEvent::CompletionUpdate` event: Adds complexity, requires additional event handling
- TUI calculates completion from story_events: Would duplicate logic and require TUI to understand task structure

### Decision 2: Update at story start only

**Chosen approach**: Send `completed_stories` count when each story starts (in `StoryProgress` event).

**Rationale**: This matches the current emission pattern. Progress updates at story boundaries, which is frequent enough for user perception.

## Risks / Trade-offs

**[Risk] Test updates required** → Low impact; only need to add `completed` field to test event constructions.

**[Trade-off] Progress updates only at story boundaries** → Acceptable; sub-story progress would require significant restructuring with minimal UX benefit.
