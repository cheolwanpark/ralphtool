## Context

The current Loop Execution screen uses a simple log-based display:
- `LoopEvent::AgentOutput { line: String }` carries truncated text (100 chars max)
- `App::loop_log: Vec<String>` stores all messages as flat strings without story association
- No navigation between stories; users can only watch the current output
- No distinction between intermediate messages and final results

The orchestrator already tracks story context internally but loses this information when emitting events. The agent layer provides `StreamEvent::Message(String)` and `StreamEvent::Done(Response)` with full content and usage stats, but this is converted to simple strings before reaching the TUI.

## Goals / Non-Goals

**Goals:**
- Enable users to navigate between started stories and review past/current work
- Display full agent messages without truncation, with clear role identification
- Show story completion progress with visual progress bar
- Preserve StreamEvent data (including Done response with usage stats) through to the UI
- Add mouse scroll support for better UX

**Non-Goals:**
- Changing how the orchestrator processes stories (iteration logic stays the same)
- Adding story-scenario matching (confirmed: no relationship exists between these concepts)
- Modifying the agent interface or streaming behavior
- Adding new agent capabilities or changing prompt generation

## Decisions

### Decision 1: Replace AgentOutput with StoryEvent

Change the LoopEvent enum to carry full StreamEvent data with story association:

```rust
pub enum LoopEvent {
    StoryProgress { story_id, story_title, current, total },
    StoryEvent {
        story_id: String,
        event: StreamEvent,  // Message(String) or Done(Response)
    },
    Error { message },
    Complete,
}
```

**Rationale**: Preserves all information from the agent layer. The Response struct contains content, turns, tokens, and cost - all valuable for the Done display.

**Alternative considered**: Keep AgentOutput but add optional Response fields. Rejected because it duplicates data structure and complicates the event handling.

### Decision 2: Per-story message storage with HashMap

Replace `loop_log: Vec<String>` with:

```rust
pub story_events: HashMap<String, Vec<StreamEvent>>,
pub started_story_ids: Vec<String>,  // Tracks order of story starts
```

**Rationale**: Enables story-based navigation. Vec<String> for ordering since HashMap doesn't preserve insertion order. StreamEvent stored directly to avoid data loss.

**Alternative considered**: Nested struct `StoryLog { events: Vec<...>, started_at: Instant }`. Rejected as over-engineering - we only need events and order.

### Decision 3: Story indicator with sliding window

Display up to 5 story indicators with the selected story centered when possible:

```
Stories ≤5: Show all            ① ② ③ ④
Stories >5: Sliding window      ③ ④ ⑤ ⑥ ⑦
                                    ↑ selected
```

Navigation constrained to started stories only. Unstarted stories are not shown and cannot be selected.

**Visual states**:
- Current (in progress): Green color
- Completed: Default color
- Selected: Underline

**Rationale**: 5 is enough for context without cluttering. Centering selection keeps context visible in both directions.

### Decision 4: Tab-based content switching

Two tabs in the Story section:
- **Info**: Story title, task list with checkboxes (☑/☐)
- **Agent**: Full messages with "Assistant:" or "Done:" prefix, separator between messages

Tab switching via Tab key. Each tab maintains its own scroll position.

**Rationale**: Separates static info from dynamic content. Users can check task status while agent works.

### Decision 5: Scroll snap to end

When scroll position is at the bottom, new content automatically scrolls into view. When user scrolls up manually, auto-scroll stops. Returns to auto-scroll when user scrolls back to bottom.

```rust
fn should_snap(&self) -> bool {
    self.scroll_offset + visible_height >= total_content_height
}
```

**Rationale**: Users watching live output want auto-scroll. Users reviewing past messages want stable position.

### Decision 6: Mouse capture for scroll

Enable mouse capture at terminal init:
```rust
execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
```

Handle MouseEventKind::ScrollUp/ScrollDown in event loop. Apply to all scrollable screens (Preview, LoopExecution, LoopResult).

**Rationale**: Standard UX expectation. Already handling keyboard scroll - mouse is natural extension.

## Risks / Trade-offs

**[Memory growth]** → Storing full StreamEvent per message uses more memory than truncated strings. Mitigation: Acceptable for typical session lengths. Could add optional pruning for very long runs in future.

**[Breaking event consumers]** → Removing AgentOutput changes the LoopEvent enum. Mitigation: Only internal consumer is App::process_loop_events which we're updating in same change.

**[Complexity increase]** → More state to track (selected story, tab, scroll positions per story). Mitigation: Clear separation of concerns. Each piece of state has single purpose.

**[StreamEvent Clone]** → May need to derive Clone on StreamEvent if not already. Mitigation: Check and add if needed - it's a simple data struct.
