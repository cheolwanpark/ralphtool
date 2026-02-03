## 1. Checkpoint Module

- [x] 1.1 Create `src/checkpoint/mod.rs` with Checkpoint struct holding change_name
- [x] 1.2 Implement `save(story_id)` using `git stash push -u -m "ralph:{change}:{story}"`
- [x] 1.3 Implement `find_stash(story_id)` that parses `git stash list` to find stash index by message
- [x] 1.4 Implement `revert(story_id)` using `git stash apply stash@{n}`
- [x] 1.5 Implement `drop(story_id)` using `git stash drop stash@{n}`
- [x] 1.6 Implement `cleanup()` that drops all stashes matching `ralph:{change}:*`
- [x] 1.7 Add checkpoint module to `src/main.rs` module tree

## 2. CLI Configuration

- [x] 2.1 Add `--max-retries` flag to loop subcommand in `src/app.rs` with default value 3
- [x] 2.2 Pass max_retries value to Orchestrator constructor

## 3. Orchestrator Retry Logic

- [x] 3.1 Add `checkpoint: Checkpoint` and `max_retries: usize` fields to Orchestrator
- [x] 3.2 Add retry loop around agent execution in `run()` method
- [x] 3.3 Call `checkpoint.save()` before agent spawn
- [x] 3.4 Parse agent output for `<promise>COMPLETE</promise>` signal
- [x] 3.5 Parse agent output for `<promise>FAILED: {reason}</promise>` signal
- [x] 3.6 On success: call `checkpoint.drop()` and continue to next story
- [x] 3.7 On failure: call `checkpoint.revert()`, increment retry count, retry if under max
- [x] 3.8 On max retries exceeded: emit Error event and stop loop
- [x] 3.9 Call `checkpoint.cleanup()` on loop exit (success or failure)

## 4. Prompt Generation

- [x] 4.1 Update completion signal instructions in `prompt.rs` to include FAILED protocol
- [x] 4.2 Add `for_story_with_retry_context(story_id, Option<String>)` method to PromptBuilder
- [x] 4.3 Include "Previous Attempt Failed" section in prompt when retry_reason is Some
- [x] 4.4 Update Orchestrator to pass failure reason to prompt builder on retries

## 5. Testing

- [x] 5.1 Add unit tests for Checkpoint methods (mock git commands or use temp repo)
- [x] 5.2 Add unit tests for retry logic in Orchestrator
- [x] 5.3 Add unit tests for FAILED signal parsing
- [x] 5.4 Add integration test for full checkpoint/revert/retry cycle
