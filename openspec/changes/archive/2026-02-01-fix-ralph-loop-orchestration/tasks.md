## 1. App State Changes

- [x] 1.1 Add `loop_event_rx: Option<std::sync::mpsc::Receiver<LoopEvent>>` field to `App` struct
- [x] 1.2 Add `loop_stop_flag: Option<Arc<AtomicBool>>` field to `App` struct
- [x] 1.3 Add `loop_thread: Option<std::thread::JoinHandle<()>>` field to `App` struct

## 2. Loop Initialization

- [x] 2.1 Update `start_loop()` to load stories from `OpenSpecAdapter` and populate `LoopState` with real counts
- [x] 2.2 Create `std::sync::mpsc::channel` for loop events in `start_loop()`
- [x] 2.3 Spawn orchestrator in background thread with tokio runtime in `start_loop()`
- [x] 2.4 Store stop flag handle and event receiver in `App` struct

## 3. Event Loop Integration

- [x] 3.1 Add `process_loop_events()` method to `App` that calls `try_recv()` and updates `LoopState`
- [x] 3.2 Call `process_loop_events()` in main TUI loop when on `LoopExecution` screen
- [x] 3.3 Handle `LoopEvent::Complete` to transition to result screen or back to selection

## 4. Stop Signal Handling

- [x] 4.1 Update `handle_loop_events()` to set stop flag when 'q' is pressed instead of immediately navigating away
- [x] 4.2 Transition screen only after orchestrator thread completes or stop is acknowledged

## 5. Cleanup

- [x] 5.1 Add `cleanup_loop()` method to join thread and clear loop state on exit
- [x] 5.2 Run `cargo clippy` and fix warnings
- [x] 5.3 Run `cargo test` to verify no regressions
