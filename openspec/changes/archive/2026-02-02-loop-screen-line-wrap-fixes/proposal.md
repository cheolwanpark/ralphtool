## Why

Long text in the loop screen tabs (Info and Agent) breaks layout when wrapped. The Info tab loses indentation on continuation lines, and the Agent tab shows excessive spacing from consecutive blank lines in agent messages.

## What Changes

- **Info Tab**: Maintain proper indentation when task descriptions wrap to multiple lines. Continuation lines should align with the description start position (after `☐ {task.id} `).
- **Agent Tab**: Compress consecutive blank lines in agent messages to a single blank line to prevent excessive vertical spacing.

## Capabilities

### New Capabilities

None - this is a fix to existing rendering behavior.

### Modified Capabilities

- `loop-story-tabs`: Fixing line wrapping and blank line handling in Info and Agent tabs

## Impact

- `src/ui/loop_screen.rs`: Modifications to `render_info_tab` and `render_message_lines` functions
