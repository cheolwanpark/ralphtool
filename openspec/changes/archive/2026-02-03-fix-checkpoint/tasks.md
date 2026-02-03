## 1. Checkpoint Module Rewrite

- [x] 1.1 Add `original_branch` field to Checkpoint struct to store the branch name before switching
- [x] 1.2 Implement `init()` method: store current branch, run `git checkout -B ralph/{change}`, commit "initial state" with `--allow-empty`
- [x] 1.3 Implement `commit_checkpoint()` method: run `git add -A && git commit -m "checkpoint: {story_id}"`
- [x] 1.4 Implement `revert()` method: run `git reset --hard HEAD`
- [x] 1.5 Implement `cleanup()` method with `CompletionOption` enum (Cleanup/Keep)
- [x] 1.6 Implement cleanup logic: checkout original branch, merge --squash, reset HEAD, delete branch
- [x] 1.7 Implement keep logic: no-op, just return success
- [x] 1.8 Remove old stash-based methods (save, drop, find_stash, old cleanup)
- [x] 1.9 Update/rewrite tests for new branch-based checkpoint system

## 2. Orchestrator Integration

- [x] 2.1 Update orchestrator to call `checkpoint.init()` at loop start (before first story)
- [x] 2.2 Change story completion flow: call `checkpoint.commit_checkpoint()` instead of `drop()`
- [x] 2.3 Update failure handling: call `checkpoint.revert()` (simplified - just reset --hard HEAD)
- [x] 2.4 Remove checkpoint.save() call before each story (no longer needed)
- [x] 2.5 Add `CompletionOption` to `LoopEvent::Complete` event
- [x] 2.6 Update orchestrator to return completion state for TUI to handle

## 3. TUI Completion Screen

- [x] 3.1 Create `CompletionScreen` component with cleanup/keep options
- [x] 3.2 Implement keyboard handling: 'c' for cleanup, 'k' for keep
- [x] 3.3 Display completion summary (stories completed, branch info)
- [x] 3.4 Show option descriptions (what each option does)
- [x] 3.5 Integrate completion screen into TUI flow after loop ends
- [x] 3.6 Handle completion option selection and trigger checkpoint cleanup/keep
- [x] 3.7 Show progress/result of cleanup operation before transitioning to result screen

## 4. Error Case Handling

- [x] 4.1 Show completion screen on max retries (not just error and exit)
- [x] 4.2 Show completion screen on user stop ('q' key)
- [x] 4.3 Ensure partial work can be recovered via cleanup/keep options
