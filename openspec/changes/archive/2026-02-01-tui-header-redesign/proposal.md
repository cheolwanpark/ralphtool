## Why

The current TUI has disconnected screen flows (Preview → LoopExecution is not wired) and inconsistent header layouts. Users cannot start the Ralph loop from the Preview screen, and the help text at the bottom is cramped and hard to scan. A unified header with ASCII art branding and clear keybinding guides will improve usability and complete the screen navigation flow.

## What Changes

- Add `R` key binding in Preview screen to transition to LoopExecution
- Redesign header section across all screens with:
  - RalphTool ASCII art branding (compact, 2-3 lines)
  - Screen-specific keybinding reference
  - Contextual usage hints
- Unify header layout across Selection, Preview, LoopExecution, and Result screens
- Move help text from bottom footer into the header section for better visibility

## Capabilities

### New Capabilities

- `tui-header`: Shared header component with ASCII art, keybindings display, and screen context

### Modified Capabilities

- `tui-core`: Adding screen transition from Preview to LoopExecution via `R` key
- `change-selection-screen`: Adopting new header layout
- `conversion-preview-screen`: Adopting new header layout, adding `R` keybinding

## Impact

- `src/ui/mod.rs`: May add shared header rendering function
- `src/ui/selection.rs`: Header layout changes
- `src/ui/preview.rs`: Header layout changes, new keybinding
- `src/ui/loop_screen.rs`: Header layout changes
- `src/ui/result_screen.rs`: Header layout changes
- `src/event.rs`: Add `R` key handler in preview events
- `src/app.rs`: Ensure `start_loop()` is properly called
