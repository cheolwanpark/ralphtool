## 1. Core Layout Functions

- [x] 1.1 Add `responsive_height(terminal_height: u16) -> u16` function in `src/ui/mod.rs` calculating 90% height clamped to [20, 50]
- [x] 1.2 Update `centered_rect(area: Rect) -> Rect` to use both `responsive_width` and `responsive_height`, centering vertically as well as horizontally

## 2. Verification

- [x] 2.1 Test layout on various terminal sizes (80x24, 120x40, 200x80) to verify centering behavior
- [x] 2.2 Verify header auto-selection (full vs compact) still works correctly within the centered area
