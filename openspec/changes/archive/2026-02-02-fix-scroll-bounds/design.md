## Context

The application has multiple scrollable screens using ratatui's `Paragraph` widget with `.scroll((offset, 0))`. Currently, most screens increment scroll offset without checking bounds, allowing users to scroll past content.

The Loop Agent Tab already implements proper scroll clamping:
```rust
let total_lines = paragraph.line_count(inner_area.width);
let max_scroll = total_lines.saturating_sub(inner_area.height as usize);
let scroll_offset = app.loop_agent_scroll.min(max_scroll);
```

## Goals / Non-Goals

**Goals:**
- Prevent scrolling past content end in all scrollable screens
- Use consistent pattern across all scrollable Paragraph widgets
- Handle text wrapping correctly when calculating content height

**Non-Goals:**
- Changing the scroll input handling (key/mouse events)
- Adding scroll indicators or position feedback
- Modifying scroll step sizes

## Decisions

### Decision 1: Render-time clamping vs. scroll-time clamping

**Chosen: Render-time clamping**

Alternatives considered:
- **Scroll-time clamping**: Clamp in `scroll_down()` methods by passing `max_scroll` parameter
  - Con: Requires render area dimensions at scroll time, which is complex to propagate
  - Con: Would need to track max_scroll per-tab in app state

- **Render-time clamping**: Clamp during render when we have the Paragraph and area
  - Pro: Already have all needed information (paragraph, inner_area)
  - Pro: Automatically adapts to window resize
  - Pro: Matches existing Agent Tab pattern

### Decision 2: Line count calculation

Use `paragraph.line_count(inner_area.width)` to get the actual rendered line count accounting for text wrapping, then subtract `inner_area.height` to get `max_scroll`.

## Risks / Trade-offs

**Risk: Content change during render**
→ Not an issue because content is stable during each render cycle

**Trade-off: Scroll offset can temporarily exceed bounds**
→ Acceptable because it's clamped before display. The stored offset being higher than max is harmless.
