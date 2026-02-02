## Context

The loop screen displays agent messages and task lists in two tabs (Agent/Info). Currently, both tabs use complex manual line-wrapping logic to maintain indentation when text wraps. This causes visual bugs where wrapped lines don't align properly.

Current layout:
```
Agent Tab:
Assistant: My task is complete for Story 2. However, I need to ensure verification passes...
           (continuation lines should indent here but don't)

Info Tab:
  ☐ 5.1 Create render_progress_bar() function that handles
        the gauge widget and displays...
```

## Goals / Non-Goals

**Goals:**
- Fix line wrapping alignment issues in both tabs
- Simplify code by removing manual wrap logic (~80 lines)
- Improve readability with clearer visual separation

**Non-Goals:**
- Changing scroll behavior
- Modifying progress bar or story indicator
- Adding new features to loop screen

## Decisions

### 1. Separate labels from content onto different lines

**Choice**: Put "Assistant:", "Done:", and task checkboxes on their own line, content below with consistent indentation.

**Rationale**:
- Eliminates the need to calculate prefix widths and continuation indentation
- `Paragraph::wrap()` can handle all wrapping natively
- Cleaner visual hierarchy

**Alternative considered**: Fix the manual wrap logic
- Rejected: More complex, still fragile, doesn't leverage ratatui's capabilities

### 2. Use simple indentation for content

**Choice**: Indent all content lines by 2 spaces for Agent tab content, 4 spaces for task descriptions.

**Rationale**:
- Consistent, predictable layout
- No complex width calculations needed
- Works with any terminal width

### 3. New layout format

**Agent Tab:**
```
Assistant:
  My task is complete for Story 2. However, I need to ensure
  verification passes. Given that Story 1 introduced a breaking
  change...


Done:
  (response content)
  Turns: 5 | Tokens: 1234 | Cost: $0.05
```

**Info Tab:**
```
Story 5: Loop Screen UI Rewrite

  ☐ 5.1
    Create render_progress_bar() function that handles the gauge
    widget and displays completion ratio
  ☑ 5.2
    Create render_story_indicator() function
```

### 4. Increase message spacing in Agent tab

**Choice**: Use 2 blank lines between messages instead of 1.

**Rationale**:
- Better visual separation between distinct agent responses
- Easier to scan through message history

## Risks / Trade-offs

- **More vertical space used**: Separating labels onto their own lines uses more vertical space. Acceptable trade-off for readability.
- **Visual change**: Users will see different layout. Low risk as this is an internal tool.
