## 1. App State Changes

- [x] 1.1 Add `loop_agent_auto_scroll: bool` field to App struct with default true
- [x] 1.2 Initialize auto_scroll to true in App::new() and cleanup_loop()

## 2. Scroll Reset on Story Change

- [x] 2.1 Reset loop_info_scroll, loop_agent_scroll, and auto_scroll in navigate_to_previous_story()
- [x] 2.2 Reset loop_info_scroll, loop_agent_scroll, and auto_scroll in navigate_to_next_story()
- [x] 2.3 Reset loop_info_scroll, loop_agent_scroll, and auto_scroll when auto-selecting new story in process_loop_events()

## 3. Preview Screen Native Scroll

- [x] 3.1 Refactor render_preview() to use Paragraph::scroll() instead of skip().collect()

## 4. Loop Screen Native Scroll

- [x] 4.1 Refactor render_info_tab() to use Paragraph::scroll() instead of skip().collect()
- [x] 4.2 Refactor render_agent_tab() to use Paragraph::scroll() with line_count() for auto-scroll
- [x] 4.3 Remove calculate_scroll_offset() function (no longer needed)
- [x] 4.4 Remove is_at_bottom() function (no longer needed)

## 5. Result Screen Conversion

- [x] 5.1 Convert render_tasks_tab() from List to Paragraph with scroll()
- [x] 5.2 Convert render_changed_files() from List to Paragraph with scroll()

## 6. Auto-Scroll Event Handling

- [x] 6.1 Update loop_scroll_up() to set auto_scroll = false
- [x] 6.2 Update loop_scroll_down() to set auto_scroll = true when reaching bottom

## 7. Cleanup

- [x] 7.1 Remove List import from result_screen.rs if no longer used
- [x] 7.2 Update tests for new auto_scroll behavior
