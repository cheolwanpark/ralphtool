## Why

The Ralph abstraction layer defines traits (`TaskSource`, `StoryProvider`, etc.) but has no concrete adapter implementation. To make the TUI functional with OpenSpec-based projects, we need an OpenSpec adapter that reads completed changes and converts them to Ralph domain types. Additionally, users need a way to select which change to implement and preview the conversion results before entering the Ralph loop.

## What Changes

- Add OpenSpec adapter implementing all Ralph traits (`TaskSource`, `StoryProvider`, `VerificationSource`, `ProgressTracker`)
- Add TUI screen for selecting completed OpenSpec changes (those where `openspec status --json` shows `isComplete: true`)
- Add TUI screen displaying conversion results with full hierarchy and verification info
- Wire up adapter to provide in-memory task/story/scenario data from selected change
- Support Esc key navigation to return from preview to selection

## Capabilities

### New Capabilities
- `openspec-adapter`: OpenSpec connector that implements all Ralph traits by using OpenSpec CLI commands (`openspec list --json`, `openspec status --change <name> --json`) and parsing change files (tasks.md, specs/*.md) to convert to Ralph domain types (Epic, Story, Task, UserStory, Scenario). Includes verification layer support per docs/openspec-ralph-verification.md (static checks, scenario-to-test mapping).
- `change-selection-screen`: TUI screen where users browse and select completed OpenSpec changes (`isComplete: true`) for implementation via Ralph loop. Shows change name, completion status, and last modified time.
- `conversion-preview-screen`: TUI screen displaying the Ralph connector's conversion results including task hierarchy (Epic > Story > Task), user stories with acceptance criteria, verification scenarios (Given/When/Then), and test mappings. Serves as dummy screen for testing until Ralph loop is integrated.

### Modified Capabilities
- `tui-core`: Add screen navigation state to support multiple screens (change selection → conversion preview → back via Esc)

## Impact

- **Code**: New `src/ralph/openspec.rs` adapter module, new TUI screen modules in `src/ui/`
- **Dependencies**: `serde` + `serde_json` for parsing OpenSpec CLI JSON output, `std::process::Command` for CLI invocation
- **Architecture**: App state expands to include current screen enum, selected change name, and loaded Ralph domain data (in-memory)
