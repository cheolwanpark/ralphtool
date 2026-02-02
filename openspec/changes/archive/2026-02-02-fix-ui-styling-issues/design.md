## Context

The TUI has two visual styling issues identified during UI review:

1. **Result Screen Changed Files**: The `render_changed_files` function applies a single color style to the entire line (e.g., "M\tsrc/app.rs"), but git diff status coloring should only apply to the status character (M/A/D), not the filename.

2. **Loop Screen Story Progress**: The `render_story_progress` function displays ellipsis ("...") before and after the visible story window when stories overflow. This is redundant since the centering behavior already indicates there are more stories.

## Goals / Non-Goals

**Goals:**
- Apply status coloring only to the status character in Changed Files tab
- Remove ellipsis indicators from story progress display
- Maintain all existing functionality (centering, scrolling, etc.)

**Non-Goals:**
- Changing the color scheme itself
- Modifying any other UI components
- Adding new features

## Decisions

### Decision 1: Use multiple Spans for Changed Files line

**Choice**: Split each changed file line into separate `Span` elements - one styled for the status character, one unstyled for the filename.

**Rationale**: ratatui's `Line` widget supports multiple `Span` elements with different styles. This is the idiomatic way to apply different styles to parts of a single line.

**Implementation**:
```rust
// Parse "M\tfilename" into status and filename
let (status, filename) = (&file[0..1], file[1..].trim_start());
Line::from(vec![
    Span::styled(status, status_style),
    Span::raw(" "),
    Span::raw(filename),
])
```

### Decision 2: Simply remove ellipsis span generation

**Choice**: Delete the ellipsis rendering code while keeping the window offset calculation (centering logic).

**Rationale**: The centering behavior already communicates that the user can scroll to see more stories. Ellipsis adds visual noise without information.

## Risks / Trade-offs

**Risk**: Users may not realize there are more stories outside the visible window.
**Mitigation**: The centering behavior and keybinding hints ("←→ Story") already communicate navigation is available. This is acceptable.
