## Why

When an agent fails during story execution (crashes, loses context, or fails to produce the expected `<promise>COMPLETE</promise>` signal), the orchestrator currently stops entirely. This loses all uncommitted work and requires manual intervention. We need automatic checkpoint/revert/retry to make the loop resilient.

## What Changes

- Add checkpoint system using git stash before each agent spawn
- Implement automatic revert on failure using stash apply
- Add retry logic with configurable max attempts (CLI flag `--max-retries`, default 3)
- Extend promise protocol: agents can signal `<promise>FAILED: reason</promise>` for graceful failures
- Include failure reason in retry prompts when available
- Clean up stashes on successful completion

## Capabilities

### New Capabilities
- `checkpoint`: Git stash-based state preservation before agent execution, with revert and cleanup operations

### Modified Capabilities
- `orchestration`: Add retry loop around agent execution with checkpoint integration
- `prompt-generation`: Include FAILED promise protocol and retry context in prompts

## Impact

- **Code**: New `checkpoint` module, modifications to `orchestrator.rs` and `prompt.rs`
- **CLI**: New `--max-retries` flag for loop command
- **Git**: Creates stashes with naming pattern `ralph:{change}:{story}` during execution
- **Agent Protocol**: New optional `<promise>FAILED: reason</promise>` signal
