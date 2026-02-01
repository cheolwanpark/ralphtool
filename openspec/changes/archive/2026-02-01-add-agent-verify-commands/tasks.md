## 1. Fix Scenario Retrieval

- [x] 1.1 Update `get_context()` in openspec.rs to call `list_scenarios()` instead of `scenarios_for(story_id)`

## 2. Add Verify Commands

- [x] 2.1 Add `Verify` subcommand enum to cli.rs with `Context` and `Pass` variants
- [x] 2.2 Create `src/agent/verify.rs` module with `VerifyContextResponse` struct
- [x] 2.3 Implement `run_verify_context()` using `list_stories()` and `list_scenarios()`
- [x] 2.4 Implement `run_verify_pass()` using `mark_passed()`
- [x] 2.5 Wire up verify commands in mod.rs dispatcher

## 3. Testing

- [x] 3.1 Add unit tests for verify context response structure
- [x] 3.2 Verify existing context command still works with all scenarios
