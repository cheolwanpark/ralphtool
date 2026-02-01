## REMOVED Requirements

### Requirement: Agent CLI subcommand
**Reason**: Environment variables don't propagate to Claude Code Bash tool. Agent works directly with files instead.
**Migration**: Agent reads OpenSpec files directly and edits tasks.md to mark progress.

### Requirement: JSON output format
**Reason**: No CLI commands to output JSON.
**Migration**: None needed - agent uses file operations.

### Requirement: CLI help text
**Reason**: CLI removed entirely.
**Migration**: None needed.
