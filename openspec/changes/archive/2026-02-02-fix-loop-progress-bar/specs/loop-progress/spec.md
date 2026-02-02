## ADDED Requirements

### Requirement: Progress bar reflects completed story count

The loop execution screen progress bar SHALL display the accurate count of completed stories transmitted from the orchestrator via `LoopEvent::StoryProgress`.

#### Scenario: Progress bar updates as stories complete
- **WHEN** the orchestrator emits a `StoryProgress` event with `completed: 2` and `total: 5`
- **THEN** the progress bar displays "2/5" and fills to 40% (2/5 ratio)

#### Scenario: Initial state shows zero progress
- **WHEN** the loop starts and no stories have completed
- **THEN** the progress bar displays "0/N" where N is total stories
