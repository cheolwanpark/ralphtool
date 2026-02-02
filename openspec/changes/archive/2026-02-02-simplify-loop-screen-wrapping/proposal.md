## Why

The loop screen's Agent and Info tabs have complex manual line wrapping logic that causes visual bugs. Text indentation is inconsistent when content wraps across lines, making messages hard to read. The current `wrap_text_with_indent` function (77 lines) is overly complex for what's needed.

## What Changes

- Separate labels ("Assistant:", "Done:", task checkbox) onto their own lines, with content below
- Remove manual `wrap_text_with_indent` function entirely
- Let ratatui's built-in `Paragraph::wrap()` handle all line wrapping
- Increase message spacing in Agent tab from 1 to 2 blank lines for better readability

## Capabilities

### New Capabilities
<!-- None - this is a refactoring/fix of existing functionality -->

### Modified Capabilities
- `loop-screen`: Changing how text is laid out in Info and Agent tabs (display format change, no functional requirement changes)

## Impact

- **Code**: `src/ui/loop_screen.rs` - refactor `render_info_tab`, `render_agent_tab`, `render_message_lines`, `render_done_section`; delete `wrap_text_with_indent`
- **Visual**: Layout changes in loop execution screen - labels on separate lines from content
- **Dependencies**: None
