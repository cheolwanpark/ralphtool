## Why

The current ConversionPreview screen displays Tasks, UserStories, and Scenarios as three flat sections. However, Scenarios belong to UserStories (1:N relationship), and this relationship is not visualized. Users cannot see which scenarios verify which user story. Additionally, mixing implementation concerns (Tasks) with specification concerns (UserStories/Scenarios) in a single scrollable view makes it harder to focus on one aspect at a time.

## What Changes

- **Add tabbed interface to ConversionPreview screen**: Split the preview into two tabs:
  - **Tasks tab**: Shows Story > Task hierarchy (unchanged from current)
  - **User Stories tab**: Shows UserStory > Scenario hierarchy (new organization)
- **Nest Scenarios under their parent UserStories**: Instead of flat scenario list, display scenarios expanded under each UserStory they belong to
- **Add tab navigation**: Tab/Shift+Tab keys to switch between tabs
- **Update help bar**: Show tab switching hint alongside scroll/navigation keys

## Capabilities

### New Capabilities
- `tabbed-preview`: Tab-based navigation within the preview screen, with tab state management and tab-aware rendering

### Modified Capabilities
- `conversion-preview-screen`: Restructure to use tabs, nest scenarios under user stories, update layout and keybindings

## Impact

- **Code changes**:
  - `src/app.rs`: Add tab state (active tab enum, tab switching methods)
  - `src/ui/preview.rs`: Refactor to render based on active tab, implement scenario nesting under UserStories
  - `src/event.rs`: Add Tab/Shift+Tab handling for preview screen
- **Data model**: No changes needed - `scenario_to_story` mapping already exists in `OpenSpecAdapter`
- **Dependencies**: No new dependencies
- **Breaking changes**: None - this is a visual reorganization
