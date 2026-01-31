## Context

This is a greenfield Rust TUI project. The directory currently has only OpenSpec configuration. We need to establish the project structure and core patterns that will be used throughout development.

Constraints:
- Must use ratatui for TUI rendering (user requirement)
- Should follow Rust ecosystem conventions and best practices
- Terminal must be properly restored on exit (graceful shutdown)

## Goals / Non-Goals

**Goals:**
- Establish a clean, idiomatic Rust project structure
- Set up ratatui with crossterm backend
- Create a working main loop with event handling
- Implement proper terminal cleanup on normal exit and panic

**Non-Goals:**
- Building actual application features (just scaffold)
- Complex state management (keep it simple for now)
- Configuration system or CLI argument parsing
- Async runtime (use synchronous event polling)

## Decisions

### 1. Terminal Backend: crossterm
**Decision**: Use crossterm as the terminal backend for ratatui.
**Rationale**: crossterm is cross-platform (Windows, macOS, Linux), well-maintained, and the most commonly used backend with ratatui. Alternatives like termion are Linux-only.

### 2. Project Structure: Flat modules initially
**Decision**: Start with a flat `src/` structure: `main.rs`, `app.rs`, `ui.rs`, `event.rs`.
**Rationale**: Avoid premature abstraction. Easy to refactor into nested modules as the project grows. Keeps initial complexity low.

### 3. Error Handling: anyhow
**Decision**: Use `anyhow` for application error handling.
**Rationale**: Provides ergonomic error context and propagation with `?` operator. Appropriate for application code (vs library code where thiserror would be preferred).

### 4. Event Loop: Synchronous polling
**Decision**: Use synchronous `crossterm::event::poll()` with timeout.
**Rationale**: Simpler than async, sufficient for TUI applications. Avoids tokio dependency for now. Can migrate to async later if needed.

## Risks / Trade-offs

- **[Terminal not restored on panic]** → Install panic hook that restores terminal before unwinding
- **[Blocking on event poll]** → Use timeout-based polling (e.g., 250ms) to allow periodic UI updates
- **[Simple structure may not scale]** → Acceptable for scaffold; refactor when patterns emerge
