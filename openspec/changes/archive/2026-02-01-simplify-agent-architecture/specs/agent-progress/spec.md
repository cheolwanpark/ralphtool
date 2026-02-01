## REMOVED Requirements

### Requirement: Status command
**Reason**: Agent reads files directly to understand progress.
**Migration**: Agent reads tasks.md to count completed vs total tasks.

### Requirement: Progress JSON response
**Reason**: No CLI to produce JSON output.
**Migration**: Agent parses tasks.md checkboxes directly.
