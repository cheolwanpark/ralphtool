## Why

The TUI's Scenarios tab shows all scenarios as "Unmatched" because the scenario↔story matching logic uses incompatible ID spaces. Stories parsed from `tasks.md` have numeric IDs (`"1"`, `"2"`, `"3"`) while scenarios parsed from specs have slugified requirement names as story_ids (`"loop-orchestration"`, `"codingagent-trait"`). These never match, so every scenario falls into the "Unmatched Scenarios" section.

## What Changes

- **Change scenario grouping from story-based to capability-based**: Instead of trying to match scenarios to task stories (which have different purposes), group scenarios by their source spec file (capability). This aligns with the actual data structure.
- **Add capability name to Scenario type**: Scenarios already have a `story_id` derived from requirement names, but add explicit `capability` field to track which spec file they came from.
- **Update TUI scenarios tab**: Display scenarios grouped by capability rather than trying to match to task stories.
- **Rename misleading field**: Rename `Scenario.story_id` to `requirement_id` to reflect what it actually represents (the slugified requirement name, not a task story reference).

## Capabilities

### New Capabilities
(none)

### Modified Capabilities
- `openspec-adapter`: Add capability tracking when parsing scenarios, rename story_id to requirement_id
- `conversion-preview-screen`: Update scenarios tab to group by capability instead of story, remove unmatched logic

## Impact

- **Code changes**:
  - `src/spec/types.rs`: Rename `Scenario.story_id` → `Scenario.requirement_id`, add `Scenario.capability` field
  - `src/spec/openspec.rs`: Track capability name during spec parsing
  - `src/ui/preview.rs`: Group scenarios by capability, remove unmatched section
  - `src/app.rs`: Update `scenarios_for_story` → `scenarios_for_capability` or remove if unused
- **Breaking changes**: Field rename in `Scenario` struct (internal only)
- **Dependencies**: None
