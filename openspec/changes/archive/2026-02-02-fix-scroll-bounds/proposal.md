## Why

Scrollable content in Preview, Loop Info Tab, and Result screens can be scrolled past the end of the content, showing empty space. This creates a confusing user experience where users lose context of where they are in the content.

## What Changes

- Add render-time scroll clamping to all scrollable Paragraph widgets
- Calculate `max_scroll` based on actual rendered line count (accounting for text wrapping) and viewport height
- Clamp scroll offset to `max_scroll` before applying to the Paragraph widget
- Follow the existing pattern already used in the Loop Agent Tab

## Capabilities

### New Capabilities

- `scroll-bounds`: Ensures scroll position is clamped to valid content bounds at render time

### Modified Capabilities

<!-- No existing spec requirements are changing -->

## Impact

- `src/ui/preview.rs`: Add scroll clamping for Tasks and Scenarios tabs
- `src/ui/loop_screen.rs`: Add scroll clamping for Info tab (Agent tab already has it)
- `src/ui/result_screen.rs`: Add scroll clamping for Tasks and Changed Files tabs
