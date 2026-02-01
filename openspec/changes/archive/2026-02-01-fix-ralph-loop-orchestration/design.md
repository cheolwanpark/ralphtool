## Context

The Ralph loop has all the pieces implemented but not wired together:
- `Orchestrator` in `src/ralph_loop/orchestrator.rs` - loads stories, spawns agents, emits events
- `LoopState` in `src/ralph_loop/mod.rs` - tracks progress (stories/tasks counts)
- `LoopEvent` channel types - for TUI updates
- `render_loop_screen` in `src/ui/loop_screen.rs` - displays the state

Current flow when user presses 'R':
1. `app.start_loop()` creates empty `LoopState(stories_total=0)`
2. Screen switches to `LoopExecution`
3. UI renders 0/0, running=false
4. Nothing else happens

The main challenge is integrating async orchestrator execution with the synchronous TUI event loop.

## Goals / Non-Goals

**Goals:**
- Wire up orchestrator to actually run when loop starts
- Display real story/task counts from the spec adapter
- Update TUI in real-time as orchestrator emits events
- Allow user to stop the loop with 'q'

**Non-Goals:**
- Changing the orchestrator's internal logic (already works)
- Modifying how the coding agent runs (out of scope)
- Adding new UI elements or changing the screen layout

## Decisions

### Decision 1: Use background thread with channel for orchestrator

**Choice**: Spawn orchestrator in a dedicated thread with tokio runtime, communicate via `std::sync::mpsc` channel.

**Rationale**: The TUI uses synchronous `crossterm::event::poll()`. We need a way to:
1. Run the async orchestrator without blocking the TUI
2. Receive events from orchestrator to update display
3. Send stop signal from TUI to orchestrator

Using `std::thread::spawn` with an internal tokio runtime keeps the TUI loop synchronous while the orchestrator runs async.

**Alternatives considered**:
- Make entire app async with tokio: Would require rewriting TUI event loop, more invasive
- Use `try_recv` polling: Simpler but what we'll do for receiving events

### Decision 2: Initialize LoopState before switching screens

**Choice**: Load stories from adapter in `start_loop()` before creating LoopState.

**Rationale**: User should immediately see accurate counts (e.g., "0/6 stories") rather than "0/0".

### Decision 3: Non-blocking event reception in TUI loop

**Choice**: Use `try_recv()` on a `std::sync::mpsc::Receiver` during each TUI tick.

**Rationale**: The existing TUI loop polls for keyboard events every 250ms. We can check for orchestrator events in the same loop without blocking.

## Risks / Trade-offs

**[Thread coordination complexity]** → Keep the interface minimal: one channel for events, one AtomicBool for stop flag.

**[Agent requires `ralphtool` in PATH]** → Document this requirement. When running via `cargo run`, users need `cargo install --path .` first, or we could update instructions to use `cargo run -- agent ...`.
