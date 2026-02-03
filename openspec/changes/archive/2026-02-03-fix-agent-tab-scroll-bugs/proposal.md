## Why

The Agent tab in the Loop screen has two scroll-related bugs that break user experience:
1. First scroll up causes viewport to jump to the top (instead of scrolling one line up from bottom)
2. Scrolling sometimes becomes unresponsive when auto_scroll is enabled, particularly when max_scroll is 0 or content is small

Both bugs stem from `loop_agent_scroll` not being synchronized with the actual viewport position when `auto_scroll=true`.

## What Changes

- Fix scroll position synchronization when transitioning out of auto_scroll mode
- Prevent unnecessary scroll operations when already at auto_scroll bottom position
- Ensure `loop_agent_scroll` reflects actual viewport position before manual scroll operations

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `loop-story-tabs`: Fix scroll synchronization behavior when auto_scroll is enabled

## Impact

- `src/app.rs`: `loop_scroll_up()` and `loop_scroll_down()` methods need modification to handle auto_scroll state properly
