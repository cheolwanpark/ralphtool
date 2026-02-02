## 1. Event System Changes

- [x] 1.1 Add Clone derive to StreamEvent and Response in src/agent/mod.rs if not present
- [x] 1.2 Replace LoopEvent::AgentOutput with LoopEvent::StoryEvent { story_id: String, event: StreamEvent } in src/ralph_loop/mod.rs
- [x] 1.3 Add started_story_ids: Vec<String> to LoopState in src/ralph_loop/mod.rs
- [x] 1.4 Update Orchestrator to emit StoryEvent instead of AgentOutput, passing full StreamEvent with story_id

## 2. App State Changes

- [x] 2.1 Replace loop_log: Vec<String> with story_events: HashMap<String, Vec<StreamEvent>> in App struct
- [x] 2.2 Add loop_selected_story: usize field to App for story navigation
- [x] 2.3 Add LoopTab enum (Info, Agent) and loop_tab field to App
- [x] 2.4 Add loop_agent_scroll: usize and loop_info_scroll: usize fields to App
- [x] 2.5 Update process_loop_events to handle StoryEvent and populate story_events HashMap
- [x] 2.6 Add helper methods: visible_stories(), current_story(), can_navigate_left(), can_navigate_right()

## 3. Mouse Scroll Support

- [x] 3.1 Add EnableMouseCapture to init_terminal() in src/main.rs
- [x] 3.2 Add DisableMouseCapture to restore_terminal() in src/main.rs
- [x] 3.3 Update handle_events() in src/event.rs to match Event::Mouse
- [x] 3.4 Add handle_preview_mouse() for Preview screen scroll
- [x] 3.5 Add handle_loop_mouse() for Loop Execution screen scroll
- [x] 3.6 Add handle_result_mouse() for Result screen scroll

## 4. Loop Screen Keyboard Navigation

- [x] 4.1 Add Left arrow handling to navigate to previous story
- [x] 4.2 Add Right arrow handling to navigate to next story
- [x] 4.3 Add Tab key handling to switch between Info and Agent tabs
- [x] 4.4 Add Up/Down arrow handling for scroll within current tab
- [x] 4.5 Add navigation constraint to prevent selecting unstarted stories

## 5. Loop Screen UI Rewrite

- [x] 5.1 Create render_progress_bar() function showing change name and completion ratio
- [x] 5.2 Create render_story_indicator() function with sliding window logic (max 5 visible)
- [x] 5.3 Create render_info_tab() showing story title and task list with checkboxes
- [x] 5.4 Create render_agent_tab() showing messages with "Assistant:" prefix and spacing
- [x] 5.5 Add distinct "Done:" section rendering with usage stats (turns, tokens, cost) in different color
- [x] 5.6 Implement scroll snap logic: auto-scroll when at bottom, disable on manual scroll up
- [x] 5.7 Update render_loop_screen() to compose all new components with tab bar

## 6. Integration and Testing

- [x] 6.1 Update App::cleanup_loop() to clear new state fields
- [x] 6.2 Verify story auto-selection on StoryProgress event
- [x] 6.3 Test navigation between completed stories while agent works on current story
- [x] 6.4 Test mouse scroll on all three scrollable screens
