## Why

The Ralph loop TUI screen displays "0/0" stories and immediately shows "Stopped" because `app.start_loop()` creates an empty `LoopState` but never actually starts the `Orchestrator`. The orchestrator code exists but is dead code - never instantiated or run.

## What Changes

- Wire up the `Orchestrator` to actually run when the user starts the loop from the TUI
- Load stories from the spec adapter and populate `LoopState` with real counts before displaying
- Spawn the orchestrator as an async task that sends events back to update the TUI
- Handle the async runtime integration between the synchronous TUI event loop and the async orchestrator

## Capabilities

### New Capabilities

None - this is fixing existing but non-functional code.

### Modified Capabilities

- `ralph-loop`: Adding requirement for TUI integration - the loop screen must actually start the orchestrator and receive events to update the display.

## Impact

- `src/app.rs`: `start_loop()` needs to initialize state with real story counts and spawn orchestrator
- `src/event.rs`: May need async event handling to receive orchestrator events
- `src/main.rs`: May need tokio runtime for async orchestrator execution
- `Cargo.toml`: May need tokio dependency if not already present
