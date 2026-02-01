## Context

The agent module (`src/agent/`) provides a CLI for coding agents (Claude, Amp) to interact with Ralph state. Currently it:
- Writes directly to `tasks.md` when marking tasks complete
- Writes directly to `design.md` when flushing learnings
- Reads files directly for context assembly

The Ralph abstraction (`src/ralph/`) provides traits (`TaskSource`, `ProgressTracker`, etc.) but:
- `mark_complete()` only updates in-memory state
- `record_learning()` is a no-op
- The "ralph" name is misleading—it's a spec abstraction

## Goals / Non-Goals

**Goals:**
- Agent layer uses abstraction exclusively for all spec operations
- Rename `src/ralph/` to `src/spec/` for clarity
- Move learnings/patterns ownership to agent layer (ephemeral session state)
- Spec abstraction provides `SpecWriter` trait for agent to call on flush
- Spec abstraction provides `ContextProvider` trait for unified context retrieval
- OpenSpec adapter implements persistence via CLI-first, file fallback

**Non-Goals:**
- Adding new OpenSpec CLI commands (use file fallback if CLI doesn't support)
- Changing the external agent CLI interface
- Modifying orchestrator behavior

## Decisions

### Decision: Rename ralph → spec

The abstraction layer is about spec operations, not "Ralph" branding. Renaming to `src/spec/` makes the purpose clear.

**Alternatives considered:**
- Keep `ralph/` name → Rejected: confusing, "Ralph" is the orchestrator concept not the abstraction
- Name it `adapter/` → Rejected: too generic, doesn't convey it's about specs

### Decision: Learnings/patterns owned by agent layer

Learnings and patterns are ephemeral session state—buffered during iteration, persisted on flush. This is agent responsibility, not spec abstraction.

**Before:**
```
Agent → session.learnings.push() → session JSON
Agent → write_learnings_to_design() → design.md  ← WRONG: bypasses abstraction
```

**After:**
```
Agent → session.learnings.push() → session JSON
Agent → adapter.write_learnings() → design.md   ← through abstraction
```

**Alternatives considered:**
- Keep ProgressTracker in spec layer → Rejected: learnings are session-scoped, not spec-scoped

### Decision: Remove ProgressTracker trait from spec layer

Replace with `SpecWriter` trait that agent calls on flush. The spec abstraction doesn't "track progress"—it writes data when asked.

```rust
// OLD (spec layer owns accumulation)
trait ProgressTracker {
    fn record_learning(&mut self, learning: Learning);  // no-op in current impl
}

// NEW (agent owns accumulation, calls spec layer on flush)
trait SpecWriter {
    fn write_learnings(&mut self, learnings: &[Learning]) -> Result<()>;
    fn write_patterns(&mut self, patterns: &[Pattern]) -> Result<()>;
}
```

### Decision: Add ContextProvider trait

Agent's `context` command currently assembles context by reading files directly. Move this to the abstraction.

```rust
trait ContextProvider {
    fn get_context(&self, story_id: &str) -> Result<WorkContext>;
    fn get_status(&self) -> Result<WorkStatus>;
}

struct WorkContext {
    story: Story,
    tasks: Vec<Task>,
    proposal: String,
    design: String,
    scenarios: Vec<Scenario>,
    verify: VerifyCommands,
}
```

### Decision: CLI-first with file fallback for OpenSpec adapter

OpenSpec CLI doesn't have `task done` or `learn` commands. The adapter will:
1. Try CLI command if available
2. Fall back to direct file manipulation

```rust
fn mark_complete(&mut self, task_id: &str) -> Result<()> {
    // TRY: openspec task done <id> --change <name>
    // FALLBACK: update tasks.md checkbox directly
}
```

## Risks / Trade-offs

**Risk: Breaking change to module structure**
→ Mitigation: Single PR with all import updates, run full test suite

**Risk: OpenSpec CLI may add conflicting commands later**
→ Mitigation: File fallback is isolated, easy to switch to CLI when available

**Risk: Performance overhead from CLI calls**
→ Mitigation: File fallback is fast, CLI calls only for status/list operations

## Open Questions

- Should `SpecWriter::write_patterns()` write to `specs/` or `design.md`? (Current: design.md)
