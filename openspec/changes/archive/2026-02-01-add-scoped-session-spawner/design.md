## Context

The orchestrator spawns Claude Code as a subprocess to execute coding tasks. Claude's bash commands need to call back to `ralphtool agent task done <id>` to mark tasks complete. These commands require `RALPH_SESSION` and `RALPH_STORY` environment variables, but the current orchestrator:

1. Uses the spec adapter directly (bypassing session CLI)
2. Never initializes a session
3. Never sets environment variables before spawning Claude

The session management CLI (`session init`, `session next-story`, `session flush`) exists but isn't wired into the orchestrator flow.

## Goals / Non-Goals

**Goals:**
- Provide an ergonomic RAII wrapper that manages session lifecycle
- Automatically configure subprocess environment with session variables
- Integrate cleanly with existing orchestrator without major refactoring
- Inherit parent process environment plus session-specific variables

**Non-Goals:**
- Changing the session CLI interface
- Modifying how sessions are stored or locked
- Adding async spawn support (sync `Command` is sufficient for now)
- Wrapping `SpecAdapter` inside `ScopedSession` (keep them separate)

## Decisions

### Decision: RAII ownership pattern

**Choice:** `ScopedSession` struct that initializes session on `new()` and cleans up on `drop()`.

**Rationale:** Rust's ownership model ensures cleanup happens even on early returns or panics. The session lock is held for the struct's lifetime, preventing concurrent access to the same change.

**Alternatives considered:**
- Callback/closure style (`with_session(|env| { ... })`) - Less flexible, harder to use in async contexts
- Manual init/flush calls - Error-prone, easy to forget cleanup

### Decision: Environment access via HashMap

**Choice:** Provide `ScopedSession::env() -> HashMap<String, String>` that returns session environment variables.

**Rationale:** The orchestrator passes env vars to `AgentConfig`, which then configures the subprocess. This integrates cleanly with the agent abstraction layer rather than coupling `ScopedSession` directly to `std::process::Command`.

**Alternatives considered:**
- `command() -> Command` helper - Couples session to Command, bypasses agent abstraction
- Trait-based injection - Overcomplicated for the use case

### Decision: Environment inheritance

**Choice:** Inherit parent process environment, then add session variables.

**Rationale:** Subprocess needs PATH, HOME, and other standard variables to function. Only session-specific vars need to be added.

### Decision: Explicit story management

**Choice:** `ScopedSession::next_story()` returns `Option<String>` for orchestrator to iterate.

**Rationale:** Orchestrator controls the loop, session just tracks state. Keeps concerns separated.

## Risks / Trade-offs

**[Risk] Session cleanup on panic** → `Drop` trait ensures cleanup runs even on panic, but may leave lock file if process is killed (SIGKILL). Existing lock timeout mechanism handles this.

**[Risk] Lifetime complexity** → `ScopedSession` borrows nothing, owns all data. The returned `HashMap` is owned by the caller.
