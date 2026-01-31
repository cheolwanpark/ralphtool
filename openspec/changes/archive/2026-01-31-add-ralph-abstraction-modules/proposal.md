## Why

The Ralph workflow operates on concepts like tasks, stories, progress tracking, and learnings. Different spec systems (OpenSpec, SpecKit, etc.) store these concepts in their own formats. We need an abstraction layer that:
1. Presents a unified Ralph-concept interface regardless of the underlying spec system
2. Allows bidirectional sync - reads from and writes back to the source system's native format
3. Keeps the external spec system as the single source of truth (no Ralph-specific files like prd.json, progress.txt)

This enables the Ralph execution loop to work with any spec system through adapter implementations.

## What Changes

- Add `ralph` module defining **traits** for Ralph concepts (not file formats)
- Traits are backend-agnostic interfaces that adapters implement
- Adapters read/write to their native format (e.g., OpenSpec adapter reads tasks.md, writes checkbox updates back)
- No Ralph-specific files are created - the spec system IS the persistence layer

Ralph concept traits:
- **TaskSource**: Provides hierarchical tasks (Epic > Story > Task) with completion state
- **ProgressTracker**: Records learnings and patterns (writes to source system's format)
- **StoryProvider**: Provides user stories with acceptance criteria and priority
- **VerificationSource**: Provides scenarios for verification (Given/When/Then)

## Capabilities

### New Capabilities
- `ralph-concepts`: Core Rust traits defining Ralph workflow concepts (TaskSource, ProgressTracker, StoryProvider, VerificationSource) - backend-agnostic interfaces for spec system adapters

### Modified Capabilities
<!-- None - this is foundational infrastructure -->

## Impact

- **New code**: `src/ralph/` module with trait definitions and domain types
- **Dependencies**: Minimal - just core Rust types; adapters will add their own deps
- **Architecture**: Trait-based design enables future adapters (OpenSpec, SpecKit, etc.) without modifying core
- **NOT created**: prd.json, progress.txt, or any Ralph-specific files
