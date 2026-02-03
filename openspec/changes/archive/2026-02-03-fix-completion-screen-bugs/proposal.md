## Why

The completion screen has two bugs that cause incorrect behavior after Ralph Loop finishes:
1. The story count displays "2/3 completed" even when all 3 stories completed successfully
2. The "Cleanup" option fails silently because the Checkpoint instance loses its original_branch information

## What Changes

- Fix story completion count by updating `completed_stories` when all stories are done
- Refactor cleanup logic so Orchestrator handles the entire cleanup lifecycle
- Add bidirectional communication between TUI and Orchestrator for user choice
- Remove cleanup logic from main.rs (now handled by Orchestrator)

## Capabilities

### New Capabilities

- `orchestrator-cleanup`: Orchestrator owns the complete cleanup lifecycle, waiting for user choice and executing cleanup/keep before signaling completion

### Modified Capabilities

## Impact

- `src/ralph_loop/orchestrator.rs`: Add cleanup handling and bidirectional communication
- `src/ralph_loop/mod.rs`: Add new event types for cleanup flow
- `src/main.rs`: Remove cleanup logic, add choice forwarding to Orchestrator
- `src/app.rs`: Update completion flow to forward choice to Orchestrator
- `src/event.rs`: Handle new cleanup-related events
