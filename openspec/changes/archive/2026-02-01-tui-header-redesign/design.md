## Context

The TUI currently has four screens (Selection, Preview, LoopExecution, Result) with inconsistent header layouts. Each screen renders its own title block, and the Preview screen lacks navigation to LoopExecution despite having methods (`start_loop`) ready in `App`. Help text is buried in the footer where it competes with status information.

Current header structure varies:
- Selection: Title centered in bordered block
- Preview: Two-line header with change name and counts
- LoopExecution: Title with status in bordered block
- Result: Title with description in bordered block

## Goals / Non-Goals

**Goals:**
- Unified header component usable across all screens
- ASCII art branding that fits in 2-3 terminal lines
- Clear, scannable keybinding reference in header
- Complete screen flow: Preview → LoopExecution via `R` key
- Remove redundant footer help text

**Non-Goals:**
- Dynamic/configurable ASCII art themes
- Customizable keybindings
- Changes to the actual Loop execution logic
- Modifications to result verification flow

## Decisions

### 1. Shared Header Component

**Decision**: Create a `render_header()` function in `ui/mod.rs` that all screens call.

**Rationale**: Avoids code duplication and ensures visual consistency. Each screen passes its specific keybindings and context.

**Alternatives considered**:
- Trait-based approach: More abstract but overkill for static rendering
- Macro-based: Would obscure the simple layout logic

### 2. Header Layout Structure

**Decision**: Three-column layout within a single bordered block:

```
┌─────────────────────────────────────────────────────────────────┐
│  ╦═╗┌─┐┬  ┌─┐┬ ┬   │ Selection Screen    │ ↑↓ Navigate         │
│  ╠╦╝├─┤│  ├─┘├─┤   │                     │ Enter Select        │
│  ╩╚═┴ ┴┴─┘┴  ┴ ┴   │                     │ q Quit              │
└─────────────────────────────────────────────────────────────────┘
```

- Left: ASCII art (fixed width ~18 chars)
- Center: Screen title + context info
- Right: Keybindings (right-aligned)

**Rationale**: Maximizes information density while maintaining readability. ASCII art provides instant brand recognition.

**Alternatives considered**:
- Stacked layout (art above keys): Wastes vertical space
- Art only on Selection screen: Loses brand presence

### 3. ASCII Art Choice

**Decision**: Use compact box-drawing style that fits in 3 lines:

```
╦═╗┌─┐┬  ┌─┐┬ ┬
╠╦╝├─┤│  ├─┤├─┤
╩╚═┴ ┴┴─┘┴  ┴ ┴
```

**Rationale**: Readable at small sizes, uses standard Unicode box-drawing characters supported by most terminals.

### 4. Keybinding Display Format

**Decision**: Single-column list with key highlighted:

```
↑↓ Navigate
Enter Select
R Run Loop
q Quit
```

**Rationale**: Scannable, consistent with terminal conventions. Arrow symbols are more recognizable than "Up/Down".

### 5. Screen Transition Keybinding

**Decision**: Use `R` (uppercase/lowercase both work) to start the loop from Preview.

**Rationale**:
- `R` for "Run" is intuitive
- `Enter` is already used for selection
- Consistent with terminal app conventions (vim, htop use single letters)

## Risks / Trade-offs

**[Risk] Terminal width constraints** → Header layout degrades on terminals < 60 chars wide. Mitigation: Test minimum width, potentially hide ASCII art on narrow terminals.

**[Risk] Unicode compatibility** → Box-drawing characters may not render on legacy terminals. Mitigation: Characters used are in basic Unicode block, widely supported since ~2010.

**[Trade-off] Vertical space** → New header is 5 lines (3 art + borders) vs current 3 lines. Acceptable given improved usability and that content areas still have ample space.

**[Trade-off] Footer removal** → Some users expect help at bottom. Mitigation: Header placement is actually more visible; keys are shown before users need them.
