## REMOVED Requirements

### Requirement: Task done command
**Reason**: Environment variables don't propagate to Bash tool. Agent can edit files directly.
**Migration**: Agent edits tasks.md directly, changing `[ ]` to `[x]`.

### Requirement: Task validation
**Reason**: No CLI command to validate against.
**Migration**: Agent reads tasks.md to see valid task IDs.

### Requirement: Remaining tasks response
**Reason**: No CLI to return remaining tasks.
**Migration**: Agent reads tasks.md to see incomplete tasks.
