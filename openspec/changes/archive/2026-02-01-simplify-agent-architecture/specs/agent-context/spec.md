## REMOVED Requirements

### Requirement: Context command
**Reason**: Agent reads files directly instead of requesting context via CLI.
**Migration**: Agent reads proposal.md, design.md, tasks.md, and specs/ directly.

### Requirement: Context JSON response
**Reason**: No CLI to produce JSON output.
**Migration**: Agent parses files using its native file reading capabilities.
