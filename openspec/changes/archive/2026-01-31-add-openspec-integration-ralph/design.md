## Context

The ralphtool TUI application has a Ralph abstraction layer with traits (`TaskSource`, `StoryProvider`, `VerificationSource`, `ProgressTracker`) and domain types (`Epic`, `Story`, `Task`, `UserStory`, `Scenario`, etc.) defined in `src/ralph/`. Currently there are no concrete adapters implementing these traits.

The application needs to integrate with OpenSpec to read completed changes and present them for implementation via the Ralph loop. OpenSpec provides CLI commands (`openspec list --json`, `openspec status --change <name> --json`) and stores change artifacts in `openspec/changes/<name>/` directories with `tasks.md` and `specs/<capability>/spec.md` files.

Current TUI has basic event loop with quit functionality (`tui-core` spec) and welcome screen (`ui-rendering` spec). It uses ratatui + crossterm.

## Goals / Non-Goals

**Goals:**
- Implement OpenSpec adapter that provides Ralph domain types from OpenSpec change data
- Enable users to select completed changes for implementation
- Display conversion results (task hierarchy, stories, scenarios) in a preview screen
- Support screen navigation (selection → preview → back via Esc)

**Non-Goals:**
- Full Ralph loop execution (out of scope - preview only)
- Writing back to OpenSpec (ProgressTracker write operations return Ok but are no-ops for now)
- Real-time change watching / auto-refresh
- Multiple adapter support (only OpenSpec adapter in this change)

## Decisions

### Decision 1: CLI invocation vs direct file parsing

**Choice:** Hybrid approach - use CLI for listing/status, parse files for content.

**Rationale:**
- `openspec list --json` and `openspec status --change <name> --json` provide structured data about changes and completion status
- CLI doesn't expose parsed task/spec content, so we must parse `tasks.md` and `specs/*.md` directly
- Using CLI for metadata ensures compatibility with OpenSpec versioning

**Alternatives considered:**
- Pure CLI: Not feasible - no commands expose task/scenario content
- Pure file parsing: Would duplicate logic for determining completion status

### Decision 2: Task hierarchy mapping

**Choice:** Map OpenSpec structure to Ralph hierarchy as:
- `## N. Group Name` in tasks.md → `Epic`
- Implicit story (one per epic for now) → `Story`
- `- [ ] N.M Task description` → `Task`

**Rationale:**
- OpenSpec tasks.md uses numbered groups (epics) with checkbox tasks
- Ralph expects Epic > Story > Task hierarchy
- Creating one Story per Epic keeps mapping simple; can refine later

**Alternatives considered:**
- Flatten all tasks: Loses grouping context
- Parse story markers from tasks.md: No standard format exists yet

### Decision 3: Scenario extraction

**Choice:** Parse `#### Scenario:` blocks from specs, extracting GIVEN/WHEN/THEN lines.

**Rationale:**
- OpenSpec specs use `#### Scenario: <name>` format with bullet points for steps
- Maps directly to `Scenario { name, given, when, then }` domain type
- GIVEN/WHEN/THEN keywords are consistently used in specs

### Decision 4: Screen state management

**Choice:** Add `Screen` enum to `App` state with variants `ChangeSelection` and `ConversionPreview`.

**Rationale:**
- Simple enum-based state machine for two screens
- `App` holds current screen, selected change name, and loaded Ralph data
- Esc key transitions `ConversionPreview` → `ChangeSelection`

**Alternatives considered:**
- Separate screen structs with trait: Over-engineered for two screens
- Modal overlay: Doesn't match selection → preview flow

### Decision 5: Adapter module structure

**Choice:** Add `src/ralph/openspec.rs` module with:
- `OpenSpecAdapter` struct holding change name and cached data
- Implements all four Ralph traits
- Internal parser functions for tasks.md and spec.md files

**Rationale:**
- Keeps adapter code isolated from trait definitions
- Caching parsed data avoids re-parsing on each trait method call
- Single struct implements all traits (common in adapter pattern)

### Decision 6: Error handling

**Choice:** Use `anyhow::Error` for adapter error type, wrap CLI/parse failures.

**Rationale:**
- Project already uses `anyhow` (in Cargo.toml)
- CLI failures and parse errors need context
- Trait associated types allow `type Error = anyhow::Error`

## Risks / Trade-offs

**Risk: CLI availability** → Adapter assumes `openspec` is in PATH. Mitigation: Check on startup, show helpful error if missing.

**Risk: Parse brittleness** → tasks.md/spec.md format changes could break parsing. Mitigation: Use defensive parsing, log warnings for unrecognized formats.

**Risk: Large change data** → Loading all tasks/specs into memory for preview. Mitigation: Acceptable for typical change sizes; add lazy loading later if needed.

**Trade-off: One story per epic** → Simplifies mapping but loses potential sub-grouping. Acceptable for initial implementation.

**Trade-off: No-op writes** → ProgressTracker methods don't persist. Acceptable since this is preview-only; real persistence comes with Ralph loop integration.

## Module Layout

```
src/
├── ralph/
│   ├── mod.rs          # re-exports
│   ├── traits.rs       # existing trait definitions
│   ├── types.rs        # existing domain types
│   └── openspec.rs     # NEW: OpenSpec adapter
├── ui/
│   ├── mod.rs          # NEW: ui module root
│   ├── selection.rs    # NEW: change selection screen
│   └── preview.rs      # NEW: conversion preview screen
├── app.rs              # MODIFIED: add Screen enum, selected change
├── event.rs            # MODIFIED: handle Esc for navigation
├── ui.rs               # MODIFIED: dispatch to screen renderers
└── main.rs             # minimal changes
```

## Data Flow

```
1. App starts → ChangeSelection screen
2. Load changes: openspec list --json → filter isComplete
3. User selects change → Enter pressed
4. Load adapter: OpenSpecAdapter::new(change_name)
   - Calls openspec status --change <name> --json
   - Parses tasks.md → Epics/Stories/Tasks
   - Parses specs/*.md → UserStories, Scenarios
5. Transition to ConversionPreview screen
6. Render: task hierarchy, stories, scenarios
7. Esc pressed → back to ChangeSelection
```
