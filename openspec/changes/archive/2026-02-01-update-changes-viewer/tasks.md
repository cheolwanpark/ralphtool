## 1. Add Tab State to App

- [x] 1.1 Add `PreviewTab` enum with `Tasks` and `UserStories` variants to `app.rs`
- [x] 1.2 Add `active_tab: PreviewTab` field to `App` struct with default `Tasks`
- [x] 1.3 Add `tasks_scroll_offset` and `user_stories_scroll_offset` fields to `App`
- [x] 1.4 Add `switch_to_next_tab()` and `switch_to_previous_tab()` methods to `App`
- [x] 1.5 Update scroll methods to use the active tab's scroll offset

## 2. Add Tab Switching Event Handling

- [x] 2.1 Add Tab key handling in `handle_preview_events` to call `switch_to_next_tab()`
- [x] 2.2 Add Shift+Tab handling in `handle_preview_events` to call `switch_to_previous_tab()`

## 3. Expose Scenario Lookup in OpenSpecAdapter

- [x] 3.1 Add `scenarios_for_story(&self, story_id: &str) -> Vec<&Scenario>` method to `OpenSpecAdapter`

## 4. Update Preview Screen Layout

- [x] 4.1 Add tab bar layout constraint (Length 1) between header and content in `render_preview`
- [x] 4.2 Implement `render_tab_bar` function that shows `[Tasks] | User Stories` format
- [x] 4.3 Update help text to include "Tab/Shift+Tab Switch"

## 5. Implement Tab Content Rendering

- [x] 5.1 Refactor `render_preview` to dispatch to `render_tasks_tab` or `render_user_stories_tab` based on active tab
- [x] 5.2 Extract current Tasks section rendering into `render_tasks_tab` function
- [x] 5.3 Implement `render_user_stories_tab` that shows UserStories with nested Scenarios
- [x] 5.4 Add indentation and visual hierarchy for Scenarios under each UserStory

## 6. Testing and Verification

- [x] 6.1 Verify tab switching works with Tab/Shift+Tab keys
- [x] 6.2 Verify scroll positions are preserved when switching tabs
- [x] 6.3 Verify Scenarios display under correct UserStories
- [x] 6.4 Verify empty state handling (UserStory with no Scenarios)
