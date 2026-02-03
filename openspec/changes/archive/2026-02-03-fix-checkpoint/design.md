## Context

The Ralph Loop uses a checkpoint system to preserve working directory state before agent execution and restore it on failure. The current implementation uses `git stash push -u` which has a fundamental flaw: it saves changes AND cleans the working directory back to HEAD. This causes Story N's completed work to be stashed when Story N+1 starts, leaving agents confused about missing code.

Current flow (broken):
```
Story N complete → agent made changes → checkpoint.save(story-N+1)
→ git stash push -u → working directory cleaned → Story N changes LOST
```

## Goals / Non-Goals

**Goals:**
- Fix the checkpoint system so completed story changes persist across story transitions
- Provide clean branch/commit-based checkpoints for reliable restore on failure
- Allow users to choose how to handle completed work (cleanup to original branch or keep checkpoint branch)

**Non-Goals:**
- Changing the retry/failure detection logic (COMPLETE/FAILED signals)
- Modifying the agent prompt system
- Adding git conflict resolution (assume clean working state at loop start)

## Decisions

### Decision 1: Use branch + commit instead of stash

**Choice**: Create `ralph/{change}` branch with commits as checkpoints

**Rationale**:
- `git stash` is designed for "save and clean" - not our use case
- `git stash create` + `git stash store` could avoid cleaning but is complex and edge-case prone
- Commits naturally preserve state without modifying working directory
- Branch isolation prevents pollution of user's working branch

**Alternatives considered**:
- `git stash create` (no-clean stash): Complex API, stash list management issues
- Temporary commits on current branch: Pollutes user's branch history
- File-based backup: Doesn't handle git state (new/deleted files) well

### Decision 2: Checkpoint timing

**Choice**:
- Create branch + "initial state" commit at loop start
- Create checkpoint commit AFTER each story completes successfully
- On failure: `git reset --hard HEAD` to restore to last successful checkpoint

**Rationale**:
- Initial commit captures state before any agent changes
- Post-completion commits preserve successful work
- `reset --hard HEAD` is simple and reliable for discarding failed agent changes

**Flow**:
```
Loop start → create ralph/{change} branch → commit "initial state"
Story 1 runs → success → commit "checkpoint: story-1"
Story 2 runs → failure → reset --hard HEAD (back to story-1 checkpoint) → retry
Story 2 retry → success → commit "checkpoint: story-2"
...
All complete → TUI shows completion options
```

### Decision 3: Completion options via TUI

**Choice**: Show completion screen with two options: "cleanup" and "keep"

**cleanup**:
```bash
git checkout {original_branch}
git merge --squash ralph/{change}   # brings all changes as staged
git reset HEAD                       # optional: make unstaged
git branch -D ralph/{change}
```

**keep**:
- Stay on `ralph/{change}` branch
- User handles merge/rebase manually later

**Rationale**:
- `git merge --squash` correctly handles additions, modifications, AND deletions
- Cleanup gives clean uncommitted changes on original branch
- Keep gives flexibility for users who want manual control

### Decision 4: Branch naming and conflict handling

**Choice**:
- Branch name: `ralph/{change-name}`
- If branch exists: delete and recreate (`git checkout -B`)

**Rationale**:
- Namespaced under `ralph/` to avoid conflicts with user branches
- Force-recreate handles abandoned previous runs cleanly
- Simple approach preferred over complex "resume" logic

### Decision 5: Use --allow-empty for initial commit

**Choice**: Always create initial commit with `--allow-empty`

**Rationale**:
- Simplifies code (no conditional logic for "changes exist?")
- Provides consistent base for diff calculations
- Empty commit has negligible cost

## Risks / Trade-offs

**[Risk] User has uncommitted changes at loop start**
→ The initial commit will include them, which is correct behavior (checkpoint should preserve current state)

**[Risk] User manually modifies files during loop execution**
→ Those changes will be included in next checkpoint commit. This is acceptable - user intervention is their responsibility.

**[Risk] Branch ralph/{change} already exists from previous run**
→ Mitigated by using `git checkout -B` which force-recreates the branch

**[Trade-off] Commits visible in reflog even after cleanup**
→ Acceptable. Reflog entries expire naturally. Branch deletion removes refs.

**[Trade-off] More git operations than stash approach**
→ Acceptable. Reliability is more important than minimal git operations.
