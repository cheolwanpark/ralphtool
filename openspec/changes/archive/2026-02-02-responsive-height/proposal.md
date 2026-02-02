## Why

The TUI currently uses responsive width (85% of terminal, clamped 60-140 columns) but uses full height, leaving content anchored to the top. On tall terminals, this creates visual imbalance with excessive empty space at the bottom. Adding responsive height with vertical centering completes the centered layout experience.

## What Changes

- Add `responsive_height()` function calculating height as percentage of terminal with min/max bounds
- Update `centered_rect()` to apply both responsive width and height with vertical centering
- Content will be vertically centered within the terminal, similar to how it's horizontally centered

## Capabilities

### New Capabilities

- `responsive-height`: Percentage-based responsive height calculation with vertical centering, mirroring the existing responsive-width approach

### Modified Capabilities

- `responsive-layout`: Extends to include height calculation alongside width
- `tui-centered-layout`: Adds vertical centering to the existing horizontal centering

## Impact

- `src/ui/mod.rs`: Add `responsive_height()`, update `centered_rect()` signature and implementation
- All screen renderers (`loop_screen.rs`, `preview.rs`, `result_screen.rs`, `selection.rs`): May need adjustments if they rely on full height behavior
