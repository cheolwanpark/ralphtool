## Context

The loop screen has two tabs (Info and Agent) that display text content. Both use Ratatui's `Paragraph` widget with `Wrap { trim: false }` for text wrapping. However:

1. **Info Tab**: When task descriptions are long and wrap to multiple lines, the continuation lines start at column 0, breaking the visual hierarchy of the task list.

2. **Agent Tab**: Agent messages can contain consecutive blank lines (common in markdown output), which creates excessive vertical spacing between content sections.

Current rendering in `src/ui/loop_screen.rs`:
- `render_info_tab`: Uses `Paragraph::new(lines).wrap(Wrap { trim: false })`
- `render_message_lines`: Adds each line from the message directly without filtering

## Goals / Non-Goals

**Goals:**
- Info Tab: Continuation lines align with the description start position (after checkbox and task ID)
- Agent Tab: Consecutive blank lines are collapsed to a single blank line

**Non-Goals:**
- Changing the overall tab layout or styling
- Adding new rendering features
- Changing how messages are received or stored

## Decisions

### Decision 1: Manual text wrapping for Info Tab

Instead of relying on Ratatui's `Wrap`, manually calculate where to break long task descriptions and add appropriate indentation to continuation lines.

**Approach**:
- Calculate the indent width: `2 (leading spaces) + 1 (checkbox) + 1 (space) + task.id.len() + 1 (space)`
- Calculate available width for description: `area.width - indent_width - 2 (borders)`
- Manually split the description into lines that fit
- First line: full prefix (`  ☐ {task.id} `) + description start
- Continuation lines: spaces equal to indent width + remaining description

**Alternative considered**: Using a custom wrapping widget - rejected as over-engineering for this use case.

### Decision 2: Filter consecutive blank lines in Agent Tab

In `render_message_lines`, track whether the previous line was blank and skip adding another blank line if so.

**Approach**:
- Add a `prev_was_blank` boolean flag
- Before adding a blank line, check if previous was also blank
- Skip if consecutive, add if not

**Alternative considered**: Post-processing all lines after building - rejected as less efficient and harder to maintain.

## Risks / Trade-offs

- **Manual wrapping complexity**: The manual text wrapping logic needs to handle Unicode correctly. Mitigation: Use character count for simplicity; Ratatui handles display width internally.
- **Loss of some formatting**: Compressing blank lines may alter intended markdown formatting. Mitigation: Only consecutive blanks are affected; single blank lines between sections remain.
