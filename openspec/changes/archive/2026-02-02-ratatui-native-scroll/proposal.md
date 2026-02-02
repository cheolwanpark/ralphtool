## Why

Current scroll implementation uses manual `skip()` and `take()` on line collections, which causes issues:
1. Scroll doesn't work properly on Loop screen - appears frozen then jumps
2. Wrap and scroll position are misaligned (skip operates on logical lines, not rendered lines)
3. Story switching doesn't reset scroll, causing empty screens when new story has fewer lines
4. Inconsistent patterns across different screens (some use List, others Paragraph)

## What Changes

- Replace all manual `skip().collect()` scroll patterns with Ratatui's native `Paragraph::scroll((y, x))`
- Use `Paragraph::line_count(width)` to calculate actual rendered line count for proper scroll bounds
- Convert `result_screen.rs` from `List` widget to `Paragraph` for consistency
- Add `loop_agent_auto_scroll: bool` flag for snap-to-bottom behavior in Agent tab
- Reset scroll position when story changes in loop screen
- Remove custom scroll calculation functions (`calculate_scroll_offset`, `is_at_bottom`)

## Capabilities

### New Capabilities
- `native-scroll`: Unified scroll implementation using Ratatui's native Paragraph::scroll() across all screens

### Modified Capabilities
- `loop-story-tabs`: Add auto_scroll flag and proper snap-to-bottom using native scroll
- `mouse-scroll`: Implementation detail change only (same behavior, native method)

## Impact

- `src/ui/preview.rs` - Replace skip() with scroll()
- `src/ui/loop_screen.rs` - Replace skip() with scroll(), add auto_scroll logic, reset scroll on story change
- `src/ui/result_screen.rs` - Convert List to Paragraph, replace skip/take with scroll()
- `src/app.rs` - Add `loop_agent_auto_scroll` field, modify scroll methods, reset scroll in story navigation
- `src/event.rs` - Update scroll handlers to manage auto_scroll flag
