## Context

The TUI currently implements responsive width (85% of terminal, clamped to 60-140 columns) with horizontal centering. Height uses the full terminal area with content anchored to the top. On tall terminals, this creates visual imbalance.

Current state:
- `responsive_width(terminal_width) -> u16`: 85% width clamped to [60, 140]
- `centered_rect(area) -> Rect`: Uses responsive width, full height, horizontal centering only

## Goals / Non-Goals

**Goals:**
- Add `responsive_height()` function mirroring the responsive width approach
- Update `centered_rect()` to center content both horizontally and vertically
- Maintain the existing header adaptive behavior (full vs compact based on height)

**Non-Goals:**
- Changing the 85% ratio or min/max bounds for width
- Per-screen custom height calculations
- Dynamic content-aware height adjustment

## Decisions

### Decision: Height percentage and bounds

Use 90% of terminal height, clamped to [20, 50] rows.

**Rationale:**
- 90% (vs 85% for width) because vertical space is more constrained in typical terminals
- Minimum 20 rows ensures header + content area remain usable
- Maximum 50 rows prevents excessive vertical scrolling distance and keeps content readable

**Alternatives considered:**
- Same 85% as width: Would waste too much vertical space on typical 24-40 row terminals
- No maximum: Content spread too thin on very tall terminals

### Decision: Unified centering in `centered_rect()`

Modify `centered_rect()` to apply both width and height responsiveness, returning a rect that is centered both horizontally and vertically.

**Rationale:**
- Single function provides complete centered layout
- Consistent API with existing usage pattern
- All screens already call `centered_rect()` so change is minimal

### Decision: Keep header calculation inside content area

The header height calculation (`render_header_auto`) will continue to operate within the centered rect, using the centered area's height for its compact/full decision.

**Rationale:**
- Keeps header adaptive to the effective content area, not raw terminal
- Maintains existing header behavior logic

## Risks / Trade-offs

**[Risk] Small terminals may feel cramped** → The 90% ratio and 20-row minimum should handle most cases. Very small terminals (< 22 rows) already use compact header mode.

**[Trade-off] Vertical margins may look asymmetric on odd-height terminals** → Using integer division for centering; difference is at most 1 row which is acceptable.
