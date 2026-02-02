## Why

The TUI has two visual styling issues that degrade user experience:
1. In the Result screen's Changed Files tab, color styling is applied to the entire line instead of just the status character, making the display hard to read
2. In the Loop screen, ellipsis indicators ("...") appear when stories overflow, but they're unnecessary since the current story is already centered in the visible window

## What Changes

- **Result Screen Changed Files**: Apply color styling only to the status character (M/A/D), leaving the filename in default color
- **Loop Screen Story Progress**: Remove the ellipsis ("...") indicators from story navigation display while keeping the centering behavior

## Capabilities

### New Capabilities
<!-- None - this is a fix to existing capabilities -->

### Modified Capabilities
- `result-screen`: Fix color styling in Changed Files tab to apply only to status character
- `loop-story-navigation`: Remove ellipsis indicators from story progress display

## Impact

- `src/ui/result_screen.rs`: `render_changed_files` function - split line into multiple styled spans
- `src/ui/loop_screen.rs`: `render_story_progress` function - remove ellipsis span generation
