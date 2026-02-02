## 1. Core Layout Function

- [x] 1.1 Add `responsive_width()` function in `src/ui/mod.rs` that calculates 85% of terminal width clamped to [60, 140]
- [x] 1.2 Simplify `centered_rect()` to remove `max_height` parameter - only take area and use responsive width
- [x] 1.3 Remove `MAX_WIDTH` constant from `src/ui/mod.rs`

## 2. Update Screen Renderers

- [x] 2.1 Update `render_selection()` in `src/ui/selection.rs` to use new `centered_rect()` signature
- [x] 2.2 Update `render_preview()` in `src/ui/preview.rs` to use new `centered_rect()` signature
- [x] 2.3 Update `render_loop_screen()` in `src/ui/loop_screen.rs` to use new `centered_rect()` signature
- [x] 2.4 Update `render_result_screen()` in `src/ui/result_screen.rs` to use new `centered_rect()` signature

## 3. Dead Code Cleanup

- [x] 3.1 Remove any unused imports after refactoring
- [x] 3.2 Verify no references to old `MAX_WIDTH` constant remain in codebase
- [x] 3.3 Run `cargo build` to verify compilation succeeds with no warnings

## 4. Verification

- [x] 4.1 Test on narrow terminal (80 columns) - verify minimum width behavior
- [x] 4.2 Test on standard terminal (120 columns) - verify responsive percentage
- [x] 4.3 Test on wide terminal (200+ columns) - verify maximum width cap
