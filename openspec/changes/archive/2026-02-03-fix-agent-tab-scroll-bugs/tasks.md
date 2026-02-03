## 1. Fix scroll_up synchronization

- [x] 1.1 Modify loop_scroll_up() to sync loop_agent_scroll = max_scroll when auto_scroll is true before decrementing

## 2. Fix scroll_down no-op at bottom

- [x] 2.1 Modify loop_scroll_down() to return early when auto_scroll is true (already at bottom)

## 3. Update tests

- [x] 3.1 Add test for scroll_up from auto_scroll mode syncing position correctly
- [x] 3.2 Add test for scroll_down being no-op when auto_scroll is true
