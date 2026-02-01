## Why

When the orchestrator spawns Claude Code as a subprocess, Claude's bash commands fail with "RALPH_SESSION environment variable not set" because session environment variables aren't passed to spawned processes. The session management CLI exists but the orchestrator bypasses it, using the adapter directly without setting up the required environment.

## What Changes

- Add `ScopedSession` wrapper in the session module that:
  - Initializes a session on construction
  - Provides environment variables for subprocess spawning via `env()` method
  - Cleans up session on drop (releases lock, flushes if needed)
- Migrate orchestrator to use `ScopedSession` instead of direct adapter access
- Subprocess spawns inherit parent environment plus session variables (`RALPH_SESSION`, `RALPH_STORY`)

## Capabilities

### New Capabilities

- `scoped-session`: RAII-style session wrapper that manages session lifecycle and provides subprocess environment configuration

### Modified Capabilities

None - this is additive. Existing CLI commands and session management remain unchanged.

## Impact

- `src/session/mod.rs` - Add `ScopedSession` struct and implementation
- `src/ralph_loop/orchestrator.rs` - Use `ScopedSession` instead of direct adapter/session calls
- `src/agent/claude.rs` - Accept env vars via `AgentConfig.env`
