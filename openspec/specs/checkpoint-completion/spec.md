## Purpose

TUI completion screen for handling the end of a Ralph Loop. Presents cleanup/keep options for managing the checkpoint branch and completed work.

## Requirements

### Requirement: Completion screen display
The TUI SHALL display a completion screen with options when the Ralph loop finishes.

#### Scenario: Show completion screen on success
- **WHEN** all stories complete successfully
- **THEN** the TUI displays a completion screen
- **AND** shows summary of completed work (stories completed, branch name)
- **AND** presents two options: "cleanup" and "keep"

#### Scenario: Show completion screen on max retries
- **WHEN** max retries exceeded for a story
- **THEN** the TUI displays a completion screen with error context
- **AND** shows which story failed and how many succeeded
- **AND** presents two options: "cleanup" and "keep"

#### Scenario: Show completion screen on user stop
- **WHEN** user stops the loop via 'q' key
- **THEN** the TUI displays a completion screen
- **AND** shows progress summary (completed stories / total)
- **AND** presents two options: "cleanup" and "keep"

### Requirement: Cleanup option behavior
The TUI SHALL execute cleanup when user selects the cleanup option.

#### Scenario: User selects cleanup
- **WHEN** user selects "cleanup" on completion screen
- **THEN** the TUI triggers checkpoint cleanup operation
- **AND** displays progress: "Returning to {original_branch}..."
- **AND** on success, shows "Changes ready on {original_branch} (uncommitted)"
- **AND** transitions to result screen

#### Scenario: Cleanup keyboard shortcut
- **WHEN** completion screen is displayed
- **THEN** pressing 'c' key selects the cleanup option

### Requirement: Keep option behavior
The TUI SHALL preserve the checkpoint branch when user selects the keep option.

#### Scenario: User selects keep
- **WHEN** user selects "keep" on completion screen
- **THEN** the TUI skips cleanup operations
- **AND** displays "Staying on ralph/{change} branch"
- **AND** transitions to result screen

#### Scenario: Keep keyboard shortcut
- **WHEN** completion screen is displayed
- **THEN** pressing 'k' key selects the keep option

### Requirement: Option descriptions
The completion screen SHALL display clear descriptions for each option.

#### Scenario: Cleanup option description
- **WHEN** completion screen shows cleanup option
- **THEN** it displays description: "Return to {original_branch} with changes uncommitted"

#### Scenario: Keep option description
- **WHEN** completion screen shows keep option
- **THEN** it displays description: "Stay on ralph/{change} branch with checkpoint commits"
