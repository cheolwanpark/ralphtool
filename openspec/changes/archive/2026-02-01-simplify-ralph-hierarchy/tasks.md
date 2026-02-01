## 1. Remove Epic from Types

- [x] 1.1 Remove `Epic` struct from `src/ralph/types.rs`
- [x] 1.2 Remove Epic-related tests from `src/ralph/types.rs`
- [x] 1.3 Update module exports in `src/ralph/mod.rs` to remove Epic

## 2. Update TaskSource Trait

- [x] 2.1 Change `TaskSource::list_tasks()` return type from `Vec<Epic>` to `Vec<Story>` in `src/ralph/traits.rs`
- [x] 2.2 Update trait doc comments to reflect Story > Task hierarchy

## 3. Update OpenSpec Adapter

- [x] 3.1 Rename `parse_epic_header` to `parse_story_header` in `src/ralph/openspec.rs`
- [x] 3.2 Update `parse_tasks_md` to return `Vec<Story>` instead of `Vec<Epic>`
- [x] 3.3 Remove auto-Story creation logic (epics no longer contain stories)
- [x] 3.4 Update `OpenSpecAdapter` struct to store `Vec<Story>` instead of `Vec<Epic>`
- [x] 3.5 Update `TaskSource` impl to return `Vec<Story>` from `list_tasks()`
- [x] 3.6 Update `next_task()` to iterate over stories directly
- [x] 3.7 Update `mark_complete()` to iterate over stories directly
- [x] 3.8 Update adapter tests to use Story instead of Epic

## 4. Update App State

- [x] 4.1 Change `App.epics: Vec<Epic>` to `App.stories: Vec<Story>` in `src/app.rs`
- [x] 4.2 Update `load_selected_change()` to populate `app.stories`
- [x] 4.3 Update imports to remove Epic

## 5. Update TUI Preview

- [x] 5.1 Update task count calculation in `src/ui/preview.rs` to use stories
- [x] 5.2 Change rendering loop from `app.epics` to `app.stories`
- [x] 5.3 Update "Epic N:" label to "Story N:" in task section
- [x] 5.4 Remove inner story iteration (now just story.tasks)

## 6. Update Specs

- [x] 6.1 Update `openspec/specs/ralph-concepts/spec.md` to remove Epic requirements
- [x] 6.2 Update `openspec/specs/openspec-adapter/spec.md` to reference Story instead of Epic
- [x] 6.3 Update `openspec/specs/conversion-preview-screen/spec.md` to reference Story instead of Epic

## 7. Update Documentation

- [x] 7.1 Update `docs/openspec-ralph-concepts.md` to describe Story > Task hierarchy
- [x] 7.2 Update `docs/openspec-ralph-implementation.md` to remove Epic references

## 8. Verification

- [x] 8.1 Run `cargo check` to verify no compilation errors
- [x] 8.2 Run `cargo test` to verify all tests pass
- [x] 8.3 Run `cargo run` and verify TUI displays correctly
