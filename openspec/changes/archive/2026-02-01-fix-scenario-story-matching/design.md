## Context

The TUI has a Scenarios tab that attempts to group scenarios under stories. Currently:
- Stories come from `tasks.md` with numeric IDs (`"1"`, `"2"`)
- Scenarios come from `specs/*.md` with slugified requirement names as `story_id` (`"loop-orchestration"`)

The matching logic in `render_scenarios_tab` filters scenarios where `scenario.story_id == story.id`, which never matches because the ID namespaces are completely different. All scenarios end up in "Unmatched Scenarios".

The root cause is a conceptual mismatch: task stories (numbered implementation phases) and spec requirements (named behavioral contracts) are different concepts that were incorrectly assumed to be linked.

## Goals / Non-Goals

**Goals:**
- Scenarios display grouped meaningfully in the TUI
- Remove broken matching logic that produces misleading "Unmatched" results
- Clarify the data model to reflect what `story_id` actually represents

**Non-Goals:**
- Creating a link between task stories and spec requirements (different concepts)
- Changing how scenarios are parsed from specs
- Modifying the task parsing logic

## Decisions

### Decision 1: Group scenarios by capability, not by story

Scenarios naturally belong to capabilities (spec files). Each `specs/<capability>/spec.md` file contains requirements with their scenarios. Grouping by capability:
- Reflects the actual file structure
- Provides meaningful organization (all auth scenarios together, all loop scenarios together)
- Requires no artificial ID mapping

**Alternative considered**: Create explicit story↔requirement mapping in tasks.md. Rejected because task stories and requirements serve different purposes - forcing a link adds complexity without value.

### Decision 2: Add `capability` field to Scenario, rename `story_id` to `requirement_id`

The current `story_id` field stores the slugified requirement name (e.g., `"loop-orchestration"`), not a story ID. Renaming to `requirement_id` clarifies intent. Adding `capability` (the spec folder name, e.g., `"ralph-loop"`) enables grouping.

**Data model change:**
```rust
pub struct Scenario {
    pub name: String,
    pub capability: String,      // NEW: e.g., "ralph-loop"
    pub requirement_id: String,  // RENAMED from story_id: e.g., "loop-orchestration"
    pub given: Vec<String>,
    pub when: String,
    pub then: Vec<String>,
}
```

### Decision 3: Replace TUI matching logic entirely

Remove:
- `scenarios_for_story()` method in App
- Story-based grouping in `render_scenarios_tab`
- "Unmatched Scenarios" section

Replace with:
- Capability-based grouping: iterate unique capabilities, show scenarios under each
- Requirements shown as sub-headers under capabilities

## Risks / Trade-offs

**[Breaking internal API]** → The `Scenario` struct field rename affects all code using `story_id`. Mitigation: grep and update all usages in the same change.

**[Semantic shift]** → Users might expect scenarios to relate to task stories. Mitigation: The grouping by capability is more intuitive than "everything is unmatched".
