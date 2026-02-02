## 1. Data Structure Updates

- [x] 1.1 Add `stories: Vec<Story>` field to `LoopResult` struct in `result_screen.rs`
- [x] 1.2 Add `ResultTab` enum (Tasks, ChangedFiles) to `app.rs`
- [x] 1.3 Add `result_tab: ResultTab` field to `App` struct
- [x] 1.4 Add `result_tasks_scroll: usize` field to `App` for Tasks tab scroll offset (reuse existing `result_scroll_offset` for Changed Files)

## 2. Data Population

- [x] 2.1 Update `build_loop_result()` to re-parse tasks.md via OpenSpecAdapter and populate `stories` field
- [x] 2.2 Calculate `stories_completed` as count of stories where `is_complete()` is true
- [x] 2.3 Calculate `stories_total` from stories vector length
- [x] 2.4 Calculate `tasks_completed` as count of tasks with `done == true`
- [x] 2.5 Add `tasks_total` field to `LoopResult` and calculate from total task count

## 3. Result Screen UI

- [x] 3.1 Update Summary section to display "Stories: X/Y completed" and "Tasks: X/Y completed" format
- [x] 3.2 Remove Verification section and its render function
- [x] 3.3 Create tabbed layout with Tasks and Changed Files tabs replacing the old two-section layout
- [x] 3.4 Implement `render_tabs()` function to display tab bar with active tab highlighted
- [x] 3.5 Implement `render_tasks_tab()` function showing stories with task checkboxes ([x]/[ ])
- [x] 3.6 Update `render_changed_files()` to work with new tabbed layout

## 4. Event Handling

- [x] 4.1 Add `switch_result_tab()` method to `App` that toggles between Tasks and ChangedFiles
- [x] 4.2 Add `result_tasks_scroll_up()` and `result_tasks_scroll_down()` methods to `App`
- [x] 4.3 Update `handle_result_events()` in `event.rs` to handle Tab key for tab switching
- [x] 4.4 Update scroll handlers to use appropriate scroll offset based on active tab

## 5. Keybindings

- [x] 5.1 Update `RESULT_KEYBINDINGS` constant to include "Tab Switch" instruction
