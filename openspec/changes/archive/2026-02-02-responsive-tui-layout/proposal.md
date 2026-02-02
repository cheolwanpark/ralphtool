## Why

The current TUI uses fixed max-width (100 columns) and max-height (40 lines) constraints, which wastes screen space on large terminals and may not adapt well to narrow terminals. A responsive layout that calculates dimensions as percentages of actual terminal size would better utilize available space across different terminal configurations.

## What Changes

- Replace fixed `MAX_WIDTH = 100` with percentage-based width calculation (e.g., 85% of terminal width)
- Add minimum and maximum bounds to prevent extreme layouts (min 60, max 140 columns)
- Remove the unused `max_height` parameter from `centered_rect()` since height is always full
- Add responsive width breakpoints for narrow/medium/wide terminals
- **Clean up dead code**: Remove `MAX_WIDTH` constant after migration to responsive calculation
- Update all screen renderers to use the new responsive layout function

## Capabilities

### New Capabilities

- `responsive-layout`: Percentage-based layout calculation with min/max bounds that adapts to terminal dimensions

### Modified Capabilities

- `tui-centered-layout`: Changes from fixed 100-column max-width to responsive percentage-based width calculation with configurable bounds

## Impact

- `src/ui/mod.rs`: Core layout functions (`centered_rect`, constants)
- `src/ui/selection.rs`: Selection screen layout
- `src/ui/preview.rs`: Preview screen layout
- `src/ui/loop_screen.rs`: Loop execution screen layout
- `src/ui/result_screen.rs`: Result screen layout
- Dead code removal: `MAX_WIDTH` constant, unused `max_height` parameter
