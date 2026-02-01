## Context

The ralphtool TUI currently has a ConversionPreview screen that shows three flat sections: Tasks, User Stories, and Scenarios. The `OpenSpecAdapter` already maintains a `scenario_to_story: HashMap<String, String>` mapping that links scenarios to their parent user stories, but this relationship isn't visualized in the UI.

The TUI uses ratatui for rendering and crossterm for input handling. The current architecture has:
- `App` struct in `app.rs` with screen state and data
- `Screen` enum with `ChangeSelection` and `ConversionPreview` variants
- Separate render functions per screen in `ui/` module
- Event handlers in `event.rs` dispatched by screen

## Goals / Non-Goals

**Goals:**
- Add a tabbed interface to the ConversionPreview screen
- Display Scenarios nested under their parent UserStories
- Maintain separate scroll positions per tab
- Keep the implementation minimal and focused

**Non-Goals:**
- Changing the data model or Ralph abstraction layer
- Adding collapsible/expandable sections (scenarios always expanded)
- Supporting more than two tabs
- Adding mouse support for tab switching

## Decisions

### Decision 1: Tab state in App struct

Add a `PreviewTab` enum and `active_tab` field to `App`, plus per-tab scroll offsets.

```rust
pub enum PreviewTab {
    Tasks,
    UserStories,
}

pub struct App {
    // existing fields...
    pub active_tab: PreviewTab,
    pub tasks_scroll_offset: usize,
    pub user_stories_scroll_offset: usize,
}
```

**Rationale**: Simple, explicit state. Two scroll offsets instead of one because users expect to return to their previous position when switching tabs.

**Alternative considered**: Single `scroll_offsets: HashMap<PreviewTab, usize>` - rejected as over-engineering for two tabs.

### Decision 2: Tab bar as separate layout constraint

Add a new layout row for the tab bar between header and content:

```
Constraint::Length(4)  → Header
Constraint::Length(1)  → Tab bar (new)
Constraint::Min(10)    → Content
Constraint::Length(3)  → Help
```

**Rationale**: Clean separation, tab bar doesn't scroll with content.

### Decision 3: Tab rendering format

Active tab shown with brackets: `[Tasks] | User Stories`

**Rationale**: User preference from exploration session. Minimal, clear indication.

### Decision 4: Expose scenario lookup in OpenSpecAdapter

Add a public method to get scenarios for a user story:

```rust
impl OpenSpecAdapter {
    pub fn scenarios_for_story(&self, story_id: &str) -> Vec<&Scenario> {
        // Uses existing scenario_to_story mapping
    }
}
```

**Rationale**: The mapping exists but isn't exposed. Need it for nested rendering.

**Alternative considered**: Passing the HashMap to the UI - rejected to maintain encapsulation.

### Decision 5: Tab/Shift+Tab for navigation

Use Tab key to switch to next tab, Shift+Tab for previous.

**Rationale**: Standard keyboard convention, user preference confirmed.

## Risks / Trade-offs

**[Risk] Tab key conflicts with terminal** → Using crossterm's key detection which handles Tab correctly in raw mode.

**[Risk] Layout breaks on small terminals** → Tab bar is only 1 row, minimal impact. Content area already handles small sizes.

**[Trade-off] Two scroll offsets increase App state** → Acceptable for better UX. Only 2 extra `usize` fields.
