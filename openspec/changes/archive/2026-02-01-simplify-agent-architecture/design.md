## Context

The current Ralph loop architecture uses a session-based approach:
1. Orchestrator creates a session (UUID, lock file, session state file)
2. Session provides environment variables (`RALPH_SESSION`, `RALPH_STORY`)
3. Agent CLI commands require these env vars to function
4. Orchestrator parses agent output to detect task completions

Problem discovered: Claude Code's Bash tool does **not inherit environment variables** from the parent process. This means the orchestrator can pass env vars to the `claude` CLI, but they're not available when the agent runs `ralphtool agent task done`. This causes failures and forces agents to manually initialize sessions.

Current code:
- `src/session/` module (~600 lines): state.rs, scoped.rs, cli.rs, instructions.rs, mod.rs
- Lock files: `.ralph/locks/<change>.lock`
- Session files: `/tmp/ralph/sessions/<id>.json`

## Goals / Non-Goals

**Goals:**
- Remove session concept entirely (no session files, no env vars, no agent CLI)
- Let agents work directly with files (read proposals/designs/tasks, edit tasks.md)
- Simplify orchestrator to just spawn agent with a single prompt
- Maintain TUI progress display via file watching or output streaming
- Reduce codebase by ~600 lines

**Non-Goals:**
- Adding new agent capabilities
- Changing how OpenSpec files are structured
- Modifying the TUI screens (beyond removing session-dependent state)
- Adding new verification mechanisms

## Decisions

### Decision 1: Remove agent CLI entirely

**Choice**: Delete all `ralphtool agent *` commands.

**Rationale**:
- The CLI exists only to bridge session state to file operations
- Without sessions, agents can directly read/edit files
- Claude Code has Read, Edit, Write tools that work reliably

**Alternatives considered**:
- Keep CLI but use `RALPH_CHANGE` env var instead of session → Still has env var inheritance problem
- Pass values in prompt and prefix commands → Works but adds unnecessary complexity

### Decision 2: Agent edits tasks.md directly

**Choice**: Agent marks tasks complete by editing `tasks.md`: `[ ]` → `[x]`

**Rationale**:
- OpenSpec files are the source of truth anyway
- Agent already knows how to edit files
- No intermediate state needed
- Progress visible by reading the file

**Alternatives considered**:
- Keep task CLI with file-based change discovery → Adds complexity without benefit

### Decision 3: Single prompt with change location

**Choice**: Provide agent with:
- Change directory path
- Brief instructions on workflow (read files, implement, mark done)
- Verification commands

**Rationale**:
- Agent can read any files it needs
- No need to generate complex multi-section prompts
- Agent adapts to what it finds

### Decision 4: File watching for progress display

**Choice**: TUI watches `tasks.md` for changes to update progress display.

**Rationale**:
- Simple, reliable, no coordination needed
- Works regardless of how agent marks tasks (edit, write)
- Can also stream agent stdout for logs

**Alternatives considered**:
- Parse agent output for patterns → Fragile, doesn't work if agent uses different wording
- Keep event channel from orchestrator → Orchestrator has nothing to emit if agent does everything

### Decision 5: Remove lock files

**Choice**: No lock mechanism for concurrent orchestrator prevention.

**Rationale**:
- Users are unlikely to run multiple agents on same change
- If they do, worst case is conflicting file edits (git handles this)
- Removes fs2 dependency and lock file management

## Risks / Trade-offs

**[Risk] Agent might not follow prompt instructions correctly**
→ Mitigation: Prompt template will be clear and tested. Agent has full file access to self-correct.

**[Risk] No progress events if agent doesn't edit tasks.md regularly**
→ Mitigation: Also stream agent stdout to TUI log. Progress display is informational, not critical.

**[Trade-off] Removing learn command means no accumulated learnings**
→ Accepted: Learnings feature was rarely used. Agent can add notes directly to design.md if needed.

**[Trade-off] No session ID for debugging/tracking**
→ Accepted: Claude Code already tracks its own session. We don't need duplicate tracking.
