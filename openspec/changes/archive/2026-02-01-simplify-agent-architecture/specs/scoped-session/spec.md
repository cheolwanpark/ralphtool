## REMOVED Requirements

### Requirement: RAII session wrapper
**Reason**: Session concept removed entirely.
**Migration**: No session lifecycle management needed.

### Requirement: Session environment variables
**Reason**: Environment variables don't propagate to Bash tool subprocesses.
**Migration**: Prompt contains all information agent needs.

### Requirement: Exclusive lock on change
**Reason**: Lock mechanism removed for simplicity. User responsibility to not run concurrent agents.
**Migration**: None - accept risk of concurrent edits (handled by git).

### Requirement: Automatic cleanup on drop
**Reason**: No session files or locks to clean up.
**Migration**: None needed.
