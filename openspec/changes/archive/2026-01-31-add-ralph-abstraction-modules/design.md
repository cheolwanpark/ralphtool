## Context

Ralph is an autonomous AI agent loop that works on tasks iteratively. Different spec systems (OpenSpec, SpecKit, etc.) store task/story/progress data in their own formats. We need a Rust abstraction layer that:
- Defines traits representing Ralph workflow concepts
- Allows adapters to implement these traits for specific spec systems
- Keeps the spec system as the single source of truth (no Ralph-specific files)

The codebase is a Rust ratatui TUI application. This change adds foundational modules that future adapters will implement.

## Goals / Non-Goals

**Goals:**
- Define clear trait interfaces for Ralph concepts (TaskSource, ProgressTracker, StoryProvider, VerificationSource)
- Provide domain types that traits operate on (Task, Story, Epic, Progress, Scenario, etc.)
- Keep traits minimal and focused - one concept per trait
- Enable bidirectional operations (read state, write updates back to source)

**Non-Goals:**
- Implementing specific adapters (OpenSpec, SpecKit) - that's Phase 2
- Creating Ralph-specific files (prd.json, progress.txt)
- Building the Ralph execution loop - that uses these traits but is separate
- Parsing any specific file format - adapters handle that

## Decisions

### Decision: Trait-per-concept over monolithic interface
Define separate traits (TaskSource, ProgressTracker, etc.) rather than one large `RalphBackend` trait.

**Rationale**:
- Adapters can implement only what they support
- Easier to test and mock individual concerns
- Follows Interface Segregation Principle

**Alternatives considered**:
- Single `RalphBackend` trait: Rejected - forces adapters to implement everything
- Struct with optional fields: Rejected - loses type safety

### Decision: Owned types over references in trait returns
Trait methods return owned types (e.g., `Vec<Task>`) rather than references.

**Rationale**:
- Adapters may construct data on-the-fly from their format
- Simplifies lifetime management
- Async compatibility is easier with owned types

**Alternatives considered**:
- Return `&[Task]`: Rejected - requires adapter to store data in specific way
- Return iterators: Considered for large datasets, may add later

### Decision: Error type as associated type
Each trait defines its own `Error` associated type rather than a shared error enum.

**Rationale**:
- Adapters have different failure modes
- Allows adapter-specific error details
- Can use `anyhow::Error` or custom types

### Decision: Sync traits initially, async later
Start with synchronous trait methods. Add async variants when needed.

**Rationale**:
- Simpler to implement and test
- File-based adapters don't need async
- Can add `AsyncTaskSource` etc. when network adapters are needed

## Risks / Trade-offs

**Risk**: Trait design doesn't fit all adapters well
→ Mitigation: Start with OpenSpec adapter in mind, iterate based on real usage

**Risk**: Over-abstraction making simple things complex
→ Mitigation: Keep traits minimal, add methods only when needed by actual adapters

**Trade-off**: Owned return types use more memory than references
→ Acceptable: Task lists are small, simplicity wins over micro-optimization

## Open Questions

- Should `ProgressTracker::record_learning()` be sync or async? (Depends on whether adapters need to call external services)
- Do we need a `ChangeProvider` trait for managing multiple changes, or is that adapter-specific?
