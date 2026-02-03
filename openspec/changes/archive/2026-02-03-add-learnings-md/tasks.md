## 1. Learnings File Management

- [x] 1.1 Create `learnings.rs` module in `src/ralph_loop/` with path helper function `learnings_path(change_name: &str) -> PathBuf` returning `/tmp/ralphtool/{change_name}-learnings.md`
- [x] 1.2 Add `ensure_learnings_file(change_name: &str) -> Result<()>` function that creates the directory and file with initial template if missing, preserves existing file
- [x] 1.3 Add `read_learnings(change_name: &str) -> Result<Option<String>>` function that returns `Some(content)` if file exists and has content beyond template, `None` otherwise
- [x] 1.4 Add unit tests for learnings module: path generation, file creation, content reading

## 2. Prompt Integration

- [x] 2.1 Update `PromptBuilder` to accept optional `learnings_content: Option<String>` in constructor or method
- [x] 2.2 Add learnings section to `for_story_with_retry_context()` when `learnings_content` is `Some`: include guidance on what to record and the learnings file path
- [x] 2.3 Update `Orchestrator` to read learnings content and pass to `PromptBuilder`
- [x] 2.4 Call `ensure_learnings_file()` at the start of `Orchestrator::run()` before the story loop

## 3. Testing

- [x] 3.1 Add integration test: learnings file is created on first iteration start
- [x] 3.2 Add integration test: existing learnings content appears in prompt
- [x] 3.3 Add integration test: empty learnings file results in no learnings section in prompt
