## Context

The result screen (`result_screen.rs`) is displayed after the Ralph Loop completes execution. Currently it has three sections: Summary, Changed Files, and Verification. The Summary shows hardcoded zeros because `build_loop_result()` doesn't populate actual data. The Verification section is always empty. Users need to see which tasks were completed.

The existing loop execution screen already has a working tab implementation (Info/Agent tabs with Tab key switching), providing a proven pattern to follow.

## Goals / Non-Goals

**Goals:**
- Display accurate story and task completion counts in Summary
- Show task list with completion checkboxes ([x] / [ ])
- Replace Verification section with a tabbed interface (Tasks / Changed Files)
- Maintain consistency with existing tab patterns in the codebase

**Non-Goals:**
- Running verification commands (build, test) from result screen
- Editing task completion status from result screen
- Persisting result data between sessions

## Decisions

### 1. Data Source for Task Completion Status

**Decision**: Re-parse tasks.md via OpenSpecAdapter after loop completion

**Rationale**: The agent modifies tasks.md directly during execution, marking tasks complete with `[x]`. Re-parsing ensures we show the actual file state rather than potentially stale in-memory data.

**Alternatives considered**:
- Use `app.stories` directly: Rejected because it's loaded at preview time and not updated during loop execution
- Track completion via LoopEvent: Would require orchestrator changes and duplicate state management

### 2. Store Stories in LoopResult vs Reference from App

**Decision**: Add `stories: Vec<Story>` field to `LoopResult`

**Rationale**: Makes LoopResult self-contained. The result screen can render without needing access to other App state. Consistent with how `changed_files` is already stored in LoopResult.

### 3. Tab Implementation Pattern

**Decision**: Follow existing `LoopTab` pattern from loop execution screen

**Rationale**: Proven pattern already in codebase. Uses enum for tab state, Tab key for switching, separate scroll offsets per tab.

### 4. Layout Structure

**Decision**: Two-section layout (Summary + Tabbed Content)

```
┌─────────────────────────────────┐
│ Summary (5 lines)               │
│   Stories: 3/5 completed        │
│   Tasks: 12/15 completed        │
├─────────────────────────────────┤
│ [Tasks]  [Changed Files]        │
│                                 │
│ (scrollable content area)       │
│                                 │
└─────────────────────────────────┘
```

**Rationale**: Removes the fixed-height Verification section (always empty), gives more space to useful content.

## Risks / Trade-offs

**[Risk]** Re-parsing tasks.md adds I/O at loop completion
→ Mitigation: Single file read is negligible; only happens once when transitioning to result screen

**[Risk]** Tab state not preserved when navigating away and back
→ Mitigation: Acceptable for MVP; result screen is typically viewed once then dismissed

**[Trade-off]** Storing stories in LoopResult duplicates data briefly
→ Acceptable: Data is small, simplifies rendering logic, gets cleaned up on next loop or app exit
