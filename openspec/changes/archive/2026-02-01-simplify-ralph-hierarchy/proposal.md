## Why

The current Ralph abstraction layer uses a 3-level hierarchy (Epic > Story > Task) but in practice, the Story layer is vestigial—every Epic has exactly one auto-created Story. This adds unnecessary complexity and prevents efficient iteration: with 29 tasks across 10 "Epics", iterating per-Task means 29 agent launches instead of 10. The original Ralph method uses a simpler Story > Task model where Story is the iteration unit, which is more practical.

## What Changes

- **BREAKING**: Remove `Epic` type from Ralph abstraction layer
- **BREAKING**: `TaskSource::list_tasks()` returns `Vec<Story>` instead of `Vec<Epic>`
- Update OpenSpec adapter to parse `## Heading` as Story (not Epic)
- Update TUI preview screen to render Story > Task hierarchy
- Update App state to store `Vec<Story>` instead of `Vec<Epic>`
- Update tests to reflect new 2-level hierarchy
- Update specs and docs to reflect simplified model

## Capabilities

### New Capabilities

(none - this is a simplification of existing capability)

### Modified Capabilities

- `ralph-concepts`: Requirement changes from Epic > Story > Task to Story > Task hierarchy. TaskSource trait signature changes.
- `openspec-adapter`: Parsing logic changes to map `## Heading` directly to Story instead of Epic with auto-created Story.
- `conversion-preview-screen`: Display changes from Epic > Task to Story > Task rendering.

## Impact

- **Types**: Remove `Epic` struct, keep `Story` with tasks
- **Traits**: `TaskSource::list_tasks()` returns `Vec<Story>`
- **OpenSpec Adapter**: Parse `## N. Title` as Story, checkboxes as Tasks
- **App State**: `app.epics` becomes `app.stories`
- **TUI Preview**: Update rendering loop from `epic.stories.tasks` to `story.tasks`
- **Tests**: Update all Epic-related tests to Story-based tests
- **Specs**: Update ralph-concepts spec to remove Epic requirements
- **Docs**: Update openspec-ralph-concepts.md and openspec-ralph-implementation.md
