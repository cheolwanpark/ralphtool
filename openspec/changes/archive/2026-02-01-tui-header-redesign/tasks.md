## 1. Shared Header Component

- [x] 1.1 Define ASCII art constant for RalphTool logo (3 lines, ~18 chars wide)
- [x] 1.2 Create `render_header()` function in `src/ui/mod.rs` with three-column layout
- [x] 1.3 Define `HeaderContext` struct to pass screen title, context info, and keybindings

## 2. Screen Header Integration

- [x] 2.1 Update `selection.rs` to use shared header, remove footer help section
- [x] 2.2 Update `preview.rs` to use shared header, pass change name and counts as context
- [x] 2.3 Update `loop_screen.rs` to use shared header
- [x] 2.4 Update `result_screen.rs` to use shared header

## 3. Preview to Loop Navigation

- [x] 3.1 Add `R` key handler in `handle_preview_events()` in `src/event.rs`
- [x] 3.2 Call `app.start_loop()` when R is pressed
- [x] 3.3 Remove `#[allow(dead_code)]` from `start_loop()` method

## 4. Cleanup

- [x] 4.1 Remove old footer render functions from each screen module
- [x] 4.2 Adjust layout constraints to account for new header height (5 lines)
