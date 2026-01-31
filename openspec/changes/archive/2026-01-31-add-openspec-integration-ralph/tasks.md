## 1. Project Setup

- [x] 1.1 Add serde and serde_json dependencies to Cargo.toml
- [x] 1.2 Create src/ui/ module directory structure (mod.rs, selection.rs, preview.rs)
- [x] 1.3 Create src/ralph/openspec.rs module file

## 2. OpenSpec Adapter Core

- [x] 2.1 Implement OpenSpecAdapter struct with change_name field and cached data
- [x] 2.2 Implement CLI invocation helper (run_openspec_command) using std::process::Command
- [x] 2.3 Implement list_changes() using `openspec list --json`
- [x] 2.4 Implement is_complete() using `openspec status --change <name> --json`
- [x] 2.5 Implement tasks.md parser (extract epics and tasks from markdown)
- [x] 2.6 Implement spec.md parser (extract requirements and scenarios)

## 3. Ralph Trait Implementations

- [x] 3.1 Implement TaskSource trait for OpenSpecAdapter
- [x] 3.2 Implement StoryProvider trait for OpenSpecAdapter
- [x] 3.3 Implement VerificationSource trait for OpenSpecAdapter
- [x] 3.4 Implement ProgressTracker trait for OpenSpecAdapter (no-op writes)

## 4. App State Updates

- [x] 4.1 Add Screen enum (ChangeSelection, ConversionPreview) to app.rs
- [x] 4.2 Add selected_change_name field to App struct
- [x] 4.3 Add loaded Ralph data fields (epics, stories, scenarios) to App struct
- [x] 4.4 Add list of available changes to App struct for selection screen

## 5. Change Selection Screen

- [x] 5.1 Implement selection screen state (selected_index, changes list)
- [x] 5.2 Implement render function for change list with highlight
- [x] 5.3 Implement Up/Down arrow key navigation with wrap-around
- [x] 5.4 Implement Enter key to load selected change and transition to preview
- [x] 5.5 Implement empty state rendering when no completed changes

## 6. Conversion Preview Screen

- [x] 6.1 Implement preview screen layout with sections (Tasks, Stories, Scenarios)
- [x] 6.2 Implement task hierarchy rendering (Epic > Tasks with completion indicators)
- [x] 6.3 Implement user stories rendering with acceptance criteria
- [x] 6.4 Implement scenarios rendering with Given/When/Then structure
- [x] 6.5 Implement scroll state and Up/Down/PageUp/PageDown navigation
- [x] 6.6 Implement change context header with name and summary counts

## 7. Screen Navigation

- [x] 7.1 Update event.rs to dispatch events based on current screen
- [x] 7.2 Implement Escape key handler to transition preview → selection
- [x] 7.3 Update ui.rs to dispatch rendering to current screen
- [x] 7.4 Preserve selection state when returning from preview

## 8. Integration

- [x] 8.1 Wire up App initialization to load changes list on startup
- [x] 8.2 Wire up change selection to create OpenSpecAdapter and load data
- [x] 8.3 Update main.rs to start with ChangeSelection screen
- [x] 8.4 Add error handling for missing openspec CLI
