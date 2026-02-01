## Context

The Ralph abstraction layer was designed with a 3-level hierarchy (Epic > Story > Task), but in practice the middle Story layer is vestigial. The OpenSpec adapter creates exactly one Story per Epic, making the hierarchy effectively 2-level. This unnecessary complexity increases iteration count (29 task-level iterations instead of 10 story-level iterations) and adds conceptual overhead.

The original Ralph method used a simple Story > Task model where:
- **Story** = the iteration unit (one agent session handles one story)
- **Task** = sub-steps within a story (tracked for progress, but all done in same session)

Current code structure:
- `src/ralph/types.rs`: Defines `Epic`, `Story`, `Task` structs
- `src/ralph/traits.rs`: `TaskSource::list_tasks()` returns `Vec<Epic>`
- `src/ralph/openspec.rs`: Parses `## N. Title` as Epic with auto-created Story
- `src/app.rs`: Stores `Vec<Epic>` in app state
- `src/ui/preview.rs`: Renders Epic > Story > Task hierarchy

## Goals / Non-Goals

**Goals:**
- Simplify to 2-level hierarchy: Story > Task
- Story becomes the iteration unit (matches original Ralph)
- Reduce iteration count from ~29 to ~10 for typical changes
- Maintain backward compatibility with existing tasks.md format

**Non-Goals:**
- Changing the tasks.md file format (still use `## N. Title` and `- [ ] N.M Desc`)
- Adding new features beyond hierarchy simplification
- Changing verification workflow (StoryProvider, VerificationSource unchanged)

## Decisions

### Decision 1: Remove Epic type entirely

Remove `Epic` struct from types.rs. The `Story` struct becomes the top-level container.

**Rationale**: Epic serves no purpose when there's always 1:1 Epic-to-Story mapping. Removing it simplifies the data model and makes the iteration boundary clear.

**Alternative considered**: Keep Epic as optional grouping for display purposes only. Rejected because it adds complexity without benefit.

### Decision 2: TaskSource returns Vec<Story>

Change `TaskSource::list_tasks()` return type from `Vec<Epic>` to `Vec<Story>`.

**Rationale**: This is a breaking change but reflects the true iteration model. The adapter now returns stories directly as the top-level work units.

### Decision 3: Parse `## Heading` as Story

In OpenSpec adapter, parse `## N. Title` markdown headings directly as Story entries (not Epic with auto-Story).

**Rationale**: The heading text describes a cohesive unit of work - this is semantically a Story in the simplified model.

### Decision 4: Rename app.epics to app.stories

Change `App.epics: Vec<Epic>` to `App.stories: Vec<Story>` in app state.

**Rationale**: Semantic clarity. The field now holds what it says it holds.

## Risks / Trade-offs

**[Breaking API change]** → The change to `TaskSource::list_tasks()` signature is breaking. Mitigated by: this is internal-only, no external consumers.

**[Test updates required]** → All Epic-related tests need updating. Mitigated by: tests are colocated, straightforward to update.

**[Docs drift]** → External docs reference Epic > Story > Task. Mitigated by: update docs as part of this change.
