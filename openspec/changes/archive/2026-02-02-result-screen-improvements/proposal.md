## Why

The result screen currently displays hardcoded values (0/0 completed, Tasks: 0) because `build_loop_result()` doesn't reference actual loop execution data. Additionally, the Verification section is always empty and provides no value, while the Tasks completion status—which users actually care about—is not visible.

## What Changes

- Fix Summary section to display actual completion counts by re-parsing tasks.md after loop execution
- Replace the static "Changed Files + Verification" layout with a tabbed interface (Tasks / Changed Files)
- Add `ResultTab` enum to App state for tab management
- Update `LoopResult` struct to include `stories: Vec<Story>` for task display
- Add Tab key support for switching between tabs in the result screen
- Update keybindings display to include Tab instruction

## Capabilities

### New Capabilities

- `result-screen`: Result screen display after loop execution showing completion summary, task list with checkboxes, and changed files in a tabbed interface

### Modified Capabilities

<!-- No existing spec-level requirements are changing -->

## Impact

- `src/ui/result_screen.rs`: Major rewrite for tabbed layout and task rendering
- `src/app.rs`: Add `ResultTab` enum and `result_tab` field, update `build_loop_result()` to populate actual data
- `src/event.rs`: Add Tab key handler for result screen
