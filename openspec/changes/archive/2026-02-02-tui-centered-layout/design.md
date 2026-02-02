## Context

The RalphTool TUI currently renders content at full terminal width with a compact 5-line horizontal header. This approach has usability issues:
- On wide terminals (>120 cols), lines become hard to read
- The header mixes logo, title, and keybindings horizontally, feeling cramped
- No visual breathing room between logo and content

The TUI uses ratatui 0.29 with crossterm backend. All rendering happens through a main `render()` function that dispatches to screen-specific renderers.

## Goals / Non-Goals

**Goals:**
- Constrain content to max 100 columns, centered horizontally and vertically
- Create a unified header section occupying ~20% of height with logo, title, description, and keybindings
- Support graceful degradation on small terminals (hide logo when height < 24)
- Apply consistent styling across all 4 screens

**Non-Goals:**
- Changing the screen navigation flow or state machine
- Adding new screens or functionality
- Supporting configurable max-width (hardcoded is fine for now)
- Horizontal scrolling for narrow terminals

## Decisions

### Decision 1: Centering Implementation
**Choice**: Use a `centered_rect()` helper function that calculates padding based on terminal size and max constraints.

```rust
fn centered_rect(area: Rect, max_width: u16, max_height: u16) -> Rect {
    let width = area.width.min(max_width);
    let height = area.height.min(max_height);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
```

**Rationale**: Simple, no external dependencies, easy to test. The function is pure and can be called at the start of each render.

**Alternatives Considered**:
- ratatui's `Layout::centered()` - doesn't exist in 0.29
- Nested layouts with `Constraint::Percentage` - more complex, harder to understand

### Decision 2: Header Section Structure
**Choice**: Vertical layout within header section:
```
┌─────────────────────────────────────────┐
│  █▀█ ▄▀█ █   █▀█ █ █                   │  Line 1-2: Logo
│  █▀▄ █▀█ █▄▄ █▀▀ █▀█                   │
│                                         │  Line 3: Blank
│  ◆ Change Selection                     │  Line 4: Title
│  Select a change to preview and run     │  Line 5: Description
│                                         │  Line 6: Blank
│  ↑↓ Navigate   Enter Select   q Quit   │  Line 7: Keybindings
└─────────────────────────────────────────┘
```

**Rationale**: Clear visual hierarchy, logo gets prominence, keybindings at bottom are easy to find.

### Decision 3: Logo Storage
**Choice**: Store logo as a const array of &str lines in `ui/mod.rs`.

```rust
const LOGO: [&str; 2] = [
    "█▀█ ▄▀█ █   █▀█ █ █",
    "█▀▄ █▀█ █▄▄ █▀▀ █▀█",
];
```

**Rationale**: Simple, no file I/O, easy to modify. The logo is small (2 lines, ~21 chars).

### Decision 4: Small Terminal Handling
**Choice**: Check terminal height at render time. If < 24 lines, use compact single-line header.

**Compact mode format**: `◆ Selection │ ↑↓ Navigate  Enter Select  q Quit`

**Rationale**: 24 lines is the classic terminal minimum. Below this, prioritize content over branding.

### Decision 5: Constants Location
**Choice**: Define layout constants in `ui/mod.rs`:

```rust
const MAX_WIDTH: u16 = 100;
const MIN_HEIGHT_FOR_LOGO: u16 = 24;
const HEADER_LINES: u16 = 8;  // When logo is shown
const HEADER_LINES_COMPACT: u16 = 1;  // When logo is hidden
```

**Rationale**: Centralized, easy to tune, no magic numbers in render code.

## Risks / Trade-offs

**[Risk] Vertical space consumption** → The 20% header takes space from content. Mitigated by compact mode for small terminals and the fact that most users have 30+ line terminals.

**[Risk] Logo character rendering** → Block characters may not render correctly on all terminals/fonts. Mitigation: These are standard Unicode block characters widely supported. Fallback: could add ASCII-only alternative later if needed.

**[Trade-off] Fixed max-width** → Some users might want wider/narrower. Accepted for simplicity; can add config later if requested.

**[Trade-off] Y-centering with variable content** → When content is taller than max_height, no Y-centering happens. This is expected behavior - we center the container, not the content within it.
