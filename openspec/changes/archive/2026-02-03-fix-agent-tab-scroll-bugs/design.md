## Context

The Agent tab uses two state variables for scroll control:
- `loop_agent_scroll: usize` - the scroll offset value
- `loop_agent_auto_scroll: bool` - whether to automatically follow new content

When `auto_scroll=true`, the render function uses `max_scroll` as the viewport position, ignoring `loop_agent_scroll`. This creates a desync: the viewport is at `max_scroll`, but `loop_agent_scroll` remains at its previous value (often 0).

When user scrolls up:
1. `loop_agent_scroll` decrements from 0 → 0 (saturating_sub)
2. `auto_scroll` becomes false
3. Next render uses `loop_agent_scroll` (0) instead of `max_scroll`
4. Viewport jumps from bottom to top

## Goals / Non-Goals

**Goals:**
- Fix scroll-up causing viewport jump to top
- Fix scroll becoming unresponsive when auto_scroll is true and max_scroll is 0

**Non-Goals:**
- Changing the auto_scroll feature behavior
- Modifying Info tab scroll (unaffected by this bug)

## Decisions

### Decision 1: Synchronize scroll position before manual scroll operations

**Choice:** In `loop_scroll_up()`, sync `loop_agent_scroll = max_scroll` before decrementing when `auto_scroll=true`

**Rationale:** This ensures the scroll position starts from the actual viewport position, not a stale value.

**Alternative considered:** Sync in render function - rejected because it would make scroll state mutations happen in render, violating separation of concerns.

### Decision 2: No-op scroll_down when already at auto_scroll bottom

**Choice:** In `loop_scroll_down()`, return early if `auto_scroll=true` (already at bottom)

**Rationale:**
- When auto_scroll is true, viewport is already at max_scroll
- Scrolling down further is meaningless
- Prevents state churn and potential edge cases with max_scroll=0

## Risks / Trade-offs

**[Risk] max_scroll is 0 when content fits in viewport** → Scroll operations become no-ops, which is correct behavior (nothing to scroll)

**[Trade-off] Adding conditional logic to scroll functions** → Slight complexity increase, but necessary for correctness
