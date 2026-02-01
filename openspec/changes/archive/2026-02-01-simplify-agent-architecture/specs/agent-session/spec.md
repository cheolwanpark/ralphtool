## REMOVED Requirements

### Requirement: Session initialization
**Reason**: Session concept removed. Agent works directly with files.
**Migration**: Orchestrator provides change location in prompt.

### Requirement: Session state persistence
**Reason**: No session state needed. OpenSpec files are the source of truth.
**Migration**: Agent reads/writes OpenSpec files directly.

### Requirement: Session flush
**Reason**: No accumulated state to flush.
**Migration**: Agent writes directly to files during execution.

### Requirement: Next story tracking
**Reason**: Agent manages its own story progression by reading tasks.md.
**Migration**: Prompt instructs agent to complete stories in order.
