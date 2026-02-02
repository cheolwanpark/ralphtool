## Context

The TUI currently uses a `centered_rect()` function that takes `max_width` and `max_height` parameters, with `MAX_WIDTH = 100` as a constant. All screens call this with fixed values, resulting in:
- Wasted horizontal space on wide terminals (e.g., 200+ columns)
- The `max_height` parameter is effectively unused (always passed as `area.height`)

The responsive header system already adapts based on terminal height (full vs compact mode), demonstrating the pattern we want to extend to width.

## Goals / Non-Goals

**Goals:**
- Replace fixed width constraint with percentage-based calculation
- Maintain minimum width for usability (content requires ~60 columns minimum)
- Cap maximum width for readability (very wide text is hard to read)
- Remove dead code: unused constants and parameters
- Keep the centered layout behavior

**Non-Goals:**
- Adding configuration file or runtime settings for layout values
- Changing the responsive header breakpoints
- Adding horizontal scrolling for narrow terminals
- Supporting different layouts per screen

## Decisions

### Decision 1: Percentage-based width with bounds

**Choice**: Calculate width as 85% of terminal width, clamped to [60, 140]

```rust
fn responsive_width(terminal_width: u16) -> u16 {
    let target = (terminal_width as f32 * 0.85) as u16;
    target.clamp(60, 140)
}
```

**Rationale**:
- 85% provides comfortable margins without wasting too much space
- Min 60 ensures content remains readable (selection list, log entries)
- Max 140 prevents overly wide text which hurts readability

**Alternatives considered**:
- Fixed breakpoints (60/80/100/120 based on terminal size) - more complex, less fluid
- Pure percentage (no bounds) - could result in unusable layouts at extremes

### Decision 2: Remove max_height parameter

**Choice**: Simplify `centered_rect()` to only handle width, use full height

**Rationale**:
- Analysis shows `max_height` is always passed as `area.height` (no actual capping)
- Vertical centering adds complexity without benefit for this TUI
- Simpler API: `centered_rect(area, width)` vs `centered_rect(area, width, height)`

### Decision 3: Single responsive function replaces constant

**Choice**: Replace `MAX_WIDTH` constant with `responsive_width()` function

**Rationale**:
- Single point of change for width calculation logic
- Easy to adjust percentage/bounds if needed
- Clean removal of dead constant

## Risks / Trade-offs

**[Risk] Breaking visual consistency** → Users on 100-column terminals will see slightly different layout (85 columns instead of 100)
- Mitigation: The change is subtle and improves overall appearance

**[Risk] Content overflow on narrow terminals** → Content designed for ~80 columns might truncate
- Mitigation: Min width of 60 ensures basic usability; content already handles wrapping

**[Trade-off] Simplicity vs flexibility** → Single percentage value vs configurable
- Accepted: Configuration adds complexity without clear benefit; can be added later if needed
