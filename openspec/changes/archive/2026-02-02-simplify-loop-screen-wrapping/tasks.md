## 1. Agent Tab Refactor

- [x] 1.1 Refactor `render_message_lines()` to put "Assistant:" on its own line, followed by indented content
- [x] 1.2 Refactor `render_done_section()` to put "Done:" on its own line, followed by indented content and stats
- [x] 1.3 Update `render_agent_tab()` message separation from 1 to 2 blank lines

## 2. Info Tab Refactor

- [x] 2.1 Refactor `render_info_tab()` to put checkbox+task ID on its own line, description below with indentation

## 3. Cleanup

- [x] 3.1 Delete `wrap_text_with_indent()` function (no longer needed)
- [x] 3.2 Remove any unused imports or constants related to manual wrapping

## 4. Verification

- [x] 4.1 Run `cargo check` to ensure no compilation errors
- [x] 4.2 Run `cargo clippy` to ensure no warnings
