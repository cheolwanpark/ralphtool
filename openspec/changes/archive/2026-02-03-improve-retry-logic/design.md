## Context

The Orchestrator runs agents for each story in sequence. Currently, if an agent fails (error, crash, or missing completion signal), the entire loop stops and all uncommitted changes are lost. Users must manually investigate and restart.

Current flow in `orchestrator.rs`:
```
loop {
    story = next_incomplete_story()
    result = agent.run(prompt)
    if result.has(COMPLETE) → continue to next story
    else → emit Error, break
}
```

## Goals / Non-Goals

**Goals:**
- Preserve working directory state before each agent spawn
- Automatically revert on failure and retry up to N times
- Capture failure reasons from agents when available
- Clean up checkpoint artifacts on success or final failure

**Non-Goals:**
- Custom checkpoint backends (filesystem snapshots, etc.) — git stash is sufficient
- Per-file granularity — story-level checkpoints are enough
- Automatic conflict resolution — revert is a hard reset to checkpoint

## Decisions

### D1: Use git stash for checkpoints

**Decision**: Use `git stash push -u -m "ralph:{change}:{story}"` for checkpoints.

**Rationale**:
- Git is already required for the project
- Stash handles both tracked and untracked files (`-u` flag)
- Named stashes allow targeted revert/drop operations
- No new dependencies needed

**Alternatives considered**:
- Filesystem copy: More complex, doesn't handle git state
- Git branches: Pollutes branch namespace, harder to clean up
- Custom snapshot system: Overkill for this use case

### D2: Stash naming convention

**Decision**: `ralph:{change_name}:{story_id}`

**Rationale**:
- Prefix `ralph:` identifies tool-managed stashes
- Change and story context allows targeted operations
- No attempt counter needed — only one stash per story at a time

### D3: Revert strategy

**Decision**: Use `git stash apply` (not `pop`) for revert, `git stash drop` on success.

**Rationale**:
- Apply keeps the stash for potential subsequent retries
- Drop only after confirmed success
- If max retries exceeded, stash remains for manual investigation

### D4: Promise protocol extension

**Decision**: Add `<promise>FAILED: {reason}</promise>` as optional failure signal.

**Rationale**:
- Agents can gracefully report why they couldn't complete
- Reason is included in next retry prompt
- Absence of any promise = abnormal termination, retry without context

### D5: CLI configuration

**Decision**: `--max-retries N` flag with default of 3.

**Rationale**:
- Simple, discoverable
- Default of 3 balances retry chance vs. infinite loops
- Environment variable override not needed initially

## Risks / Trade-offs

**[Git stash conflicts]** → Stash apply may fail if files changed between save and apply. Mitigation: This shouldn't happen since we revert before any new changes, but log clear error if it does.

**[Untracked file accumulation]** → Agent might create files, then those get included in stash. Mitigation: This is expected behavior; stash captures full state.

**[Stash leak on crash]** → If ralphtool crashes, stashes remain. Mitigation: Acceptable; users can clean up with `git stash list` and drop manually. Could add cleanup command later.

**[Max retries too low/high]** → 3 might not be enough for flaky issues, or too many for deterministic failures. Mitigation: CLI flag allows adjustment per run.
