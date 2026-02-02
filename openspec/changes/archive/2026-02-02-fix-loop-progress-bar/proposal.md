## Why

The progress bar in the Ralph loop execution screen always shows 0% completion because `completed_stories` is never transmitted from the orchestrator to the TUI. The orchestrator calculates this value correctly but only updates a local state variable—it's not included in the `LoopEvent::StoryProgress` event sent to the app.

## What Changes

- Add `completed` field to `LoopEvent::StoryProgress` event to transmit completed story count
- Update orchestrator to include `completed_stories` when emitting progress events
- Update app's `process_loop_events()` to extract and store the completed count in `loop_state`

## Capabilities

### New Capabilities

(none - this is a bug fix affecting existing functionality)

### Modified Capabilities

(none - no spec-level behavior changes, only implementation fix)

## Impact

- `src/ralph_loop/mod.rs`: Add `completed` field to `LoopEvent::StoryProgress` enum
- `src/ralph_loop/orchestrator.rs`: Include `completed_stories` in event emission
- `src/app.rs`: Destructure and update `completed_stories` in `process_loop_events()`
- Tests using `LoopEvent::StoryProgress` will need updating for the new field
