## 1. Core Layout Infrastructure

- [x] 1.1 Add layout constants to `src/ui/mod.rs` (MAX_WIDTH=100, MIN_HEIGHT_FOR_LOGO=24, HEADER_LINES=8, HEADER_LINES_COMPACT=1)
- [x] 1.2 Implement `centered_rect(area: Rect, max_width: u16, max_height: u16) -> Rect` helper function
- [x] 1.3 Add LOGO constant as `[&str; 2]` array with Slim Block "RALPH" ASCII art

## 2. Header Section Component

- [x] 2.1 Create `HeaderSection` struct with fields: title, description, keybindings
- [x] 2.2 Implement `render_header_section()` function for full header with logo (8 lines)
- [x] 2.3 Implement `render_header_compact()` function for single-line header without logo
- [x] 2.4 Add logic to choose between full/compact header based on terminal height

## 3. Update Selection Screen

- [x] 3.1 Wrap `render_selection()` content in centered container using `centered_rect()`
- [x] 3.2 Replace old header with new `render_header_section()` call
- [x] 3.3 Update layout constraints to use percentage-based header (20%) + content (80%)

## 4. Update Preview Screen

- [x] 4.1 Wrap `render_preview()` content in centered container
- [x] 4.2 Replace old header with new header section
- [x] 4.3 Adjust tab bar and content area to work within new layout

## 5. Update Loop Screen

- [x] 5.1 Wrap `render_loop_screen()` content in centered container
- [x] 5.2 Replace old header with new header section
- [x] 5.3 Adjust status and log sections to work within new layout

## 6. Update Result Screen

- [x] 6.1 Wrap `render_result_screen()` content in centered container
- [x] 6.2 Replace old header with new header section
- [x] 6.3 Adjust summary, files, and verification sections to work within new layout

## 7. Cleanup

- [x] 7.1 Remove old `render_header()` function and `HeaderContext` struct
- [x] 7.2 Remove unused constants and imports from old header implementation
- [x] 7.3 Verify all screens render correctly at various terminal sizes
