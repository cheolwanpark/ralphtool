## Why

The current Loop Execution screen displays agent output as a simple log list, making it difficult to track progress across stories, review past work, or understand agent activity in context. Users need a richer interface that shows story-level progress, allows navigation between stories, and presents agent messages with full context including role identification and usage statistics.

## What Changes

- Replace the simple log display with a story-centric interface featuring story navigation and dual-tab content
- Add progress bar showing overall story completion with change name display
- Implement story indicator showing up to 5 story numbers with current story highlighted in color
- Add Info tab displaying story details (title, tasks with completion status)
- Add Agent tab showing full messages with role labels (Assistant/Done), proper spacing, and usage stats
- Enable keyboard navigation (arrow keys for story selection, Tab for tab switching)
- Add mouse scroll support across all scrollable screens
- Implement scroll snap behavior (auto-scroll to end when at bottom)
- Send StreamEvent data directly from orchestrator instead of truncated strings
- Store messages per-story instead of single flat log

## Capabilities

### New Capabilities
- `loop-story-navigation`: Story-based navigation in loop execution screen with keyboard controls and visible story indicators
- `loop-story-tabs`: Dual-tab interface (Info/Agent) for viewing story details and agent messages
- `mouse-scroll`: Mouse wheel scroll support for TUI screens

### Modified Capabilities
- `ralph-loop`: Change event structure from `AgentOutput { line }` to `StoryEvent { story_id, event: StreamEvent }` to preserve full message data and enable per-story message storage

## Impact

- `src/ralph_loop/mod.rs`: LoopEvent enum changes, LoopState additions
- `src/ralph_loop/orchestrator.rs`: Emit StoryEvent instead of AgentOutput
- `src/app.rs`: New state fields for story navigation, tab selection, scroll positions; replace loop_log with story_events HashMap
- `src/ui/loop_screen.rs`: Complete rewrite with new layout (progress bar, story indicator, tabbed content)
- `src/event.rs`: Add mouse event handling, new keyboard handlers for loop screen
- `src/main.rs`: Enable mouse capture in terminal initialization
- `src/agent/mod.rs`: StreamEvent may need Clone derive if not already present
