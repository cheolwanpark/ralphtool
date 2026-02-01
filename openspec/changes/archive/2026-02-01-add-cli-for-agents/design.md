# Design: Add CLI for Agents

## Context

The Ralph Loop is an autonomous AI agent loop that spawns fresh AI instances iteratively. Each instance has no memory of previous iterations—memory persists through git commits and file state (tasks.md, design.md). Currently, agents manipulate these files directly, which is error-prone.

This change adds a structured `ralphtool agent` subcommand that provides validated, scoped access to Ralph state. The CLI is designed for machine consumption (JSON output, requires session) rather than human use.

Current architecture:
- `ralphtool` → TUI application (ratatui-based)
- `src/ralph/` → Ralph domain types and OpenSpecAdapter
- No CLI argument parsing currently (always launches TUI)

## Goals / Non-Goals

**Goals:**
- Provide structured CLI for coding agents to read/write Ralph state
- Session-based isolation for concurrent-safe operations
- Story-scoped context per iteration
- JSON output for machine parsing
- Accumulate learnings in session, flush at end
- Helpful error messages when used incorrectly

**Non-Goals:**
- Human-friendly CLI (TUI remains the human interface)
- MCP server (may add later as wrapper)
- Orchestrator implementation (separate concern)
- Modifying existing TUI behavior

## Decisions

### Decision 1: Subcommand structure

**Choice:** `ralphtool agent <command>` as subcommand tree.

**Alternatives considered:**
- Separate binary (`ralph-agent`): More complex build, harder to share code
- Flag-based (`ralphtool --agent`): Less discoverable, awkward

**Structure:**
```
ralphtool                    # No args → TUI (existing behavior)
ralphtool agent              # Show help for agent commands
ralphtool agent session init # Orchestrator: init session
ralphtool agent session next-story  # Orchestrator: get next story
ralphtool agent session flush       # Orchestrator: flush and cleanup
ralphtool agent context      # Agent: get story context
ralphtool agent task done    # Agent: mark task complete
ralphtool agent status       # Agent: check status
ralphtool agent learn        # Agent: record learning
```

### Decision 2: Session via environment variable

**Choice:** Require `RALPH_SESSION` env var for all agent commands.

**Rationale:**
- Orchestrator sets it once when spawning agent
- Agent doesn't need to know about sessions
- Commands fail with helpful message if missing
- Enables session isolation without agent awareness

**Additional:** `RALPH_STORY` env var for story scope (set by orchestrator per iteration).

### Decision 3: Session storage location

**Choice:** `std::env::temp_dir()/ralph/sessions/<session_id>.json`

**Rationale:**
- OS-agnostic (works on macOS, Linux, Windows)
- Automatically cleaned on reboot
- No project directory pollution
- Session ID is UUID, prevents collisions

### Decision 4: File locking strategy

**Choice:** flock-based locking on session init, command-level locking for writes.

**Implementation:**
- Session init acquires exclusive lock on `.ralph/locks/<change>.lock`
- Individual task updates use file-level locks for atomicity
- Lock released on session flush or process death

### Decision 5: Learnings accumulation

**Choice:** Buffer learnings in session state, write to design.md on flush.

**Rationale:**
- Reduces file writes during iteration
- Atomic batch write at end
- Learnings immediately available in context (from session state)
- No risk of partial writes on agent crash

### Decision 6: CLI framework

**Choice:** Add `clap` for argument parsing.

**Rationale:**
- Industry standard for Rust CLIs
- Derive macros for type-safe parsing
- Auto-generated help text
- Already familiar pattern

## Risks / Trade-offs

**[Session file in temp dir may be cleaned unexpectedly]**
→ Mitigation: Session has reasonable TTL, orchestrator flushes promptly

**[Lock file may not work across network filesystems]**
→ Mitigation: Document local-filesystem requirement, most CI runs locally

**[Agent may crash without flushing learnings]**
→ Mitigation: Learnings are convenience, task completion is persisted immediately

**[Clap adds dependency weight]**
→ Mitigation: Clap is well-maintained, benefit outweighs cost

## Module Structure

```
src/
├── main.rs          # Entry point, dispatch TUI vs agent
├── agent/           # NEW: Agent CLI module
│   ├── mod.rs       # Subcommand dispatch
│   ├── cli.rs       # Clap definitions
│   ├── session.rs   # Session lifecycle
│   ├── context.rs   # Context retrieval
│   ├── tasks.rs     # Task operations
│   └── progress.rs  # Learning operations
├── ralph/           # Existing: Domain types, adapter
└── ui/              # Existing: TUI
```

## Open Questions

1. Should `ralphtool agent --help` show a note that this is for machine use?
   → Resolved: Yes, include warning in help text.

2. How to handle session recovery after crash?
   → Deferred: For now, require re-init. May add recovery later.
