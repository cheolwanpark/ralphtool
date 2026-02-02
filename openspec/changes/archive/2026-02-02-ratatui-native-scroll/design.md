## Context

The TUI has three scrollable screens (Preview, Loop Execution, Result) that currently implement scroll differently:
- Preview: `skip(offset).collect()` + Paragraph + Wrap
- Loop: `skip(offset).collect()` + Paragraph + Wrap + custom `calculate_scroll_offset()`
- Result: `skip(offset).take(visible_height).collect()` + List widget

The manual skip approach has problems:
1. `skip()` operates on logical `Line` objects before Wrap is applied
2. `Wrap` applies during rendering, so actual displayed lines differ from skipped count
3. No bounds checking - scroll can exceed content, showing empty screen
4. Story switching in Loop screen doesn't reset scroll position

Ratatui provides `Paragraph::scroll((y, x))` which scrolls after wrapping, and `line_count(width)` to get actual rendered line count.

## Goals / Non-Goals

**Goals:**
- Unified scroll pattern across all screens using native Ratatui methods
- Working scroll on Loop screen (both Info and Agent tabs)
- Snap-to-bottom behavior for Agent tab (auto-scroll to new messages)
- Scroll position reset when switching stories

**Non-Goals:**
- Horizontal scroll support
- Custom scroll indicators/bars
- Changing the visual appearance of screens

## Decisions

### Decision 1: Use Paragraph::scroll() instead of manual skip()

**Choice:** Replace `lines.skip(offset).collect()` with `Paragraph::new(lines).scroll((offset, 0))`

**Rationale:**
- Native method handles Wrap correctly (scrolls rendered lines, not logical lines)
- Simpler code - no manual visible_lines construction
- Consistent with Ratatui idioms

**Alternatives considered:**
- Keep manual skip() but fix bounds → Still has Wrap alignment issues
- Use ScrollableState → Overkill for simple vertical scroll

### Decision 2: Convert List to Paragraph in result_screen

**Choice:** Replace `List::new(items)` with `Paragraph::new(lines)` in result_screen.rs

**Rationale:**
- List widget is designed for selectable items (not used here)
- Paragraph supports scroll() method natively
- Enables consistent scroll pattern across all screens

**Alternatives considered:**
- Keep List with manual skip/take → Inconsistent with other screens
- Create wrapper abstraction → Over-engineering for simple change

### Decision 3: Auto-scroll flag for Agent tab

**Choice:** Add `loop_agent_auto_scroll: bool` to App state

**Behavior:**
- Default: `true` (follow new content)
- Scroll up: Set to `false`
- Scroll down to bottom: Set back to `true`
- When `true`: Calculate offset as `total_lines - visible_height`

**Rationale:**
- Users expect chat-like interfaces to follow new messages
- Manual scroll up should preserve position (user is reading history)
- Scrolling back to bottom should re-enable following

### Decision 4: Reset scroll on story change

**Choice:** Reset `loop_info_scroll` and `loop_agent_scroll` to 0 when:
- `navigate_to_previous_story()` is called
- `navigate_to_next_story()` is called
- New story is auto-selected in `process_loop_events()`

**Rationale:**
- Different stories have different content lengths
- Old scroll position is meaningless for new story
- Prevents empty screen when new story has fewer lines

### Decision 5: Use line_count() for scroll bounds

**Choice:** Use `Paragraph::line_count(area.width)` to get actual rendered line count

**Application:**
- Agent tab: Calculate max scroll for auto-scroll
- All tabs: Clamp scroll offset to valid range (optional, scroll() handles overflow gracefully)

**Rationale:**
- Accounts for Wrap-induced line expansion
- Accurate bounds prevent user from scrolling into empty space

## Risks / Trade-offs

**Risk:** Performance impact from line_count() on large content
→ Mitigation: Agent tab content is bounded by story; line_count() is O(n) but n is small

**Risk:** Paragraph scroll() may differ slightly from current behavior
→ Mitigation: Current behavior is broken anyway; native behavior is more correct

**Trade-off:** Removing List widget in result_screen loses potential for future selection
→ Acceptable: No current requirement for selection; can add back if needed
