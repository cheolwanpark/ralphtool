## 1. Update Scenario Type

- [x] 1.1 Rename `story_id` field to `requirement_id` in `src/spec/types.rs`
- [x] 1.2 Add `capability: String` field to Scenario struct in `src/spec/types.rs`
- [x] 1.3 Update Scenario test cases to use new field names

## 2. Update OpenSpec Adapter Parsing

- [x] 2.1 Track capability name (spec folder name) when parsing specs in `parse_specs_dir`
- [x] 2.2 Pass capability name to `parse_spec_md` function
- [x] 2.3 Set `capability` field on each parsed Scenario
- [x] 2.4 Rename `current_story_id` variable to `current_requirement_id` in parse logic
- [x] 2.5 Update all Scenario instantiation sites to use new field names

## 3. Update App State and Methods

- [x] 3.1 Remove `scenarios_for_story` method from App in `src/app.rs`
- [x] 3.2 Add `scenarios_for_capability` method that filters by capability field
- [x] 3.3 Add `unique_capabilities` method that returns sorted list of capability names

## 4. Rewrite Scenarios Tab Rendering

- [x] 4.1 Remove story-based grouping logic from `render_scenarios_tab` in `src/ui/preview.rs`
- [x] 4.2 Remove "Unmatched Scenarios" section entirely
- [x] 4.3 Implement capability-based grouping: iterate capabilities, show scenarios under each
- [x] 4.4 Add requirement sub-headers within each capability section
- [x] 4.5 Maintain existing scenario detail rendering (GIVEN/WHEN/THEN formatting)

## 5. Update Context Provider

- [x] 5.1 Update `Context` struct usage in `src/spec/openspec.rs` to use new Scenario fields
- [x] 5.2 Update any scenario filtering in context assembly to use `requirement_id`

## 6. Verification

- [x] 6.1 Run `cargo check` to ensure compilation
- [x] 6.2 Run `cargo clippy` to check for warnings
- [x] 6.3 Run `cargo test` to verify existing tests pass
- [x] 6.4 Manual test: run TUI and verify Scenarios tab shows capability groupings
