## 1. Fix Story Count Bug

- [ ] 1.1 In orchestrator.rs, add `state.completed_stories = state.total_stories` in the `None` match arm (around line 349) when all stories are complete

## 2. Add AwaitingUserChoice Event

- [ ] 2.1 In ralph_loop/mod.rs, add `AwaitingUserChoice` variant to `LoopEvent` enum with a `tokio::sync::oneshot::Sender<CompletionOption>` field
- [ ] 2.2 In ralph_loop/mod.rs, re-export `CompletionOption` from checkpoint module so TUI can use it

## 3. Update Orchestrator to Wait for User Choice

- [ ] 3.1 In orchestrator.rs, after the story_loop ends (before `Complete` event), create a oneshot channel and send `AwaitingUserChoice` event with the sender
- [ ] 3.2 In orchestrator.rs, wait for the user's choice via the oneshot receiver (handle Err case as Keep)
- [ ] 3.3 In orchestrator.rs, call `self.checkpoint.cleanup(option)` with the received choice
- [ ] 3.4 In orchestrator.rs, only send `Complete` event after cleanup finishes

## 4. Update TUI to Forward User Choice

- [ ] 4.1 In app.rs, add a field to store the oneshot sender from `AwaitingUserChoice` event (e.g., `completion_choice_tx: Option<oneshot::Sender<CompletionOption>>`)
- [ ] 4.2 In app.rs `process_loop_events()`, handle `AwaitingUserChoice` event by storing the sender and transitioning to completion screen
- [ ] 4.3 In app.rs, add a method to send the user's choice via the stored oneshot sender

## 5. Remove Cleanup Logic from main.rs

- [ ] 5.1 In main.rs, remove the `if app.screen == Screen::LoopCompletion && app.completion_data.in_progress` block that creates a new Checkpoint and calls cleanup
- [ ] 5.2 In main.rs, update completion screen handling to call the new app method that forwards choice to Orchestrator
- [ ] 5.3 In event.rs, update Enter key handling on completion screen to trigger choice forwarding instead of setting in_progress

## 6. Update CompletionData and Screen Flow

- [ ] 6.1 In completion_screen.rs or app.rs, update flow so `in_progress` triggers waiting for `Complete` event instead of running cleanup
- [ ] 6.2 Ensure completion screen shows progress indicator while waiting for Orchestrator to finish cleanup
