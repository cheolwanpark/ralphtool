## Why

The current git stash-based checkpoint system has a fundamental bug: `git stash push -u` saves changes AND cleans the working directory back to HEAD. When Story N completes and Story N+1 starts, `checkpoint.save()` stashes all of Story N's completed work, causing agents to find missing code they just created.

## What Changes

- **BREAKING**: Replace git stash-based checkpoint with branch + commit approach
- Create a dedicated `ralph/{change}` branch at loop start with an "initial state" commit
- Use commits as checkpoints instead of stashes (one commit per completed story)
- On failure, use `git reset --hard HEAD` to restore to last checkpoint
- Add TUI completion options: "cleanup" (squash changes back to original branch) or "keep" (stay on ralph branch)

## Capabilities

### New Capabilities

- `checkpoint-completion`: TUI options for handling completed loop (cleanup vs keep branch)

### Modified Capabilities

- `checkpoint`: Change from git stash to branch + commit approach
- `ralph-loop`: Update orchestrator to work with new checkpoint system and handle completion options

## Impact

- `src/checkpoint/mod.rs`: Complete rewrite - replace stash operations with branch/commit operations
- `src/ralph_loop/orchestrator.rs`: Update checkpoint integration, add completion handling
- `src/ralph_loop/mod.rs`: Add completion option types
- TUI changes for completion option selection
- Existing checkpoint tests will need to be rewritten
