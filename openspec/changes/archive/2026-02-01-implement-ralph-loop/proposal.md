## Why

Enable autonomous AI-driven development through the Ralph loop - an orchestrator that spawns fresh AI instances iteratively to complete OpenSpec changes. Currently, the agent CLI provides the building blocks (session, context, task done) but there's no orchestrator to drive the loop. This adds the missing piece: a TUI-integrated loop that shows real-time progress.

## What Changes

- **New `src/loop/` module**: Orchestration logic that drives the Ralph loop
  - Iterates through stories, spawning AI for each
  - Manages iteration lifecycle and completion detection
  - Provides real-time status for TUI display

- **New `src/agent/` module**: Coding agent abstraction
  - Trait for different AI backends (Claude Code, Amp, etc.)
  - Claude Code integration using `-p` flag with `--output-format json`
  - Simple spawn/output capture interface

- **Rename `src/agent/` → `src/session/`**: Current agent CLI becomes session module
  - Session lifecycle (init, next-story, flush)
  - Agent CLI commands (context, task done, learn, status)
  - No functional changes, just reorganization

- **TUI Flow Enhancement**: Four-step workflow
  1. Select change (existing)
  2. Review change content (existing)
  3. Ralph loop screen (new - shows iterations in real-time)
  4. Review result (new - changed files, verification status)

## Capabilities

### New Capabilities
- `ralph-loop`: Orchestration logic for autonomous AI development loops
- `coding-agent`: Abstraction for AI coding backends with Claude Code implementation

### Modified Capabilities
- `agent-cli`: Rename module to `session`, update imports (no behavior change)

## Impact

- **Code Structure**:
  - `src/agent/` → `src/session/` (rename)
  - New `src/loop/` module
  - New `src/agent/` module (different purpose)

- **Dependencies**:
  - Requires `claude` CLI installed and authenticated
  - Uses `-p`, `--output-format json`, `--allowedTools` flags

- **TUI**:
  - New screens for loop execution and result review
  - Real-time iteration display
